/*
 * 模块 board，实现棋盘这个数据结构及一些杂项。
 * 本文件属于 libdonyeh，使用需遵守 LGPL-3.0 协议。
 */

/// 棋盘
#[derive(Debug, Clone)]
pub struct Board {
    finished: bool,
    map: [[Option<Piece>; 10]; 9],
    unmove_records: Vec<UnmoveRecord>,
    winner: Option<Side>,
}

/// 一步移动
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub pos_from: (i32, i32),
    pub pos_to: (i32, i32),
    pub turn_into: Option<Piece>,
}

/// 棋子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub side: Side,
    pub kind: PieceKind,
}

/// 棋子种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    帥 = 1,
    車 = 2,
    馬 = 3,
    炮 = 4,
    相 = 5,
    仕 = 6,
    中兵 = 7,
    濟兵 = 8,
    庶兵 = 9,
    底兵 = 10,
}

/// 阵营
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Red,
    Black,
}

/// 悔棋记录
#[derive(Debug, Clone, PartialEq, Eq)]
struct UnmoveRecord {
    pos_0: (i32, i32),
    piece_0: Option<Piece>,
    pos_1: (i32, i32),
    piece_1: Option<Piece>,
}

impl Board {
    /// 判断走子是否合法并走子
    pub fn apply_move(&mut self, mov: &Move) -> Result<(), ()> {
        if self.check_move(&mov) {
            self.apply_move_unchecked(&mov);
            Ok(())
        } else {
            Err(())
        }
    }

    /// \[不安全\] 走子但不作检查
    pub fn apply_move_unchecked(&mut self, mov: &Move) {
        let from = mov.pos_from;
        let to = mov.pos_to;
        // 检查游戏是否结束
        match self.map[to.0 as usize][to.1 as usize] {
            Some(piece_killed) => {
                if piece_killed.kind == PieceKind::帥 {
                    // 更新游戏状态
                    self.finished = true;
                    self.winner = Some(piece_killed.side.other());
                }
            }
            None => {}
        }
        // 记录悔棋信息
        let unmove_record = UnmoveRecord {
            pos_0: mov.pos_from,
            piece_0: self.get_piece_at(mov.pos_from),
            pos_1: mov.pos_to,
            piece_1: self.get_piece_at(mov.pos_to),
        };
        self.unmove_records.push(unmove_record);
        // 更新棋盘数据
        self.map[from.0 as usize][from.1 as usize] = None;
        self.map[to.0 as usize][to.1 as usize] = mov.turn_into;
    }

    /// 检查走子是否合法
    pub fn check_move(&self, mov: &Move) -> bool {
        let possible_moves = self.query_possible_moves_from(mov.pos_from);
        for step in possible_moves {
            if step == *mov {
                return true;
            }
        }
        false
    }

    /// 查询某位置是否被占据
    pub fn crossing_occupied(&self, pos: (i32, i32)) -> bool {
        self.get_piece_at(pos).is_some()
    }

    /// 查询某位置是否被某方占据
    pub fn crossing_occupied_by_side(&self, pos: (i32, i32), side: Side) -> bool {
        match self.get_piece_at(pos) {
            Some(piece) => piece.side == side,
            None => false,
        }
    }

    /// 查询游戏是否结束
    pub fn game_finished(&self) -> bool {
        self.finished
    }

    /// 查询某一方的帅 / 将是否被威胁
    pub fn general_threatened(&self, side: Side) -> bool {
        for step in self.query_possible_moves_of_side(side.other()) {
            if self.get_piece_at(step.pos_to) == Some(Piece::new(PieceKind::帥, side)) {
                return true;
            }
        }
        false
    }

    /// 获取记录棋子信息的二维数组
    pub fn get_board(&self) -> &[[Option<Piece>; 10]; 9] {
        &self.map
    }

    /// 获取目前步数
    pub fn get_move_count(&self) -> u32 {
        self.unmove_records.len() as u32
    }

