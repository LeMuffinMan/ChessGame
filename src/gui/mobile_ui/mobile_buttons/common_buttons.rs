use crate::ChessApp;
use crate::gui::hooks::windows::WinDia::*;
use crate::gui::layout::UiType::Mobile;
use egui::RichText;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.win = Some(Settings);
        }
    }

    pub fn new_game_button_mobile(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(Mobile);
        }
    }
    pub fn highlight_checkboxes(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(50.0);
            ui.vertical(|ui| {
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
            });
        });
    }
}
