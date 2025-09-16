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
    //for the 3 repetition draw, we need to check only if the a same situation happenned 2 times
    //and if the player can legaly reproduce a 3rd repetition
    //the situation to compare include castle state, en passant state, grid and active player
    //Instead of comparing each of these variables to verify the repetition, we can make a hash of
    //these variables together. Comparing hash will give us the info "is it the exact same as
    //somehting we found", without actually comparing the content. Only it's signature.
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

        //if the entry is occupied, it means it's a 2nd repetition
        //the ui will hook on it and seek if the next move could produce a 3rd reptition
        //if yes, it offers to the opponnent to claim the draw
        match self.draw.board_hashs.entry(hash_value) {
            Entry::Vacant(e) => {
                e.insert(1);
            }
            Entry::Occupied(_) => {
                //si les legals moves permettent la repetition // todo
                self.draw.draw_option = Some(Available(TripleRepetition));
                self.draw.draw_hash = Some(hash_value);
            }
        }
        //if the hash we juste made, is the same as the one that could be stored as reproduced
        //twice, it means the player didnt claim draw, so he loose the all sequence : we delete the
        //hash made and the one in the hashtable
        //Since the active player variable is hashed too, it does not delete the other player
        //entries
        if let Some(h) = self.draw.draw_hash {
            self.draw.board_hashs.remove(&h);
        }
    }

    //after 50 moves without pawn moves or capture, it triggers a draw
    pub fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        //if a pawn moved, wthe counter reset
        if let Some(p) = self.current.board.get(from).get_piece()
            && p == &Pawn
        {
            self.draw.draw_moves_count = 0;
            return;
        }
        //if a capture occured, we reset the counter
        //this is why we need to call this fct before updating the baord
        if !self.current.board.get(to).is_empty() {
            self.draw.draw_moves_count = 0;
            return;
        }
        //incremente in any other case
        self.draw.draw_moves_count += 1;
        //triggers the draw if count reached 50
        if self.draw.draw_moves_count >= 50 {
            self.draw.draw_option = Some(Available(FiftyMoves));
        } else {
            self.draw.draw_option = None;
        }
    }

    //returns a tuple of all the occupied cells on the board
    //the piece occupiying it, it's color, and the "cell color" (for imposible mate situation)
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

    //Some hard coded situation where there is no mat legaly possible : draw
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
                    if piece == Bishop {
                        if color == White {
                            white_bishop_cell_color = Some(cell_color);
                        } else {
                            black_bishop_cell_color = Some(cell_color);
                        }
                    }
                }
                if white_bishop_cell_color.is_some() != black_bishop_cell_color.is_some() {
                    let Some(cell_1) = white_bishop_cell_color else {
                        return false;
                    };
                    let Some(cell_2) = white_bishop_cell_color else {
                        return false;
                    };
                    if cell_1 == cell_2 {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}
