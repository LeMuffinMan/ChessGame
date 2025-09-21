use crate::ChessApp;

use egui::RichText;
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
                        ui.checkbox(&mut self.settings.show_coordinates, "Coordinates")
                            .changed();
                        ui.add_space(20.0);
                        ui.label("Highlight :");
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.settings.show_legals_moves, "Legals moves")
                            .changed();
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.settings.show_threaten_cells, "Threaten cells");
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.settings.show_last_move, "Last move")
                            .changed();
                        ui.add_space(20.0);
                        if !self.history.is_empty() {
                            ui.text_edit_singleline(&mut self.settings.file_name);
                            if ui.button(RichText::new("Download").size(20.0)).clicked() {
                                ui.separator();
                                let _ = self.export_pgn(); //Todo : handle error 
                                self.win = None;
                            }
                        }
                        ui.separator();
                        if !self.history_san.is_empty() {
                            ui.monospace(&self.history_san);
                        }
                    });
                });
            });
    }
}
