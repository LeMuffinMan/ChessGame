use crate::Board;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveType::*;
use crate::engine::zobris_table::{piece_index, zobrist};

impl Board {
    pub fn apply_move(&mut self, m: &Move, active_player: Color) {
        let capture_coord = get_capture(m, &active_player);

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
            self[capture_coord] = Cell::Free;
        }

        self.grid[m.dest.row as usize][m.dest.col as usize] = std::mem::replace(
            &mut self.grid[m.origin.row as usize][m.origin.col as usize],
            Cell::Free,
        );

        if let Promotion(promoted) = m.move_type {
            self[m.dest] = Cell::Occupied(promoted, active_player);
        }

        if let Castle(side) = m.move_type {
            let row = if active_player == White { 0 } else { 7 };
            let (r_orig, r_dest) = if side == Right { (7, 5) } else { (0, 3) };
            let rook = Cell::Occupied(Rook, active_player);
            self.update_board_score(&rook, &Coord { row, col: r_orig }, false);
            self[(row as usize, r_orig as usize)] = Cell::Free;
            self[(row as usize, r_dest as usize)] = rook;
            self.update_board_score(&rook, &Coord { row, col: r_dest }, true);
        }

        self.update_board_score(&self.get(&m.dest), &m.dest, true);

        let zt = zobrist();
        let orig_sq = m.origin.row as usize * 8 + m.origin.col as usize;
        let dest_sq = m.dest.row as usize * 8 + m.dest.col as usize;

        let dest_piece = *self.get(&m.dest).get_piece().unwrap();
        let orig_piece = if let Promotion(_) = m.move_type {
            Pawn
        } else {
            dest_piece
        };

        self.hash ^= zt.pieces[piece_index(orig_piece, active_player)][orig_sq];
        self.hash ^= zt.pieces[piece_index(dest_piece, active_player)][dest_sq];

        if let Cell::Occupied(cap_piece, cap_color) = m.capture {
            let cap_sq = capture_coord.row as usize * 8 + capture_coord.col as usize;
            self.hash ^= zt.pieces[piece_index(cap_piece, cap_color)][cap_sq];
        }

        if let Castle(side) = m.move_type {
            let row = if active_player == White { 0usize } else { 7 };
            let (r_orig, r_dest): (usize, usize) = if side == Right { (7, 5) } else { (0, 3) };
            self.hash ^= zt.pieces[piece_index(Rook, active_player)][row * 8 + r_orig];
            self.hash ^= zt.pieces[piece_index(Rook, active_player)][row * 8 + r_dest];
        }

        if m.white_castle.long {
            self.hash ^= zt.castling[0];
        }
        if m.white_castle.short {
            self.hash ^= zt.castling[1];
        }
        if m.black_castle.long {
            self.hash ^= zt.castling[2];
        }
        if m.black_castle.short {
            self.hash ^= zt.castling[3];
        }
        if self.white_castle.long {
            self.hash ^= zt.castling[0];
        }
        if self.white_castle.short {
            self.hash ^= zt.castling[1];
        }
        if self.black_castle.long {
            self.hash ^= zt.castling[2];
        }
        if self.black_castle.short {
            self.hash ^= zt.castling[3];
        }

        if let Some(ep) = m.en_passant {
            self.hash ^= zt.en_passant[ep.col as usize];
        }
        if let Some(ep) = self.en_passant {
            self.hash ^= zt.en_passant[ep.col as usize];
        }

        self.hash ^= zt.side_to_move;

        // #[cfg(debug_assertions)]
        // {
        //     use crate::engine::zobris_table::hash_from_scratch;
        //     let next_player = if active_player == White { Black } else { White };
        //     let expected = hash_from_scratch(self, next_player);
        //     debug_assert_eq!(
        //         self.hash, expected,
        //         "hash mismatch after apply_move: got {:#x}, expected {:#x}",
        //         self.hash, expected
        //     );
        // }
    }

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

pub fn get_capture(m: &Move, active_player: &Color) -> Coord {
    match m.move_type {
        EnPassant => {
            let row = if *active_player == White {
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
    }
}
