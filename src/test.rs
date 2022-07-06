/*
 * 模块 test，包括一些测试。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::prelude::*;

#[test]
fn 走爲上() {
    let mut map = [[None; 10]; 9];
    map[4][0] = Some(Piece::new(PieceKind::帥, Side::Red));
    map[3][9] = Some(Piece::new(PieceKind::帥, Side::Black));
    map[4][6] = Some(Piece::new(PieceKind::車, Side::Black));
    let board = Board::new_custom(map);
    let decider = MaxMinDecider::new(SimpleEvaluator::new(), 1000000);
    let decision = decider.make_decision(&board, Side::Red).unwrap();
    assert!(decision.pos_to == (5, 0));
}
