use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode::*;

use egui::Context;

impl ChessApp {
    pub fn new_save_load(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            #[allow(clippy::collapsible_if)] // needed to hid new game button when unecessary
            if self.current.end.is_some() {
                if ui
                    .add_enabled(self.current.end.is_some(), egui::Button::new("New game"))
                    .clicked()
                {
                    //todo : separate replay / game_on / game_end : new game ne regenere pas tout !
                    //si un time gamemode autre que replay est set au click, on build en fonction
                    *self = ChessApp::default();
                }
                if ui
                    .add_enabled(self.app_mode != Replay, egui::Button::new("Replay"))
                    .clicked()
                {
                    self.app_mode = Replay;
                }
            }
            
        });
        ui.separator();
    }
}
