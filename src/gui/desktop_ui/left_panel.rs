use crate::ChessApp;

use egui::Context;

impl ChessApp {
    //Shows turn infos, resign / draw options, new game option and timer options
    pub fn left_panel_desktop(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(230.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                if !self.history.is_empty() {
                    self.draw_resign_undo(ui);
                }
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.history.is_empty() || self.current.end.is_some() {
                    self.timer_increment(ui, ctx);
                }
            });
    }
}
