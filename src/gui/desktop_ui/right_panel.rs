use crate::ChessApp;

use egui::Context;

impl ChessApp {
    //shows the checkbox for visual settings, replay buttons and the san history
    pub fn right_panel_desktop(&mut self, ctx: &Context) {
        egui::SidePanel::right("right_panel")
            .default_width(270.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        self.side_panel_flip(ui);
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.widgets.show_coordinates, "Coordinates")
                            .changed();
                        ui.add_space(20.0);
                        ui.label("Highlight :");
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.widgets.show_legals_moves, "Legals moves")
                            .changed();
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.widgets.show_threaten_cells, "Threaten cells");
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.widgets.show_last_move, "Last move")
                            .changed();
                        ui.add_space(20.0);
                        ui.separator();
                        if !self.history_san.is_empty() {
                            ui.monospace(&self.history_san);
                        }
                    });
                });
            });
    }
}
