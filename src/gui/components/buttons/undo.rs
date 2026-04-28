use crate::Color::*;
use crate::gui::chessapp::ChessApp;
use crate::gui::hooks::windows::WinDia;

impl ChessApp {
    pub fn is_undoable(&self) -> bool {
        self.settings.allow_undo && (self.settings.white_undo > 0 || self.settings.black_undo > 0)
    }

    fn is_undo_to_display(&mut self) -> bool {
        self.game.end.is_none()
            && self.can_undo()
            && self.win.is_none()
            && (self.game.history.len() > 1
                || self.game.history.len() == 2 && self.game.active_player == White)
    }

    pub fn undo_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.is_undo_to_display(), egui::Button::new("Undo"))
            .clicked()
        {
            self.win = Some(WinDia::Undo);
            self.decremente_undo();
        }
    }
    pub fn decremente_undo(&mut self) {
        match self.game.opponent() {
            White => {
                self.settings.white_undo -= 1;
            }
            Black => {
                self.settings.black_undo -= 1;
            }
        }
    }
}
