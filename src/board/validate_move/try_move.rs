use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece::*;
use crate::board::validate_move;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::chessapp_struct::GameState;
use crate::gui::chessapp_struct::PromoteInfo;

impl ChessApp {
    //this function takes two cells as the move input from the player
    //it test the move legality, and if it exposes king to a threat
    //If it passes these tests it update the board to end turn
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
        //if it's the very first move, we setup the history and timers if needed
        if self.history.is_empty() {
            self.history.push(self.current.clone());
            self.replay_infos.index += 1;
            //for mobile test
            self.app_mode = Versus(None);
            self.mobile_timer.active = true;
            self.mobile_timer.start_of_turn.1 = Some(White);
            self.replay_infos.index += 1;
            //Setup les timers ici ?
        }
        //it triggers a draw if true, before update board for pawn detection in case of promotion
        self.fifty_moves_draw_check(&from, &to);
        //This apply the move on the board
        self.current
            .board
            .update_board(&from, &to, &self.current.active_player);
        //it triggers a draw if the board match an impossible mat situation
        if self.impossible_mate_check() {
            self.current.end = Some(Draw);
            self.app_mode = Versus(Some(Draw));
        }
        //update castles bool state for both player
        self.update_castles(&to);
        //This add a hash for the 3 repetition draw
        //it takes player on trait, the grid, the castle and en_passant state
        //hash gives us the info if this exact situation happened
        self.add_hash();

        self.current.last_move = Some((from, to));

        if self.widgets.autoflip {
            self.widgets.flip = !self.widgets.flip;
        }
        self.incremente_turn();

        //checks for promotion
        //switch player color
        //check for mate, or pat and finaly for check situation
        self.events_check();

        //since we must end this function to allow gui to ask for promotion input, we store infos
        //needed here, and we skip the "normal end" so the gui will do it after getting the input
        let prev_board = self.history[self.replay_infos.index - 1].board.clone();
        if self.current.board.pawn_to_promote.is_some() {
            self.promoteinfo = Some(PromoteInfo {
                from,
                to,
                prev_board: prev_board.clone(),
            });
        } else {
            //if there were no promotion, we add the actual board in history, and inc the index
            self.history.push(self.current.clone());
            self.replay_infos.index += 1;
            self.encode_move_to_san(&from, &to, &prev_board);
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

    //update threats and legals moves to determine if it's a draw or a mat
    pub fn check_endgame(&mut self) {
        self.current
            .board
            .update_threatens_cells(&self.current.active_player);
        self.current
            .board
            .update_legals_moves(&self.current.active_player);
        //if there is no legal moves : it's a endgame
        //  if the king is threaten : its a mat
        //  else its a pat
        if self.current.board.legals_moves.is_empty() {
            self.current.board.print(); //souvenir of the cli version ..
            let king_cell = self.current.board.get_king(&self.current.active_player);
            if let Some(coord) = king_cell {
                if self.current.board.threaten_cells.contains(&coord) {
                    self.current.end = Some(Checkmate);
                    self.app_mode = Versus(Some(Checkmate));
                } else {
                    self.current.end = Some(Pat);
                    self.app_mode = Versus(Some(Pat));
                }
            }
        }
    }

    fn events_check(&mut self) {
        self.current.board.promote_pawn(&self.current.active_player);
        self.current.switch_players_color();
        self.check_endgame();
        // println!("{:?} to move", self.current.active_player);

        if let Some(k) = self.current.board.get_king(&self.current.active_player)
            && self.current.board.threaten_cells.contains(&k)
            && let Some(k) = self.current.board.get_king(&self.current.active_player)
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
}
