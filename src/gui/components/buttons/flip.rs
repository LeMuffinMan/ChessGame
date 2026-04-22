use crate::ChessApp;

impl ChessApp {
    pub fn flip_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Flip board").clicked() {
                self.settings.flip = !self.settings.flip;
            }
            if ui
                .toggle_value(&mut self.settings.autoflip, "Autoflip")
                .changed()
            {}
        });
    }
}
