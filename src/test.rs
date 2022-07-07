/*
 * 模块 test，包括一些测试。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

use crate::prelude::*;

/// 测试：在被将军时，是否能够选择正确的躲避方式。
#[test]
fn human_behavior_1() {
    let mut map = [[None; 10]; 9];
    map[4][0] = Some(Piece::new(PieceKind::帥, Side::Red));
    map[3][9] = Some(Piece::new(PieceKind::帥, Side::Black));
    map[4][6] = Some(Piece::new(PieceKind::車, Side::Black));
    let board = Board::new_custom(map);
    let decider = MaxMinDecider::new(SimpleEvaluator::new(), 100000);
    let decision = decider.make_decision(&board, Side::Red).unwrap();
    assert!(decision.pos_to == (5, 0));
}

/// 测试：在己方的帅被威胁时，是否会取吃掉对方的帅。
#[test]
fn human_behavior_2() {
    let mut map = [[None; 10]; 9];
    map[4][0] = Some(Piece::new(PieceKind::帥, Side::Red));
    map[2][9] = Some(Piece::new(PieceKind::底兵, Side::Red));
    map[3][9] = Some(Piece::new(PieceKind::帥, Side::Black));
    map[4][6] = Some(Piece::new(PieceKind::車, Side::Black));
    let board = Board::new_custom(map);
    let decider = MaxMinDecider::new(SimpleEvaluator::new(), 100000);
    let decision = decider.make_decision(&board, Side::Black).unwrap();
    assert!(decision.pos_from == (4, 6) && decision.pos_to == (4, 0));
}
