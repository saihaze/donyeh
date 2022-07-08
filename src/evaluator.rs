/*
 * 模块 evaluator，局面评估器接口和一个简单的局面评估器实现。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::{board::{Board, Piece, PieceKind, Side}};

/// 局面评估器接口
pub trait Evaluator {
    /// 估价
    fn evaluate(&self, board: &Board, side: Side) -> f32;
}

/// 简单的局面评估器实现
#[derive(Debug, Clone)]
pub struct SimpleEvaluator {}

impl SimpleEvaluator {
    /// 构造
    pub fn new() -> SimpleEvaluator {
        SimpleEvaluator {}
    }

    fn evaluate_single_piece(piece: Option<Piece>) -> i32 {
        match piece {
            Some(piece) => match piece.kind {
                PieceKind::帥 => 2500,
                PieceKind::車 => 100,
                PieceKind::馬 => 50,
                PieceKind::炮 => 50, 
                PieceKind::相 => 30,
                PieceKind::仕 => 30,
                PieceKind::中兵 => 25,
                PieceKind::濟兵 => 25,
                PieceKind::庶兵 => 20,
                PieceKind::底兵 => 15,
            }
            None => 0,
        }
    }
}

impl Evaluator for SimpleEvaluator {
    /// 估价
    fn evaluate(&self, board: &Board, side: Side) -> f32 {
        let mut sum = 0;
        let mut side_sum = 0;
        for x in 0..9 {
            for y in 0..10 {
                let piece = board.get_piece_at((x, y));
                let score = SimpleEvaluator::evaluate_single_piece(piece);
                sum += score;
                if board.crossing_occupied_by_side((x, y), side) {
                    side_sum += score;
                }
            }
        }
        (side_sum as f32) / (sum as f32)
    }
}
