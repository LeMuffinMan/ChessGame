use crate::gui::chessapp::AppMode::*;
use crate::gui::chessapp::ChessApp;

impl ChessApp {
    pub fn replay_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.app_mode != Replay, egui::Button::new("Replay"))
            .clicked()
        {
            self.app_mode = Replay;
        }
    }
}
