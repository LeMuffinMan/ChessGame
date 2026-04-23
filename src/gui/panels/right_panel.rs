use crate::ChessApp;
use egui::Context;
use egui::RichText;

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
                        self.flip_buttons(ui);
                        ui.add_space(20.0);
                        self.highlight_checkboxes(ui);
                        ui.add_space(20.0);
                        if !self.history.snapshots.is_empty() {
                            ui.text_edit_singleline(&mut self.settings.file_name);
                            if ui.button(RichText::new("Download").size(20.0)).clicked() {
                                ui.separator();
                                #[cfg(target_arch = "wasm32")]
                                let _ = self.export_pgn(); //Todo : handle error
                                self.win = None;
                            }
                        }
                        ui.separator();
                        if !self.history.history_san.is_empty() {
                            ui.monospace(&self.history.history_san);
                        }
                    });
                });
            });
    }
}
