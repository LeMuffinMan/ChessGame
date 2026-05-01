use crate::ChessApp;
use crate::game::End;
use crate::gui::layout::UiType::*;
use egui::RichText;

impl ChessApp {
    pub fn turn_infos(&mut self, ui: &mut egui::Ui) {
        let sz = match self.ui_type { Mobile => 40.0, Desktop => 30.0 };
        let rt = |s: String| RichText::new(s).size(sz);

        ui.label(rt(format!("Turn #{}", self.game.turn)));
        if let Some(end) = &self.game.end {
            match end {
                End::Checkmate => {
                    ui.label(rt(format!("Checkmate ! {:?} win", self.game.opponent())));
                }
                End::TimeOut => {
                    ui.label(rt(format!(
                        "{:?} out of time ! {:?} win",
                        self.game.active_player,
                        self.game.opponent()
                    )));
                }
                End::Pat => { ui.label(rt("Pat !".into())); }
                End::Draw => { ui.label(rt("Draw".into())); }
                End::Resign => {
                    ui.label(rt(format!(
                        "{:?} resigned : {:?} win",
                        self.game.active_player,
                        self.game.opponent()
                    )));
                }
            };
        } else if self.game.board.check.is_some() {
            ui.label(rt(format!("Check ! {:?} to move", self.game.active_player)));
        } else {
            ui.label(rt(format!("{:?} to move", self.game.active_player)));
        }
    }
}
