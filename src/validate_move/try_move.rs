use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::PromoteInfo;
use crate::validate_move;
use crate::gui::chessapp_struct::GameState;
use crate::gui::chessapp_struct::End::Draw;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::DrawRule::*;

impl ChessApp {
    pub fn add_hash(&mut self) {
        //Si on a deux situation identiques en stock
        //que le joueur peut la reproduire
        //      Ajouter un bouton Draw
        //          Si il clique : Draw
        //Si a add hash on a le flag draw leve :
        //      si le hash apparait deja deux fois : On ecrase les 3 hashs
        //      sinon on l'ajoute
        //
        //
        //si oui on propose la nulle au joueur qui va produire la repetition 
        let mut hasher = DefaultHasher::new();
        //il faut recuperer aussi l'etat des roques et des en passant et tous les coups legaux !!
        // ((bool, bool), (bool, bool), grid, trait, en_passant)
        self.current.board.grid.hash(&mut hasher);
        let hash_value = hasher.finish();
        let count = self.board_hashs.entry(hash_value).or_insert(0);
        *count += 1;
        if *count >= 3 {
            self.draw_option = Some(Available(TripleRepetition));
        }
    }

    fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        if let Some(p) = self.current.board.get(from).get_piece() {
            match p {
                Pawn => {
                    self.draw_moves_count = 0; 
                    return
                },
                _ => { },
            };
        }
        if !self.current.board.get(to).is_empty() {
            self.draw_moves_count = 0;
            return ;
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
        for x in 0.. 8 {
            for y in 0 ..7 {
                if let Some(piece) = self.current.board.grid[x][y].get_piece()
                    && let Some(color) = self.current.board.grid[x][y].get_color() {
                        let cell_color = if (x + y) % 2 == 0 { White } else { Black };    
                        vec.push((*piece, *color, cell_color));  
                }
            }
        }
        vec
    }

    fn impossible_mate_check(&mut self) -> bool {
        let pieces = self.get_pieces_on_board();
        println!("pieces on board : {:?}", pieces);
        match pieces.len() {
            2 => return true,
            3 => {
                for (piece, _, _) in pieces {
                    if piece != King && piece != Bishop && piece != Knight {
                        return false
                    }
                }
                return true
            },
            4 => {
                let mut white_bishop_cell_color = None;
                let mut black_bishop_cell_color = None;
                for (piece, color, cell_color) in pieces {
                    if piece != King && piece != Bishop {
                        return false
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
                    return true
                }
                return false
            },
            _ => { return false },
        };
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
        self.add_hash();
        self.update_castles(&to);
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
                    from: from,
                    to: to,
                    prev_board: prev_board.clone(),
                });
            } else {
                self.from_move_to_san(&from, &to, &prev_board);
            }
        }
    }

    fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    fn update_castles(&mut self, to: &Coord) {
        let mut castle_tuple = self
            .current
            .board
            .get_castle_tuple(&self.current.active_player);
        if let Some(piece) = self.current.board.get(&to).get_piece() {
            match piece {
                Rook => {
                    match to.col {
                        7 => castle_tuple.0 = false,
                        0 => castle_tuple.1 = false,
                        _ => {}
                    };
                }
                King => {
                    castle_tuple.0 = false;
                    castle_tuple.1 = false;
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

        if let Some(k) = self.current.board.get_king(&active_player) {
            if self.current.board.threaten_cells.contains(&k) {
                if let Some(k) = self.current.board.get_king(&opponent) {
                    self.current.board.check = Some(k);
                }
                // println!("Check !");
            }
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
}


