use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::AppMode::Replay;
use crate::gui::chessapp_struct::UiType::Mobile;
use crate::gui::hooks::WinDia::*;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.win = Some(Settings);
        }
    }

    pub fn draw_endgame_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(180.0);
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Replay"))
            .clicked()
        {
            self.app_mode = Replay;
            self.current = self.history.snapshots[self.replay_infos.index - 1].clone();
        }
        ui.add_space(170.0);
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(Mobile);
        }
    }

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

    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(Mobile);
        }
    }

    pub fn lobby_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(180.0);
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Timer"))
            .clicked()
        {
            self.win = Some(Timer);
        }
        ui.add_space(180.0);
        self.new_game_button(ui);
    }

    pub fn draw_resign_undo_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(150.0);
        if ui.button("Draw").clicked() {
            self.win = Some(DrawRequest);
        }
        ui.add_space(20.0);
        if ui.button("Resign").clicked() {
            self.win = Some(Resign);
        }
        ui.add_space(150.0);
        if ui
            .add_enabled(
                self.win.is_none() && self.history.snapshots.len() > 0,
                egui::Button::new("Undo"),
            )
            .clicked()
        {
            self.win = Some(Undo);
        }
    }
}
