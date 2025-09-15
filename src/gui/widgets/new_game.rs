use crate::ChessApp;

use egui::Context;

impl ChessApp {
    pub fn new_save_load(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            if self.current.end.is_some() {
                if ui.button("New game").clicked() {
                    *self = ChessApp::default();
                }
            }

            // if ui
            //     .add_enabled(!(self.undo.is_empty()), egui::Button::new("Save"))
            //     .clicked()
            // {
            //     self.file_dialog.save_file();
            //     ui.label(format!("save file: {:?}", self.file_path));
            // }
            // if let Some(path) = self.file_dialog.update(ctx).picked() {
            //     if let Some(path) = Some(path.to_path_buf()) {
            //         println!("{:?}", path);
            //     }
            //     export_pgn(&self.current.history_san, path);
            // }
            // if ui.add_enabled(false, egui::Button::new("Load")).clicked() {
            //     println!("Load game");
            // }
        });
        ui.separator();
    }
}
