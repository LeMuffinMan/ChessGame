use crate::ChessApp;
use egui::RichText;

impl ChessApp {
    pub fn highlight_checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(
            &mut self.settings.show_coordinates,
            RichText::new("Coordinates").size(30.0),
        );
        ui.add_space(20.0);
        ui.checkbox(
            &mut self.settings.show_legals_moves,
            RichText::new("Legals moves").size(30.0),
        );
        ui.add_space(20.0);
        ui.checkbox(
            &mut self.settings.show_threaten_cells,
            RichText::new("Threaten cells").size(30.0),
        );
        ui.add_space(20.0);
        ui.checkbox(
            &mut self.settings.show_last_move,
            RichText::new("Last move").size(30.0),
        );
    }
}
