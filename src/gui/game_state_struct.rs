use crate::Board;
use crate::Coord;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End;
use crate::gui::game_state_struct::DrawOption::*;
use crate::gui::game_state_struct::DrawRule::*;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub active_player: Color,
    pub opponent: Color,
    pub end: Option<End>,
    pub last_move: Option<(Coord, Coord)>,
    pub turn: u32,
    pub draw: LateDraw,
}

#[derive(Clone, PartialEq)]
pub enum DrawOption {
    //faire une option
    Request,
    Available(DrawRule),
}

#[derive(Clone, PartialEq)]
pub enum DrawRule {
    TripleRepetition,
    FiftyMoves,
}

#[derive(Clone, PartialEq)]
pub struct LateDraw {
    pub board_hashs: HashMap<u64, usize>,
    pub draw_option: Option<DrawOption>,
    pub draw_moves_count: u32,
    pub draw_hash: Option<u64>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::init_board(),
            active_player: Color::White,
            opponent: Color::Black,
            end: None,
            last_move: None,
            turn: 1,
            draw: LateDraw {
                board_hashs: HashMap::new(),
                draw_option: None,
                draw_moves_count: 0,
                draw_hash: None,
            },
        }
    }

    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        let cell = self.board.get(coord);
        cell.is_color(&self.active_player)
    }

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

    //to rename : easier access to set the tuple of castles bools
    pub fn switch_castle(&mut self, long: bool, short: bool) {
        let castle_tuple = if self.active_player == White {
            &mut self.board.white_castle
        } else {
            &mut self.board.black_castle
        };
        castle_tuple.0 = long;
        castle_tuple.1 = short;
    }

    //for the 3 repetition draw, we need to check only if the a same situation happenned 2 times
    //and if the player can legaly reproduce a 3rd repetition
    //the situation to compare include castle state, en passant state, grid and active player
    //Instead of comparing each of these variables to verify the repetition, we can make a hash of
    //these variables together. Comparing hash will give us the info "is it the exact same as
    //somehting we found", without actually comparing the content. Only it's signature.
    pub fn add_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        let state = (
            (self.board.white_castle, self.board.black_castle),
            self.board.en_passant,
            self.active_player,
            self.board.grid,
        );
        state.hash(&mut hasher);
        let hash_value = hasher.finish();

        let count = self.draw.board_hashs.entry(hash_value).or_insert(0);
        *count += 1;

        if *count == 3 {
            self.draw.draw_option = Some(Available(TripleRepetition));
            self.draw.draw_hash = Some(hash_value);
        }

        //if the hash we saved was not used : the player can't claim this hash and the 2 previous
        if let Some(h) = self.draw.draw_hash {
            if let Some(count) = self.draw.board_hashs.get_mut(&h) {
                *count = 0;
            }
        }
    }

    //after 50 moves without pawn moves or capture, it triggers a draw
    pub fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        //if a pawn moved, wthe counter reset
        if let Some(p) = self.board.get(from).get_piece()
            && p == &Pawn
        {
            self.draw.draw_moves_count = 0;
            return;
        }
        //if a capture occured, we reset the counter
        //this is why we need to call this fct before updating the baord
        if !self.board.get(to).is_empty() {
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
                if let Some(piece) = self.board.grid[x][y].get_piece()
                    && let Some(color) = self.board.grid[x][y].get_color()
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
