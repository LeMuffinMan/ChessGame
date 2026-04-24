use crate::Board;
use crate::Color;
use crate::Color::*;
use crate::board::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Cell::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::Move;
use crate::engine::evaluator::get_piece_value_at;

impl Board {
    pub fn update_capture_rook(&mut self, m: &Move) {
        if let Cell::Occupied(Rook, color) = m.capture {
            let rights = match color {
                White => &mut self.white_castle,
                Black => &mut self.black_castle,
            };
            match (m.dest.row, m.dest.col) {
                (0, 0) => rights.long = false,
                (0, 7) => rights.short = false,
                (7, 0) => rights.long = false,
                (7, 7) => rights.short = false,
                _ => {}
            }
        }
    }

    //mode : true = we add a piece to the board / false = we remove a piece from the board
    pub fn update_board_score(&mut self, cell: &Cell, target: &Coord, mode: bool) {
        if let Occupied(piece, color) = cell {
            let value = get_piece_value_at(piece, color, target);
            if mode {
                self.score += value;
            } else {
                self.score -= value;
            }
        }
    }

    pub fn update_king_move(&mut self, active_player: &Color, m: &Move) {
        self.update_king_castle(&m.origin, &m.dest, &active_player);
        match active_player {
            White => {
                self.white_king = m.dest;
                self.white_castle = CastleRights {
                    long: false,
                    short: false,
                };
            }
            Black => {
                self.black_king = m.dest;
                self.black_castle = CastleRights {
                    long: false,
                    short: false,
                };
            }
        }
    }

    pub fn update_rook_move(&mut self, active_player: &Color, m: &Move) {
        match (active_player, m.origin.row, m.origin.col) {
            (White, 0, 0) => self.white_castle.long = false,
            (White, 0, 7) => self.white_castle.short = false,
            (Black, 7, 0) => self.black_castle.long = false,
            (Black, 7, 7) => self.black_castle.short = false,
            _ => {}
        }
    }

    pub fn update_en_passant(&mut self, origin: &Coord, to: &Coord, active_player: &Color) {
        let dif = to.row as i8 - origin.row as i8;
        if dif.abs() == 2 {
            let mid_row = match active_player {
                White => to.row - 1,
                Black => to.row + 1,
            };
            self.en_passant = Some(Coord {
                row: mid_row as u8,
                col: origin.col,
            });
        } else {
            self.en_passant = None;
        }
    }

    pub fn update_king_castle(&mut self, origin: &Coord, to: &Coord, color: &Color) {
        let dif_col = to.col as i8 - origin.col as i8;
        let row = match color {
            White => 0,
            Black => 7,
        };
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Rook, *color);
            }
        } else if dif_col == 2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][7] = Cell::Free;
                self.grid[row][col - 1] = Cell::Occupied(Rook, *color);
            }
        }
    }
}
