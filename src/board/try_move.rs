use crate::Board;
use crate::ChessApp;
use crate::Color::*;
use crate::Coord;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::Move;
use crate::board::move_gen::MoveList;
use crate::board::move_gen::generate_moves;
use crate::gui::chessapp::AppMode::*;
use crate::gui::features::gamestate::GameState;
use crate::gui::hooks::windows::End::*;

impl ChessApp {
    pub fn try_move(&mut self, from: Coord, to: Coord) {
        if let Some(m) = self.validate_move(from, to) {
            let snapshot = self.current.clone();
            self.apply_move(&m);
            if is_king_exposed(&self.current.board, &self.current.active_player) {
                self.current = snapshot;
                return; //println!("Illegal move: {from:?} -> {to:?}: king would be threaten");
            }
            self.commit_move(snapshot, m, from, to);

            //since we must end this function to allow gui to ask for promotion input, we store infos
            //needed here, and we skip the "normal end" so the gui will do it after getting the input
            let prev_board = self.get_prev_board();

            self.turn_end(&from, &to, &prev_board);
        }
    }

    pub fn get_prev_board(&self) -> Board {
        self.history.snapshots[self.replay_infos.index - 1]
            .board
            .clone()
    }

    pub fn validate_move(&mut self, from: Coord, to: Coord) -> Option<Move> {
        let mut move_list = MoveList::new();
        generate_moves(
            &mut self.current.board,
            &self.current.active_player,
            &mut move_list,
            false,
        );
        let moves = &mut move_list.moves[..move_list.count];
        let legal = moves.iter().any(|m| m.origin == from && m.dest == to);
        if !legal {
            println!("Illegal move: {from:?} -> {to:?}");
            return None;
        }
        self.build_move(from, to)
    }

    pub fn build_move(&mut self, from: Coord, to: Coord) -> Option<Move> {
        Some(
            self.current
                .board
                .build_move(from, to, self.current.active_player),
        )
    }

    pub fn apply_move(&mut self, m: &Move) {
        self.current.board.apply_move(m, self.current.active_player);
    }

    pub fn add_history_san(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        self.history.snapshots.push(self.current.clone());
        self.replay_infos.index += 1;
        self.encode_move_to_san(&from, &to, &prev_board);
    }

    pub fn commit_move(&mut self, snapshot: GameState, m: Move, from: Coord, to: Coord) {
        //if it's the very first move, we setup the history and timers if needed
        if self.history.snapshots.is_empty() {
            self.init_history(&snapshot);
            self.init_timer();
        }
        self.add_snapshot(&snapshot);
        self.fifty_moves_draw_check(&m);

        //it triggers a draw if the board match an impossible mat situation (a detailler)
        if self.current.impossible_mate_check() {
            self.current.end = Some(Draw);
            self.app_mode = Versus(Some(Draw));
        }
        self.current.add_hash();
        self.update_turn_gui(from, to);
    }

    pub fn init_history(&mut self, snapshot: &GameState) {
        self.history.snapshots.push(snapshot.clone());
        self.replay_infos.index += 1;
        //for mobile only
        self.app_mode = Versus(None);
    }

    pub fn init_timer(&mut self) {
        self.timer.active = true;
        self.timer.start_of_turn.1 = Some(White);
    }

    pub fn add_snapshot(&mut self, snapshot: &GameState) {
        self.history.snapshots.push(snapshot.clone());
        self.replay_infos.index += 1;
    }

    pub fn incremente_turn(&mut self) {
        if self.current.active_player == Black {
            self.current.turn += 1;
        }
    }

    pub fn update_turn_gui(&mut self, from: Coord, to: Coord) {
        self.current.last_move = Some((from, to));
        if self.settings.autoflip {
            self.settings.flip = !self.settings.flip;
        }
        self.incremente_turn();
    }

    pub fn check_endgame(&mut self) {
        //if there is no legal moves : it's a endgame
        if self.current.legals_moves.is_empty() {
            // self.current.board.print(); //souvenir of the cli version ..
            let king_cell = match self.current.active_player {
                White => self.current.board.white_king,
                Black => self.current.board.black_king,
            };
            if self.current.threaten_cells.contains(&king_cell) {
                //  if the king is threaten : its a mat
                self.current.end = Some(Checkmate);
                self.timer.active = false;
                self.app_mode = Versus(Some(Checkmate));
            } else {
                //  else its a pat
                self.current.end = Some(Pat);
                self.app_mode = Versus(Some(Pat));
                self.timer.active = false;
            }
        }
    }

    pub fn update_legals_moves(&mut self) {
        let mut move_list = MoveList::new();
        generate_moves(
            &mut self.current.board,
            &self.current.active_player,
            &mut move_list,
            false,
        );
        self.current.legals_moves = move_list.moves[..move_list.count].to_vec();
    }

    pub fn update_threaten_cells(&mut self) {
        self.current.threaten_cells = self
            .current
            .board
            .update_threatens_cells(&self.current.active_player);
    }

    pub fn turn_end(&mut self, from: &Coord, to: &Coord, prev_board: &Board) {
        let active_player = self.current.active_player;
        if let Some(promote_info) = self.promote_pawn(&active_player, from, to, prev_board) {
            self.promoteinfo = Some(promote_info);
        }
        self.switch_turn();

        //if there were no promotion, we add the actual board in history, and incremente the index
        //if there was a promotion, the gui handler would do this part too
        if self.promoteinfo.is_none() {
            self.add_history_san(&from, &to, &prev_board);
            if self.current.end.is_none() && self.is_bot_turn() {
                self.bot_pending = true;
            }
        }
    }

    pub fn switch_turn(&mut self) {
        self.current.switch_players_color();
        self.update_threaten_cells();
        self.update_legals_moves();
        self.check_endgame();
        let k = match self.current.active_player {
            White => self.current.board.white_king,
            Black => self.current.board.black_king,
        };
        if self.current.threaten_cells.contains(&k) {
            self.current.board.check = Some(k);
            println!("Check !");
        }
    }
}
