use crate::ChessApp;
use egui::Context;

impl ChessApp {
    pub fn right_panel_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("right_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.side_panel_flip(ui);
                ui.checkbox(&mut self.widgets.show_coordinates, "Coordinates")
                    .changed();
                ui.label("Highlight :");
                ui.checkbox(&mut self.widgets.show_legals_moves, "Legals moves")
                    .changed();
                ui.checkbox(&mut self.widgets.show_threaten_cells, "Threaten cells");
                ui.checkbox(&mut self.widgets.show_last_move, "Last move")
                    .changed();
                ui.separator();
                self.replay_panel(ui);
                if !self.history_san.is_empty() {
                    ui.monospace(&self.history_san);
                }
            });
    }
}
