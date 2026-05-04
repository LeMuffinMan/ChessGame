use crate::Board;
use crate::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::MoveType::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Move {
    pub origin: Coord,
    pub dest: Coord,
    pub capture: Cell,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
    pub move_type: MoveType,
    pub prev_score: i32,
    pub prev_non_pawn: i32,
    pub prev_hash: u64,
}

impl Move {
    pub fn is_promotion(&self, board: &Board) -> bool {
        match board[self.origin].get_color() {
            Some(White) => self.dest.row == 7,
            Some(Black) => self.dest.row == 0,
            _ => unreachable!("Move without origin"),
        }
    }
    pub fn to_uci(&self) -> String {
        let origin_col = (b'a' + self.origin.col) as char;
        let origin_row = (b'1' + self.origin.row) as char;

        let dest_col = (b'a' + self.dest.col) as char;
        let dest_row = (b'1' + self.dest.row) as char;

        format!("{}{}{}{}", origin_col, origin_row, dest_col, dest_row)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum MoveType {
    #[default]
    Regular,
    EnPassant,
    Castle(CastleSide),
    Promotion(Piece),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastleSide {
    Left,
    Right,
}

const MAX_MOVES: usize = 256;

pub struct MoveList {
    pub moves: [Move; MAX_MOVES],
    pub count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); MAX_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }
}

impl Board {
    pub fn build_move(&self, origin: Coord, dest: Coord, active_player: Color) -> Move {
        let m_type = self.get_move_type(origin, dest, self[origin].get_piece());
        self.create_move(origin, dest, active_player, m_type)
    }

    pub fn create_move(
        &self,
        origin: Coord,
        dest: Coord,
        active_player: Color,
        m_type: MoveType,
    ) -> Move {
        let capture_cell = match m_type {
            MoveType::EnPassant => match active_player {
                Color::White => {
                    self[Coord {
                        row: dest.row - 1,
                        col: dest.col,
                    }]
                }
                Color::Black => {
                    self[Coord {
                        row: dest.row + 1,
                        col: dest.col,
                    }]
                }
            },
            _ => self[dest],
        };

        self.new_move(origin, dest, capture_cell, m_type)
    }
    pub fn new_move(
        &self,
        origin: Coord,
        dest: Coord,
        capture_cell: Cell,
        m_type: MoveType,
    ) -> Move {
        Move {
            dest,
            origin,
            capture: capture_cell,
            en_passant: self.en_passant,
            check: self.check,
            white_castle: self.white_castle,
            black_castle: self.black_castle,
            move_type: m_type,
            prev_score: self.score,
            prev_non_pawn: self.non_pawn_material,
            prev_hash: self.hash,
        }
    }

    fn get_move_type(&self, origin: Coord, dest: Coord, piece_moving: Option<&Piece>) -> MoveType {
        match piece_moving {
            Some(Pawn) => {
                if dest.col != origin.col && self[dest] == Cell::Free {
                    EnPassant
                } else {
                    Regular
                }
            }
            Some(King) => {
                if (origin.col as i8 - dest.col as i8).abs() > 1 {
                    if (origin.col as i8 - dest.col as i8) < 0 {
                        Castle(Right)
                    } else {
                        Castle(Left)
                    }
                } else {
                    Regular
                }
            }
            _ => Regular,
        }
    }

    pub fn move_from_uci(&self, word: &str, active_player: Color) -> Option<Move> {
        let b = word.as_bytes();
        if b.len() < 4 {
            return None;
        }
        let origin_col = b[0].checked_sub(b'a').filter(|&c| c < 8)?;
        let origin_row = b[1].checked_sub(b'1').filter(|&r| r < 8).map(|r| 7 - r)?;
        let dest_col = b[2].checked_sub(b'a').filter(|&c| c < 8)?;
        let dest_row = b[3].checked_sub(b'1').filter(|&r| r < 8).map(|r| 7 - r)?;

        let origin = Coord {
            row: origin_row,
            col: origin_col,
        };
        let dest = Coord {
            row: dest_row,
            col: dest_col,
        };

        let m_type = if b.len() >= 5 {
            let piece = match b[4] {
                b'q' => Queen,
                b'r' => Rook,
                b'b' => Bishop,
                b'n' => Knight,
                _ => return None,
            };
            Promotion(piece)
        } else {
            self.get_move_type(origin, dest, self[origin].get_piece())
        };

        Some(self.create_move(origin, dest, active_player, m_type))
    }
}
