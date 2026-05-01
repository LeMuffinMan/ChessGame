use crate::Color::*;
use crate::engine::bot::PlayerType::*;
use crate::gui::chessapp::ChessApp;

impl ChessApp {
    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                !self.game.history.is_empty() || self.game.end.is_some(),
                egui::Button::new("New"),
            )
            .clicked()
        {
            //revoir : ne pas changer les settings !
            *self = ChessApp::new(self.ui_type.clone());
        }
    }
    pub fn revenge_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                !self.game.history.is_empty() || self.game.end.is_some(),
                egui::Button::new("Revenge"),
            )
            .clicked()
        {
            *self = ChessApp::revenge(self.ui_type.clone(), &self);
            if self.game.active_player == White && self.settings.white_bot != Human {
                self.bot_pending = true;
            }
        }
    }
}
