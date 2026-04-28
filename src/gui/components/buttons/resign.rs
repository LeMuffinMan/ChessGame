use crate::gui::chessapp::ChessApp;
use crate::gui::hooks::windows::WinDia;

impl ChessApp {
    pub fn resign_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.game.end.is_none() && self.win.is_none(),
                egui::Button::new("Resign"),
            )
            .clicked()
        {
            self.win = Some(WinDia::Resign);
        }
    }
}
