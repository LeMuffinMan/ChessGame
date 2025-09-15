use crate::ChessApp;

impl ChessApp {
    pub fn side_panel_flip(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Flip board").clicked() {
                self.widgets.flip = !self.widgets.flip;
            }
            if ui
                .toggle_value(&mut self.widgets.autoflip, "Autoflip")
                .changed()
            {}
        });
    }
}
