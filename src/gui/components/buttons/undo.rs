use crate::Color::*;
use crate::gui::chessapp::ChessApp;
use crate::gui::hooks::windows::WinDia;

impl ChessApp {
    pub fn is_undoable(&self) -> bool {
        self.settings.allow_undo && (self.settings.white_undo > 0 || self.settings.black_undo > 0)
    }

    fn is_undo_to_display(&mut self) -> bool {
        self.current.end.is_none()
            && self.can_undo()
            && self.win.is_none()
            && (self.history.snapshots.len() > 1
                || self.history.snapshots.len() == 2 && self.current.active_player == White)
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
        match self.current.opponent {
            White => {
                self.settings.white_undo -= 1;
            }
            Black => {
                self.settings.black_undo -= 1;
            }
        }
    }
}