    /// 获取某位置的棋子
    pub fn get_piece_at(&self, pos: (i32, i32)) -> Option<Piece> {
        debug_assert!(pos.0 >= 0 && pos.0 < 9 && pos.1 >= 0 && pos.1 < 10);
        self.map[pos.0 as usize][pos.1 as usize]
    }

    /// 获取赢家
    pub fn get_winner(&self) -> Option<Side> {
        self.winner
    }

    /// 检查最后四步是否形成循环
    pub fn looped(&self) -> bool {
        let records = &self.unmove_records;
        if records.len() >= 4 {
            let len = records.len();
            records[len - 4].piece_1.is_none()
                && records[len - 3].piece_1.is_none()
                && records[len - 2].piece_1.is_none()
                && records[len - 1].piece_1.is_none()
                && records[len - 4].pos_0 == records[len - 2].pos_1
                && records[len - 4].pos_1 == records[len - 2].pos_0
                && records[len - 3].pos_0 == records[len - 1].pos_1
                && records[len - 3].pos_1 == records[len - 1].pos_0
        } else {
            false
        }
    }

    /// 按照默认开局构建棋盘
    pub fn new() -> Board {
        let mut map = [[None; 10]; 9];
        for (x, kind) in [
            (0, PieceKind::車),
            (1, PieceKind::馬),
            (2, PieceKind::相),
            (3, PieceKind::仕),
            (4, PieceKind::帥),
            (5, PieceKind::仕),
            (6, PieceKind::相),
            (7, PieceKind::馬),
            (8, PieceKind::車),
        ] {
            map[x][0] = Some(Piece::new(kind, Side::Red));
            map[x][9] = Some(Piece::new(kind, Side::Black));
        }
        for x in [0, 2, 6, 8] {
            map[x][3] = Some(Piece::new(PieceKind::庶兵, Side::Red));
            map[x][6] = Some(Piece::new(PieceKind::庶兵, Side::Black));
        }
        map[4][3] = Some(Piece::new(PieceKind::中兵, Side::Red));
        map[4][6] = Some(Piece::new(PieceKind::中兵, Side::Black));
        map[1][2] = Some(Piece::new(PieceKind::炮, Side::Red));
        map[7][2] = Some(Piece::new(PieceKind::炮, Side::Red));
        map[1][7] = Some(Piece::new(PieceKind::炮, Side::Black));
        map[7][7] = Some(Piece::new(PieceKind::炮, Side::Black));
        Board {
            finished: false,
            map,
            unmove_records: Vec::new(),
            winner: None,
        }
    }

    /// 构造自定义棋盘
    pub fn new_custom(map: [[Option<Piece>; 10]; 9]) -> Board {
        Board {
            finished: false,
            map,
            unmove_records: Vec::new(),
            winner: None,
        }
    }

    /// 判断位置是否在棋盘内
    pub fn position_within_board(pos: (i32, i32)) -> bool {
        Board::position_within_range(pos, (0, 0), (8, 9))
    }

    /// 判断位置是否在某范围内
    pub fn position_within_range(pos: (i32, i32), bound1: (i32, i32), bound2: (i32, i32)) -> bool {
        (pos.0 - bound1.0) * (pos.0 - bound2.0) <= 0 && (pos.1 - bound1.1) * (pos.1 - bound2.1) <= 0
    }

    /// 查询范围内棋子数
    pub fn query_piece_count_between(&self, pos1: (i32, i32), pos2: (i32, i32)) -> u32 {
        // 确定左右边界
        let (left, right) = if pos1.0 < pos2.0 {
            (pos1.0, pos2.0)
        } else {
            (pos2.0, pos1.0)
        };
        // 确定上下边界
        let (down, up) = if pos1.1 < pos2.1 {
            (pos1.1, pos2.1)
        } else {
            (pos2.1, pos1.1)
        };
        // 检查是否在棋盘范围内
        debug_assert!(left >= 0 && right < 9);
        debug_assert!(down >= 0 && up < 10);
        let mut ret = 0u32;
        for x in left..right + 1 {
            for y in down..up + 1 {
                if self.get_piece_at((x, y)).is_some() {
                    ret += 1;
                }
            }
        }
        ret
    }

