/*
 * 模块 prelude，引入一些符号。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

pub use crate::board::{Board, Move, Piece, PieceKind, Side};
pub use crate::decider::{Decider, MaxMinDecider, RandomDecider};
pub use crate::evaluator::{Evaluator, SimpleEvaluator};
