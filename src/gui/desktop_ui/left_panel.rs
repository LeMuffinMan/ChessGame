use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::AppMode::*;
use egui::Context;

impl ChessApp {
    //Shows turn infos, resign / draw options, new game option and timer options
    pub fn left_panel_desktop(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(270.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                if matches!(&self.app_mode, AppMode::Versus(None)) {
                    self.draw_resign_undo(ui);
                }
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.app_mode == Lobby {
                    self.timer_increment(ui, ctx);
                }
            });
    }
}
