use crate::ChessApp;
use crate::gui::hooks::windows::WinDia::Settings;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.win = Some(Settings);
        }
    }
}
