#[cfg(test)]
mod test;

/// 棋盘
#[derive(Debug, Clone)]
pub struct Board {
    map: [[Option<Piece>; 10]; 9],
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
    車,
    馬,
    炮,
    相,
    仕,
    中兵,
    濟兵,
    庶兵,
    底兵,
}

/// 阵营
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Red,
    Black,
}

impl Board {
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
        todo!()
    }

    /// 获取记录棋子信息的二维数组
    #[inline(always)]
    pub fn get_board(&self) -> &[[Option<Piece>; 10]; 9] {
        &self.map
    }

    /// 获取某位置的棋子
    pub fn get_piece_at(&self, pos: (i32, i32)) -> Option<Piece> {
        debug_assert!(pos.0 >= 0 && pos.0 < 9 && pos.1 >= 0 && pos.1 < 10);
        self.map[pos.0 as usize][pos.1 as usize]
    }

    /// 获取赢家
    pub fn get_winner(&self) -> Option<Side> {
        todo!()
    }
}

impl Side {
    /// 获取另一方
    #[inline(always)]
    pub fn other(&self) -> Side {
        match self {
            Side::Red => Side::Black,
            Side::Black => Side::Red,
        }
    }
}
