/*
 * 模块 decider，走子决定器接口和若干简单的走子决定器实现。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::{
    board::{Board, Move, Side},
    evaluator::Evaluator,
};
use rand::prelude::*;

/// 走子决定器接口
pub trait Decider {
    /// 作出走子决定
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move>;
}

/// 最大-最小算法决定器实现
#[derive(Debug, Clone)]
pub struct MaxMinDecider<E: Evaluator> {
    evaluator: E,
    max_node_count: u32,
}

/// 随机走子决定器实现
#[derive(Debug, Clone)]
pub struct RandomDecider {}

impl<E: Evaluator> MaxMinDecider<E> {
    /// 构造
    pub fn new(evaluator: E, max_node_count: u32) -> MaxMinDecider<E> {
        Self {
            evaluator,
            max_node_count,
        }
    }

    /// 最大搜索
    fn max_search(
        &self,
        board: &mut Board,
        side: Side,
        depth: u32,
        current_node_count: &mut u32,
        mut alpha: f32,
        beta: f32,
    ) -> Option<f32> {
        *current_node_count += 1;
        if *current_node_count > self.max_node_count {
            return None;
        }
        if depth == 0 || board.game_finished() {
            return Some(self.evaluator.evaluate(board, side));
        }
        let mut ret = 0.0f32;
        for step in board.query_possible_moves_of_side(side) {
            board.apply_move_unchecked(&step);
            let score = self.min_search(board, side, depth - 1, current_node_count, alpha, beta)?;
            board.undo_move().unwrap();
            if score > ret {
                ret = score;
            }
            if ret > alpha {
                alpha = ret;
            }
            if alpha > beta {
                break;
            }
        }
        Some(ret)
    }

    /// 最小搜索
    fn min_search(
        &self,
        board: &mut Board,
        side: Side,
        depth: u32,
        current_node_count: &mut u32,
        alpha: f32,
        mut beta: f32,
    ) -> Option<f32> {
        *current_node_count += 1;
        if *current_node_count > self.max_node_count {
            return None;
        }
        if depth == 0 || board.game_finished() {
            return Some(self.evaluator.evaluate(board, side));
        }
        let mut ret = 1.0f32;
        for step in board.query_possible_moves_of_side(side.other()) {
            board.apply_move_unchecked(&step);
            let score = self.max_search(board, side, depth - 1, current_node_count, alpha, beta)?;
            board.undo_move().unwrap();
            if score < ret {
                ret = score;
            }
            if ret < beta {
                beta = ret;
            }
            if alpha > beta {
                break;
            }
        }
        Some(ret)
    }
}

impl<E: Evaluator> Decider for MaxMinDecider<E> {
    /// 作出走子决定
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move> {
        let mut ret = None;
        for depth in 1..1000 {
            let mut playground = board.clone();
            let mut decision = None;
            let mut current_node_count = 0;
            let mut max_score = -1.0f32;
            let mut random_value = 0.0f32;
            let mut alpha = 0.0f32;
            for step in board.query_possible_moves_of_side(side) {
                playground.apply_move_unchecked(&step);
                if playground.game_finished() {
                    return Some(step);
                }
                let score = self.min_search(
                    &mut playground,
                    side,
                    depth - 1,
                    &mut current_node_count,
                    alpha,
                    1.0f32,
                );
                playground.undo_move().unwrap();
                match score {
                    Some(score) => {
                        if score > max_score {
                            max_score = score;
                            decision = Some(step);
                            random_value = random();
                            if max_score > alpha {
                                alpha = max_score;
                            }
                            // alpha 不可能大于 beta
                            // 写出不规范 evaluator 的就应该命丧当场（狗头）
                        } else if score == max_score {
                            let new_random_value = random();
                            if new_random_value > random_value {
                                decision = Some(step);
                                random_value = new_random_value;
                            }
                        }
                    }
                    None => {
                        return ret;
                    }
                }
            }
            ret = decision;
        }
        ret
    }
}

impl RandomDecider {
    /// 构造
    pub fn new() -> RandomDecider {
        RandomDecider {}
    }
}

impl Decider for RandomDecider {
    /// 作出走子决定
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move> {
        let mut steps: Vec<Move> = board.query_possible_moves_of_side(side).collect();
        if steps.is_empty() {
            None
        } else {
            let mut rng = thread_rng();
            steps.shuffle(&mut rng);
            Some(steps[0].clone())
        }
    }
}
