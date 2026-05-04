use crate::Board;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::board::moves::apply_move::get_capture;
use crate::board::moves::move_structs::CastleSide;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveType::*;

impl Board {
    pub fn undo_move(&mut self, m: Move, active_player: Color) {
        let capture_coord = get_capture(&m, &active_player);

        match m.move_type {
            EnPassant => self.undo_en_passant(&m, active_player, &capture_coord),
            Castle(side) => self.undo_castle(&m, active_player, side),
            Promotion(_) => self.undo_promotion(&m, active_player),
            Regular => self.undo_regular(&m, active_player),
        }
        self.copy_move_infos(&m);
    }

    fn undo_regular(&mut self, m: &Move, active_player: Color) {
        let moving_piece = self.get(&m.dest);
        self[m.origin] = moving_piece;
        self[m.dest] = m.capture;

        if let Some(King) = moving_piece.get_piece() {
            match active_player {
                White => self.white_king = m.origin,
                Black => self.black_king = m.origin,
            }
        }
    }

    fn undo_promotion(&mut self, m: &Move, active_player: Color) {
        self[m.origin] = Cell::Occupied(Pawn, active_player);
        self[m.dest] = m.capture;
    }

    fn undo_castle(&mut self, m: &Move, active_player: Color, side: CastleSide) {
        let row = if active_player == White { 0 } else { 7 };
        let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
        let rook = Cell::Occupied(Rook, active_player);

        self[(row as usize, r_dest as usize)] = Cell::Free;
        self[(row as usize, r_orig as usize)] = rook;

        self[(row as usize, 4)] = Cell::Occupied(King, active_player);
        self[m.dest] = Cell::Free;

        if active_player == White {
            self.white_king = Coord { row: 0, col: 4 };
        } else {
            self.black_king = Coord { row: 7, col: 4 };
        }
    }

    fn undo_en_passant(&mut self, m: &Move, active_player: Color, capture_coord: &Coord) {
        self[m.dest] = Cell::Free;
        self[*capture_coord] = m.capture;
        self[m.origin] = Cell::Occupied(Pawn, active_player);
    }

    fn copy_move_infos(&mut self, m: &Move) {
        self.en_passant = m.en_passant;
        self.check = m.check;
        self.white_castle = m.white_castle;
        self.black_castle = m.black_castle;
        self.score = m.prev_score;
        self.non_pawn_material = m.prev_non_pawn;
        self.hash = m.prev_hash;
    }
}
