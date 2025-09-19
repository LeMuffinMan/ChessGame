use crate::ChessApp;
use crate::gui::chessapp_struct::End;

impl ChessApp {
    //Inform on the current game state, player to move, check, or endgame
    pub fn turn_infos(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Turn #{}", self.current.turn));
        if let Some(end) = &self.current.end {
            match end {
                End::Checkmate => ui.label(format!("Checkmate ! {:?} win", self.current.opponent)),
                End::TimeOut => ui.label(format!(
                    "{:?} out of time !\n{:?} win",
                    self.current.active_player, self.current.opponent
                )),
                End::Pat => ui.label("Pat !"),
                End::Draw => ui.label("Draw"),
                End::Resign => ui.label(format!(
                    "{:?} resigned : {:?} win",
                    self.current.active_player, self.current.opponent
                )),
            };
        } else {
            ui.horizontal(|ui| {
                if self.current.board.check.is_some() {
                    ui.label("Check !");
                }
                ui.label(format!("{:?} to move", self.current.active_player));
            });
        }
    }
}
