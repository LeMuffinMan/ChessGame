use crate::ChessApp;
use crate::Coord;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
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

        match self.draw.board_hashs.entry(hash_value) {
            Entry::Vacant(e) => {
                e.insert(1);
            }
            Entry::Occupied(_) => {
                //si les legals moves permettent la repetition
                self.draw.draw_option = Some(Available(TripleRepetition));
                self.draw.draw_hash = Some(hash_value);
            }
        }

        if let Some(h) = self.draw.draw_hash {
            self.draw.board_hashs.remove(&h);
        }
    }

    pub fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        if let Some(p) = self.current.board.get(from).get_piece()
            && p == &Pawn
        {
            self.draw.draw_moves_count = 0;
            return;
        }
        if !self.current.board.get(to).is_empty() {
            self.draw.draw_moves_count = 0;
            return;
        }
        self.draw.draw_moves_count += 1;
        // println!("{:?}", self.draw.draw_moves_count);
        if self.draw.draw_moves_count >= 50 {
            self.draw.draw_option = Some(Available(FiftyMoves));
        } else {
            self.draw.draw_option = None;
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

    pub fn impossible_mate_check(&mut self) -> bool {
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
}
