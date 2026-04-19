use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::generate_moves;
use crate::gui::appmode::AppMode::*;
use crate::gui::end::End::*;

impl ChessApp {
    pub fn try_move(&mut self, from: Coord, to: Coord) {
        let legal = generate_moves(&mut self.current.board, &self.current.active_player)
            .iter()
            .any(|m| m.origin == from && m.dest == to);
        if !legal {
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

        if is_king_exposed(&self.current.board, &self.current.active_player) {
            self.current = snapshot;
            // self.current.board.undo_move(m, self.current.active_player);
            //Faire remonter l'erreur
            println!("Illegal move: {from:?} -> {to:?}: king would be threaten");
            return;
        }

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
        //it triggers a draw if the board match an impossible mat situation
        if self.current.impossible_mate_check() {
            self.current.end = Some(Draw);
            self.app_mode = Versus(Some(Draw));
        }
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

        if !self.promoteinfo.is_some() {
            //if there were no promotion, we add the actual board in history, and inc the index
            self.history.snapshots.push(self.current.clone());
            self.replay_infos.index += 1;
            self.encode_move_to_san(&from, &to, &prev_board);
            if self.current.end.is_none() && self.is_bot_turn() {
                self.play_bot_turn();
            }
        }
    }

    pub fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    pub fn check_endgame(&mut self) {
        self.current.threaten_cells = self
            .current
            .board
            .update_threatens_cells(&self.current.active_player);
        self.current.legals_moves =
            generate_moves(&mut self.current.board, &self.current.active_player);

        //if there is no legal moves : it's a endgame
        //  if the king is threaten : its a mat
        //  else its a pat
        if self.current.legals_moves.is_empty() {
            self.current.board.print(); //souvenir of the cli version ..
            let king_cell = match self.current.active_player {
                White => self.current.board.white_king,
                Black => self.current.board.black_king,
            };
            if self.current.threaten_cells.contains(&king_cell) {
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

    pub fn events_check(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        let active_player = self.current.active_player;
        if let Some(promote_info) = self.promote_pawn(&active_player, from, to, prev_board) {
            self.promoteinfo = Some(promote_info);
        }
        self.current.switch_players_color();
        self.check_endgame();
        let k = match self.current.active_player {
            White => self.current.board.white_king,
            Black => self.current.board.black_king,
        };
        if self.current.threaten_cells.contains(&k) {
            self.current.board.check = Some(k);
            // println!("Check !");
        }
    }
}
