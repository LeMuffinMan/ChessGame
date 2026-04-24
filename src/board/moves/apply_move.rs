use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::MoveType::*;

use crate::Board;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::Move;

impl Board {
    pub fn apply_move(&mut self, m: &Move, active_player: Color) {
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

        self.update_board_score(&self.get(&m.origin), &m.origin, false);
        self.update_board_score(&m.capture, &capture_coord, false);

        self.en_passant = None;
        self.check = None;
        match self.get(&m.origin).get_piece() {
            Some(Pawn) => self.update_en_passant(&m.origin, &m.dest, &active_player),
            Some(King) => self.update_king_move(&active_player, m),
            Some(Rook) => self.update_rook_move(&active_player, m),
            _ => {}
        }
        self.update_capture_rook(m);

        if let EnPassant = m.move_type {
            self.grid[capture_coord.row as usize][capture_coord.col as usize] = Cell::Free;
        }

        self.grid[m.dest.row as usize][m.dest.col as usize] = std::mem::replace(
            &mut self.grid[m.origin.row as usize][m.origin.col as usize],
            Cell::Free,
        );

        if let Promotion(promoted) = m.move_type {
            self.grid[m.dest.row as usize][m.dest.col as usize] =
                Cell::Occupied(promoted, active_player);
        }

        if let Castle(side) = m.move_type {
            let row = if active_player == White { 0 } else { 7 };
            let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
            let rook = Cell::Occupied(Rook, active_player);
            self.update_board_score(&rook, &Coord { row, col: r_orig }, false);
            self.grid[row as usize][r_orig as usize] = Cell::Free;
            self.grid[row as usize][r_dest as usize] = rook;
            self.update_board_score(&rook, &Coord { row, col: r_dest }, true);
        }

        self.update_board_score(&self.get(&m.dest), &m.dest, true);

        // self.debug_check_score(&format!(
        //     "after apply_move active={:?} type={:?} from=({},{}) to=({},{}) capture={:?}",
        //     active_player,
        //     m.move_type,
        //     m.origin.row,
        //     m.origin.col,
        //     m.dest.row,
        //     m.dest.col,
        //     m.capture
        // ))
    }

    //reduire l'appel a check move ou enlever le is_king_exposed ? renommer
    pub fn check_move(
        &mut self,
        origin: &Coord,
        dest: &Coord,
        active_player: &Color,
    ) -> Option<Move> {
        if self.get(dest).is_color(active_player) {
            return None;
        }
        let m = self.build_move(*origin, *dest, *active_player);
        self.apply_move(&m, *active_player);
        let exposed = is_king_exposed(self, active_player);
        self.undo_move(m, *active_player);
        if !exposed {
            return Some(m);
        }
        None
    }
}
