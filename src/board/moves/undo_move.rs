use crate::Board;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveType::*;

impl Board {
    //refacto : prend une Struct Undo en param ?
    pub fn undo_move(&mut self, m: Move, active_player: Color) {
        let capture_coord = match m.move_type {
            EnPassant => {
                let row = if active_player == White {
                    m.dest.row - 1
                } else {
                    m.dest.row + 1
                };
                Coord {
                    row,
                    col: m.dest.col,
                }
            }
            _ => m.dest,
        };

        match m.move_type {
            EnPassant => {
                self.grid[m.dest.row as usize][m.dest.col as usize] = Cell::Free;
                self.grid[capture_coord.row as usize][capture_coord.col as usize] = m.capture;
                self.grid[m.origin.row as usize][m.origin.col as usize] =
                    Cell::Occupied(Pawn, active_player);
            }
            Castle(side) => {
                let row = if active_player == White { 0 } else { 7 };
                let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
                let rook = Cell::Occupied(Rook, active_player);

                self.grid[row as usize][r_dest as usize] = Cell::Free;
                self.grid[row as usize][r_orig as usize] = rook;

                self.grid[row as usize][4] = Cell::Occupied(King, active_player);
                self.grid[m.dest.row as usize][m.dest.col as usize] = Cell::Free;

                if active_player == White {
                    self.white_king = Coord { row: 0, col: 4 };
                } else {
                    self.black_king = Coord { row: 7, col: 4 };
                }
            }
            Promotion(_) => {
                self.grid[m.origin.row as usize][m.origin.col as usize] =
                    Cell::Occupied(Pawn, active_player);
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;
            }
            Regular => {
                let moving_piece = self.get(&m.dest);
                self.grid[m.origin.row as usize][m.origin.col as usize] = moving_piece;
                self.grid[m.dest.row as usize][m.dest.col as usize] = m.capture;

                if let Some(King) = moving_piece.get_piece() {
                    match active_player {
                        White => self.white_king = m.origin,
                        Black => self.black_king = m.origin,
                    }
                }
            }
        }

        self.en_passant = m.en_passant;
        self.check = m.check;
        self.white_castle = m.white_castle;
        self.black_castle = m.black_castle;
        self.score = m.prev_score;

        // self.debug_check_score(&format!(
        //     "after undo_move active={:?} type={:?} from=({},{}) to=({},{}) capture={:?}",
        //     active_player,
        //     m.move_type,
        //     m.origin.row,
        //     m.origin.col,
        //     m.dest.row,
        //     m.dest.col,
        //     m.capture
        // ));
    }
}
