use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece::*;
use crate::board::validate_move;
use crate::gui::chessapp_struct::AppMode::*;
use crate::gui::chessapp_struct::End::*;
// use crate::gui::chessapp_struct::PromoteInfo;

impl ChessApp {
    //this function takes two cells as the move input from the player
    //it test the move legality, and if it exposes king to a threat
    //If it passes these tests it update the board to end turn
    pub fn try_move(&mut self, from: Coord, to: Coord) {
        // cette fonction doit returner true false ou <Ok(), Result>
        //Does th rules allow this move ?
        if !self.current.board.is_legal_move(
            &from,
            &to,
            &self.current.active_player,
            &self.current.threaten_cells,
        ) {
            println!("Illegal move: {from:?} -> {to:?}");
            return;
        }

        let m = self
            .current
            .board
            .build_move(from, to, self.current.active_player); //erreur a gerer ?

        let snapshot = self.current.clone();

        self.current
            .board
            .apply_move(&m, self.current.active_player);

        if validate_move::is_king_exposed(&self.current.board, &self.current.active_player) {
            self.current = snapshot;
            // self.current.board.undo_move(m, self.current.active_player);
            //Faire remonter l'erreur
            println!("Illegal move: {from:?} -> {to:?}: king would be threaten");
            return;
        }

        //Doing this move, is the active player king threaten ?
        // if validate_move::is_king_exposed(
        //     &from,
        //     &to,
        //     &self.current.active_player,
        //     &self.current.board,
        // ) {
        //     println!("King is exposed: illegal move");
        //     return;
        // }

        //Tout ca devrait sortir de try move et serait un update app ?
        //if it's the very first move, we setup the history and timers if needed
        if self.history.snapshots.is_empty() {
            self.history.snapshots.push(snapshot.clone());
            self.replay_infos.index += 1;
            //for mobile test
            self.app_mode = Versus(None);
            self.timer.active = true;
            self.timer.start_of_turn.1 = Some(White);
            //Setup les timers ici ?
        }
        self.history.snapshots.push(snapshot);
        self.replay_infos.index += 1;
        //it triggers a draw if true, before update board for pawn detection in case of promotion
        //a reintegrer
        self.current.fifty_moves_draw_check(&m);
        //This apply the move on the board
        // self.current
        //     .board
        //     .update_board(&from, &to, &self.current.active_player);

        // self.current
        //     .board
        //     .apply_move(&m, self.current.active_player);

        //it triggers a draw if the board match an impossible mat situation
        if self.current.impossible_mate_check() {
            self.current.end = Some(Draw);
            self.app_mode = Versus(Some(Draw));
        }
        //update castles bool state for both player
        self.update_castles(&to);
        //This add a hash for the 3 repetition draw
        //it takes player on trait, the grid, the castle and en_passant state
        //hash gives us the info if this exact situation happened
        self.current.add_hash();

        self.current.last_move = Some((from, to));

        if self.settings.autoflip {
            self.settings.flip = !self.settings.flip;
        }
        self.incremente_turn();

        //since we must end this function to allow gui to ask for promotion input, we store infos
        //needed here, and we skip the "normal end" so the gui will do it after getting the input
        let prev_board = self.history.snapshots[self.replay_infos.index - 1]
            .board
            .clone();

        //checks for promotion
        //switch player color
        //check for mate, or pat and finaly for check situation
        self.events_check(&from, &to, &prev_board);

        //la retouche de ce if a peut etre casse la promotion
        if !self.promoteinfo.is_some() {
            //if there were no promotion, we add the actual board in history, and inc the index
            self.history.snapshots.push(self.current.clone());
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
        self.current.threaten_cells = self
            .current
            .board
            .update_threatens_cells(&self.current.active_player);
        self.current.legals_moves = self
            .current
            .board
            .update_legals_moves(&self.current.active_player, &self.current.threaten_cells);

        //if there is no legal moves : it's a endgame
        //  if the king is threaten : its a mat
        //  else its a pat
        if self.current.legals_moves.is_empty() {
            self.current.board.print(); //souvenir of the cli version ..
            let king_cell = self.current.board.get_king(&self.current.active_player);
            if let Some(coord) = king_cell {
                if self.current.threaten_cells.contains(&coord) {
                    self.current.end = Some(Checkmate);
                    self.timer.active = false;
                    self.app_mode = Versus(Some(Checkmate));
                } else {
                    self.current.end = Some(Pat);
                    self.app_mode = Versus(Some(Pat));
                    self.timer.active = false;
                }
            }
        }
    }

    fn events_check(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        let active_player = self.current.active_player;
        if let Some(promote_info) = self.promote_pawn(&active_player, from, to, prev_board) {
            self.promoteinfo = Some(promote_info);
        }
        self.current.switch_players_color();
        self.check_endgame();
        // println!("{:?} to move", self.current.active_player);
        // On peut virer le fct get king et acceder drect a la struct plutot ?
        if let Some(k) = self.current.board.get_king(&self.current.active_player)
            && self.current.threaten_cells.contains(&k)
            && let Some(k) = self.current.board.get_king(&self.current.active_player)
        {
            self.current.board.check = Some(k);
            // println!("Check !");
        }
    }
}