    /// 查询某位置的所有走法
    pub fn query_possible_moves_from(&self, from: (i32, i32)) -> impl Iterator<Item = Move> {
        let mut ret = Vec::<Move>::new();
        match self.get_piece_at(from) {
            Some(piece) => {
                let kind = piece.kind;
                let side = piece.side;
                match kind {
                    PieceKind::帥 => {
                        // 九宫格边界
                        let (left_down, right_up) = match side {
                            Side::Red => ((3, 0), (5, 2)),
                            Side::Black => ((3, 7), (5, 9)),
                        };
                        // 前后左右
                        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let to = (from.0 + offset.0, from.1 + offset.1);
                            if Board::position_within_range(to, left_down, right_up)
                                && !self.crossing_occupied_by_side(to, side)
                            {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                        // 跳过去吃对方的帅
                        for y in [0, 1, 2, 7, 8, 9] {
                            let to = (from.0, y);
                            if self.get_piece_at(to) == Some(Piece::new(kind, side.other())) {
                                if self.query_piece_count_between(from, to) == 2 {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            }
                        }
                    }
                    PieceKind::車 => {
                        let positions_of_same_line = (0..9).map(|x| (x, from.1));
                        let positions_of_same_col = (0..10).map(|y| (from.0, y));
                        for to in positions_of_same_line.chain(positions_of_same_col) {
                            if self.crossing_occupied_by_side(to, side) {
                                continue;
                            }
                            if self.crossing_occupied_by_side(to, side.other()) {
                                if self.query_piece_count_between(from, to) == 2 {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            } else {
                                if self.query_piece_count_between(from, to) == 1 {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            }
                        }
                    }
                    PieceKind::馬 => {
                        for (offset, checkpoint_offset) in [
                            ((2, 1), (1, 0)),
                            ((2, -1), (1, 0)),
                            ((-2, 1), (-1, 0)),
                            ((-2, -1), (-1, 0)),
                            ((1, 2), (0, 1)),
                            ((1, -2), (0, -1)),
                            ((-1, 2), (0, 1)),
                            ((-1, -2), (0, -1)),
                        ] {
                            let to = (from.0 + offset.0, from.1 + offset.1);
                            let checkpoint =
                                (from.0 + checkpoint_offset.0, from.1 + checkpoint_offset.1);
                            if Board::position_within_board(to)
                                && !self.crossing_occupied_by_side(to, side)
                                && !self.crossing_occupied(checkpoint)
                            {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                    }
                    PieceKind::炮 => {
                        let positions_of_same_line = (0..9).map(|x| (x, from.1));
                        let positions_of_same_col = (0..10).map(|y| (from.0, y));
                        for to in positions_of_same_line.chain(positions_of_same_col) {
                            if self.crossing_occupied_by_side(to, side) {
                                continue;
                            }
                            if self.crossing_occupied_by_side(to, side.other()) {
                                if self.query_piece_count_between(from, to) == 3 {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            } else {
                                if self.query_piece_count_between(from, to) == 1 {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            }
                        }
                    }
                    PieceKind::相 => {
                        // 本方边界
                        let (left_down, right_up) = match side {
                            Side::Red => ((0, 0), (8, 4)),
                            Side::Black => ((0, 5), (8, 9)),
                        };
                        // 田字
                        for (offset, checkpoint_offset) in [
                            ((2, 2), (1, 1)),
                            ((2, -2), (1, -1)),
                            ((-2, 2), (-1, 1)),
                            ((-2, -2), (-1, -1)),
                        ] {
                            let to = (from.0 + offset.0, from.1 + offset.1);
                            let checkpoint =
                                (from.0 + checkpoint_offset.0, from.1 + checkpoint_offset.1);
                            if Board::position_within_range(to, left_down, right_up)
                                && !self.crossing_occupied_by_side(to, side)
                                && !self.crossing_occupied(checkpoint)
                            {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                    }
                    PieceKind::仕 => {
                        // 九宫格边界
                        let (left_down, right_up) = match side {
                            Side::Red => ((3, 0), (5, 2)),
                            Side::Black => ((3, 7), (5, 9)),
                        };
                        // 对角
                        for offset in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                            let to = (from.0 + offset.0, from.1 + offset.1);
                            if Board::position_within_range(to, left_down, right_up)
                                && !self.crossing_occupied_by_side(to, side)
                            {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                    }
                    PieceKind::中兵 | PieceKind::庶兵 => {
                        let to = match side {
                            Side::Red => (from.0, from.1 + 1),
                            Side::Black => (from.0, from.1 - 1),
                        };
                        let opposite_river_side = match side {
                            Side::Red => 5,
                            Side::Black => 4,
                        };
                        if Board::position_within_board(to)
                            && !self.crossing_occupied_by_side(to, side)
                        {
                            if to.1 == opposite_river_side {
                                ret.push(Move::new(
                                    from,
                                    to,
                                    Some(Piece::new(PieceKind::濟兵, side)),
                                ));
                            } else {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                    }
                    PieceKind::濟兵 => {
                        let offsets = match side {
                            Side::Red => [(1, 0), (-1, 0), (0, 1)],
                            Side::Black => [(1, 0), (-1, 0), (0, -1)],
                        };
                        let opposite_bottom_line = match side {
                            Side::Red => 9,
                            Side::Black => 0,
                        };
                        for offset in offsets {
                            let to = (from.0 + offset.0, from.1 + offset.1);
                            if Board::position_within_board(to)
                                && !self.crossing_occupied_by_side(to, side)
                            {
                                if to.1 == opposite_bottom_line {
                                    ret.push(Move::new(
                                        from,
                                        to,
                                        Some(Piece::new(PieceKind::底兵, side)),
                                    ));
                                } else {
                                    ret.push(Move::new(from, to, Some(piece)));
                                }
                            }
                        }
                    }
                    PieceKind::底兵 => {
                        for to in [(from.0 + 1, from.1), (from.0 - 1, from.1)] {
                            if Board::position_within_board(to)
                                && !self.crossing_occupied_by_side(to, side)
                            {
                                ret.push(Move::new(from, to, Some(piece)));
                            }
                        }
                    }
                }
            }
            None => {}
        }
        if self.looped() {
            ret.into_iter().filter(|x| {
                let record = &self.unmove_records[self.unmove_records.len() - 4];
                x.pos_from != record.pos_0 && x.pos_to != record.pos_1
            }).collect::<Vec<Move>>().into_iter()
        } else {
            ret.into_iter()
        }
    }

    /// 查询某方的所有走法
    pub fn query_possible_moves_of_side(&self, side: Side) -> impl Iterator<Item = Move> {
        let mut ret = Vec::new();
        for x in 0..9 {
            for y in 0..10 {
                let from = (x, y);
                if self.crossing_occupied_by_side(from, side) {
                    ret.push(self.query_possible_moves_from(from));
                }
            }
        }
        ret.into_iter().flatten()
    }

    /// 撤销上一步移动
    pub fn undo_move(&mut self) -> Result<(), ()> {
        if self.unmove_records.is_empty() {
            Err(())
        } else {
            // 获取记录
            let record = self.unmove_records.pop().unwrap();
            // 恢复棋盘
            self.map[record.pos_0.0 as usize][record.pos_0.1 as usize] = record.piece_0;
            self.map[record.pos_1.0 as usize][record.pos_1.1 as usize] = record.piece_1;
            // 恢复游戏状态
            self.finished = false;
            self.winner = None;
            Ok(())
        }
    }
}

impl Move {
    /// 构造
    pub fn new(pos_from: (i32, i32), pos_to: (i32, i32), turn_into: Option<Piece>) -> Move {
        Move {
            pos_from,
            pos_to,
            turn_into,
        }
    }
}

impl Piece {
    /// 构造
    pub fn new(kind: PieceKind, side: Side) -> Piece {
        Piece { kind, side }
    }
}

impl Side {
    /// 获取另一方
    pub fn other(&self) -> Side {
        match self {
            Side::Red => Side::Black,
            Side::Black => Side::Red,
        }
    }
}
