use crate::ChessApp;
use crate::gui::chessapp_struct::End;

use egui::RichText;

impl ChessApp {
    //Inform on the current game state, player to move, check, or endgame
    pub fn mobile_turn_infos(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(format!("Turn #{}", self.current.turn)).size(40.0));
        if let Some(end) = &self.current.end {
            match end {
                End::Checkmate => ui.label(
                    RichText::new(format!("Checkmate ! {:?} win", self.current.opponent))
                        .size(40.0),
                ),
                End::TimeOut => ui.label(
                    RichText::new(format!(
                        "{:?} out of time !\n{:?} win",
                        self.current.active_player, self.current.opponent
                    ))
                    .size(40.0),
                ),
                End::Pat => ui.label(RichText::new("Pat !").size(40.0)),
                End::Draw => ui.label(RichText::new("Draw").size(40.0)),
                End::Resign => ui.label(
                    RichText::new(format!(
                        "{:?} resigned : {:?} win",
                        self.current.active_player, self.current.opponent
                    ))
                    .size(40.0),
                ),
            };
        } else {
            // ui.horizontal(|ui| {
            if self.current.board.check.is_some() {
                ui.label(
                    RichText::new(format!("Check ! {:?} to move", self.current.active_player))
                        .size(40.0),
                );
            } else {
                ui.label(
                    RichText::new(format!("{:?} to move", self.current.active_player)).size(40.0),
                );
            }
        }
    }
}
