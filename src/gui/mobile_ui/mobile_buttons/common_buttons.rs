use crate::ChessApp;
use crate::gui::hooks::windows::WinDia::*;
use crate::gui::layout::UiType::Mobile;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.win = Some(Settings);
        }
    }

    pub fn new_game_button_mobile(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(Mobile);
        }
    }
}
