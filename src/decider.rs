/*
 * 模块 decider，走子决定器接口和若干简单的走子决定器实现。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::board::{Board, Side, Move};

/// 走子决定器接口
pub trait Decider {
    /// 作出走子决定
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move>;
}
