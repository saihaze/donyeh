/*
 * 模块 evaluator，局面评估器接口和一个简单的局面评估器实现。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::board::{Board, Side};

/// 局面评估器接口
pub trait Evaluator {
    /// 估价
    fn evaluate(&self, board: &Board, side: Side) -> f32;
}
