/*
 * 模块 game。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::board::{Board, Side};
use crate::decider::Decider;

/// 游戏
pub struct Game<RD: Decider, BD: Decider> {
    red_decider: RD,
    black_decider: BD,
}

impl<RD: Decider, BD: Decider> Game<RD, BD> {
    /// 开始游戏
    pub fn go(&self, board: &mut Board) -> Result<Option<Side>, ()> {
        loop {
            let red_decision = self.red_decider.make_decision(board, Side::Red);
            if board.apply_move(&red_decision.unwrap()).is_err() {
                return Err(());
            }
            if board.game_finished() {
                return Ok(board.get_winner());
            }
            let black_decision = self.black_decider.make_decision(board, Side::Black);
            if board.apply_move(&black_decision.unwrap()).is_err() {
                return Err(());
            }
            if board.game_finished() {
                return Ok(board.get_winner());
            }
        }
    }

    /// 构建
    pub fn new(red_decider: RD, black_decider: BD) -> Game<RD, BD> {
        Self {
            red_decider,
            black_decider,
        }
    }
}
