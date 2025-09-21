use crate::ChessApp;
use crate::gui::chessapp_struct::UiType::Mobile;
use crate::gui::hooks::WinDia::*;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.win = Some(Settings);
        }
    }

    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(Mobile);
        }
    }
}
