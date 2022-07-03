/*
 * 模块 game。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::board::{Board, Move, Side};
use crate::decider::Decider;

/// 游戏
pub struct Game<RD: Decider, BD: Decider> {
    red_decider: RD,
    black_decider: BD,
    on_move: fn(board: &Board, mov: &Move),
}

impl<RD: Decider, BD: Decider> Game<RD, BD> {
    /// 绑定移动事件
    pub fn bind_on_move(&mut self, slot: fn(board: &Board, mov: &Move,)) {
        self.on_move = slot;
    }

    /// 开始游戏
    pub fn go(&self, board: &mut Board) -> Result<Option<Side>, ()> {
        loop {
            let red_decision = self.red_decider.make_decision(board, Side::Red).unwrap();
            if board.apply_move(&red_decision).is_err() {
                return Err(());
            }
            if board.game_finished() {
                return Ok(board.get_winner());
            }
            (self.on_move)(board, &red_decision);
            let black_decision = self.black_decider.make_decision(board, Side::Black).unwrap();
            if board.apply_move(&black_decision).is_err() {
                return Err(());
            }
            if board.game_finished() {
                return Ok(board.get_winner());
            }
            (self.on_move)(board, &black_decision);
        }
    }

    /// 构建
    pub fn new(red_decider: RD, black_decider: BD) -> Game<RD, BD> {
        Self {
            red_decider,
            black_decider,
            on_move: |_, _| {},
        }
    }
}
