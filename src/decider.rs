/*
 * 模块 decider，走子决定器接口和若干简单的走子决定器实现。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::board::{Board, Move, Side};
use rand::prelude::*;

/// 走子决定器接口
pub trait Decider {
    /// 作出走子决定
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move>;
}

/// 随机走子决定器实现
#[derive(Debug, Clone)]
pub struct RandomDecider {}

impl RandomDecider {
    /// 构建
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
