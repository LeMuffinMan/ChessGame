use crate::ChessApp;
use crate::game::End;

impl ChessApp {
    //Inform on the current game state, player to move, check, or endgame
    pub fn turn_infos(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Turn #{}", self.game.turn));
        if let Some(end) = &self.game.end {
            match end {
                End::Checkmate => ui.label(format!("Checkmate ! {:?} win", self.game.opponent())),
                End::TimeOut => ui.label(format!(
                    "{:?} out of time !\n{:?} win",
                    self.game.active_player, self.game.opponent()
                )),
                End::Pat => ui.label("Pat !"),
                End::Draw => ui.label("Draw"),
                End::Resign => ui.label(format!(
                    "{:?} resigned : {:?} win",
                    self.game.active_player, self.game.opponent()
                )),
            };
        } else {
            if self.game.board.check.is_some() {
                ui.label("Check !");
            }
            ui.horizontal(|ui| {
                ui.label(format!("{:?} to move", self.game.active_player));
            });
        }
    }
}
