use crate::gui::chessapp_struct::AppMode;

use crate::ChessApp;

impl ChessApp {
    pub fn speed_replay_slider(&mut self, ui: &mut egui::Ui) {
        if matches!(self.app_mode, AppMode::Replay) {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(385.0);
                ui.add(
                    egui::Slider::new(&mut self.replay_infos.sec_per_frame, 0.1..=5.0)
                        .text("sec/move")
                        .logarithmic(true),
                );
            });
            ui.add_space(20.0);
        } else {
            ui.add_space(40.0);
        }
    }
}
