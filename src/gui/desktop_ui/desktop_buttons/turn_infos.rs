use crate::ChessApp;
use crate::board::board_struct::End;

impl ChessApp {
    //Inform on the board game state, player to move, check, or endgame
    pub fn turn_infos(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Turn #{}", self.board.turn));
        if let Some(end) = &self.board.end {
            match end {
                End::Checkmate => ui.label(format!("Checkmate ! {:?} win", self.board.opponent)),
                End::TimeOut => ui.label(format!(
                    "{:?} out of time !\n{:?} win",
                    self.board.active_player, self.board.opponent
                )),
                End::Pat => ui.label("Pat !"),
                End::Draw => ui.label("Draw"),
                End::Resign => ui.label(format!(
                    "{:?} resigned : {:?} win",
                    self.board.active_player, self.board.opponent
                )),
            };
        } else {
            if self.board.check.is_some() {
                ui.label("Check !");
            }
            ui.horizontal(|ui| {
                ui.label(format!("{:?} to move", self.board.active_player));
            });
        }
    }
}
