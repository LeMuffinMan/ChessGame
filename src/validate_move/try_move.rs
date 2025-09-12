use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End::Draw;
use crate::gui::chessapp_struct::GameState;
use crate::gui::chessapp_struct::PromoteInfo;
use crate::validate_move;

use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::DrawRule::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::Entry;
use std::hash::{Hash, Hasher};

impl ChessApp {
    pub fn add_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        let state = (
            (
                self.current.board.white_castle,
                self.current.board.black_castle,
            ),
            self.current.board.en_passant,
            self.current.active_player,
            self.current.board.grid,
        );
        state.hash(&mut hasher);
        let hash_value = hasher.finish();

        match self.board_hashs.entry(hash_value) {
            Entry::Vacant(e) => {
                e.insert(1);
            }
            Entry::Occupied(_) => {
                self.draw_option = Some(Available(TripleRepetition));
                self.draw_hash = Some(hash_value);
            }
        }

        if let Some(h) = self.draw_hash {
            self.board_hashs.remove(&h);
        }
    }

    fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        if let Some(p) = self.current.board.get(from).get_piece()
            && p == &Pawn
        {
            self.draw_moves_count = 0;
            return;
        }
        if !self.current.board.get(to).is_empty() {
            self.draw_moves_count = 0;
            return;
        }
        self.draw_moves_count += 1;
        // println!("{:?}", self.draw_moves_count);
        if self.draw_moves_count >= 50 {
            self.draw_option = Some(Available(FiftyMoves));
        } else {
            self.draw_option = None;
        }
    }

    fn get_pieces_on_board(&mut self) -> Vec<(Piece, Color, Color)> {
        let mut vec = Vec::new();
        for x in 0..8 {
            for y in 0..7 {
                if let Some(piece) = self.current.board.grid[x][y].get_piece()
                    && let Some(color) = self.current.board.grid[x][y].get_color()
                {
                    let cell_color = if (x + y) % 2 == 0 { White } else { Black };
                    vec.push((*piece, *color, cell_color));
                }
            }
        }
        vec
    }

    fn impossible_mate_check(&mut self) -> bool {
        let pieces = self.get_pieces_on_board();
        // println!("pieces on board : {:?}", pieces);
        match pieces.len() {
            2 => true,
            3 => {
                for (piece, _, _) in pieces {
                    if piece != King && piece != Bishop && piece != Knight {
                        return false;
                    }
                }
                true
            }
            4 => {
                let mut white_bishop_cell_color = None;
                let mut black_bishop_cell_color = None;
                for (piece, color, cell_color) in pieces {
                    if piece != King && piece != Bishop {
                        return false;
                    }
                    white_bishop_cell_color = if piece == Bishop && color == White {
                        Some(cell_color)
                    } else {
                        None
                    };
                    black_bishop_cell_color = if piece == Bishop && color == Black {
                        Some(cell_color)
                    } else {
                        None
                    };
                }
                if white_bishop_cell_color != black_bishop_cell_color {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub fn try_move(&mut self, from: Coord, to: Coord) {
        if !self
            .current
            .board
            .is_legal_move(&from, &to, &self.current.active_player)
        {
            println!("Illegal move: {from:?} -> {to:?}");
            return;
        }
        if validate_move::is_king_exposed(
            &from,
            &to,
            &self.current.active_player,
            &self.current.board,
        ) {
            println!("King is exposed: illegal move");
            return;
        }
        self.undo.push(self.current.clone());
        self.fifty_moves_draw_check(&from, &to);
        self.current
            .board
            .update_board(&from, &to, &self.current.active_player);
        if self.impossible_mate_check() {
            self.current.end = Some(Draw);
        }
        self.update_castles(&to);
        self.add_hash();
        self.redo.clear();
        self.current.last_move = Some((from, to));
        if self.autoflip {
            self.flip = !self.flip;
        }
        self.incremente_turn();
        self.events_check();
        if let Some(prev_state) = self.undo.last() {
            let prev_board = &prev_state.board.clone();
            if self.current.board.pawn_to_promote.is_some() {
                self.promoteinfo = Some(PromoteInfo {
                    from,
                    to,
                    prev_board: prev_board.clone(),
                });
            } else {
                self.encode_move_to_san(&from, &to, prev_board);
            }
        }
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    fn update_castles(&mut self, to: &Coord) {
        if let Some(piece) = self.current.board.get(to).get_piece() {
            match piece {
                Rook => {
                    match to.col {
                        7 => self.current.switch_castle(false, true),
                        0 => self.current.switch_castle(true, false),
                        _ => {}
                    };
                }
                King => {
                    self.current.switch_castle(false, false);
                }
                _ => {}
            }
        };
    }

    fn events_check(&mut self) {
        self.current.board.promote_pawn(&self.current.active_player);
        self.current.switch_players_color();
        self.check_endgame();
        // println!("{:?} to move", self.current.active_player);
        let active_player = if self.current.active_player == White {
            White
        } else {
            Black
        };
        let opponent = if self.current.active_player != White {
            White
        } else {
            Black
        };

        if let Some(k) = self.current.board.get_king(&active_player)
            && self.current.board.threaten_cells.contains(&k)
            && let Some(k) = self.current.board.get_king(&opponent)
        {
            self.current.board.check = Some(k);
            // println!("Check !");
        }
    }
}

impl GameState {
    pub fn switch_players_color(&mut self) {
        self.active_player = match self.active_player {
            White => Black,
            Black => White,
        };
        self.opponent = match self.opponent {
            White => Black,
            Black => White,
        };
    }

    pub fn switch_castle(&mut self, long: bool, short: bool) {
        let castle_tuple = if self.active_player == White {
            &mut self.board.white_castle
        } else {
            &mut self.board.black_castle
        };
        castle_tuple.0 = long;
        castle_tuple.1 = short;
    }
}
