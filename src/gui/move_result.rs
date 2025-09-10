use crate::Coord;
use crate::Color;
use crate::ChessApp;
use crate::validate_move;
use crate::mat_or_pat;

impl ChessApp {

    pub fn try_apply_move(&mut self, from: Coord, to: Coord) {
        if !self.current.board.is_legal_move(&from, &to, &self.current.active_player) {
            println!("Illegal move: {from:?} -> {to:?}");
            // msgs.push(format!("Illegal move : {from:?} -> {to:?}"));
        }
        if validate_move::is_king_exposed(&from, &to, &self.current.active_player, &self.current.board) {
            println!("King is exposed: illegal move");
            // msgs.push("King is exposed : illegal move".into());
            // return Some(MoveOutcome { applied: false, mate: false, pat:false, check: false, messages: msgs });
        }
        self.from_move_to_pgn((from, to));
        self.undo.push(self.current.clone());
        self.current.board.update_board(&from, &to, &self.current.active_player);

        self.redo.clear();
        self.current.last_move = Some((from, to));
        if self.autoflip {
            self.flip = !self.flip;
        }
        if self.current.active_player == Color::Black {
            self.current.turn += 1;
        }
        self.current.active_player = match self.current.active_player { Color::White => Color::Black, Color::Black => Color::White };

        let (end, mate) = mat_or_pat(&mut self.current.board, &self.current.active_player);
        if end {
            if mate {
                self.current.checkmate = true;
            } else {
                self.current.pat = true;
            }
        }

        println!("{:?} to move", self.current.active_player);
        if let Some(k) = self.current.board.get_king(&self.current.active_player) {
            if self.current.board.threaten_cells.contains(&k) {
                self.current.board.check = true;
                println!("Check !");
            }
        }
    }

}
