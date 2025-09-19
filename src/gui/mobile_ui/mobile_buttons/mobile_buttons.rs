use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::AppMode::Replay;
use crate::gui::hooks::WinDia::*;

impl ChessApp {
    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("Settings"))
            .clicked()
        {
            self.mobile_win = Some(Settings);
        }
    }

    pub fn draw_endgame_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(180.0);
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("Replay"))
            .clicked()
        {
            self.app_mode = Replay;
            self.current = self.history[self.replay_infos.index - 1].clone();
        }
        ui.add_space(170.0);
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(true);
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

    pub fn replay_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(50.0);
        if ui.button("|<").clicked() {
            self.replay_infos.index = 0;
            self.current = self.history[0].clone();
        }
        ui.add_space(25.0);
        if ui.button("<").clicked() && self.replay_infos.index > 0 {
            self.replay_infos.index -= 1;
            self.current = self.history[self.replay_infos.index].clone();
        }
        ui.add_space(25.0);
        if self.replay_infos.next_step.is_none() {
            if ui
                .add_enabled(self.mobile_win.is_none(), egui::Button::new("▶"))
                .clicked()
            {
                self.replay_infos.index = 0;
                self.current = self.history[0].clone();

                let now = ui.input(|i| i.time);
                let delay = self.replay_infos.sec_per_frame;
                self.replay_infos.next_step = Some(now + delay);
            }
        } else if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("⏸"))
            .clicked()
        {
            self.replay_infos.next_step = None;
        }
        ui.add_space(25.0);
        if ui
            .add_enabled(
                self.replay_infos.index < self.history.len() - 1,
                egui::Button::new(">"),
            )
            .clicked()
            && self.replay_infos.index < self.history.len() - 1
        {
            self.replay_infos.index += 1;
            self.current = self.history[self.replay_infos.index].clone();
        }
        ui.add_space(25.0);
        if ui
            .add_enabled(
                self.replay_infos.index < self.history.len() - 1,
                egui::Button::new(">|"),
            )
            .clicked()
        {
            self.replay_infos.index = self.history.len() - 1;
            self.current = self.history[self.replay_infos.index].clone();
        }
        ui.add_space(50.0);
        self.new_game_button(ui);
    }

    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("New Game"))
            .clicked()
        {
            *self = ChessApp::new(true);
        }
    }

    pub fn lobby_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(180.0);
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("Timer"))
            .clicked()
        {
            self.mobile_win = Some(Timer);
        }
        ui.add_space(180.0);
        self.new_game_button(ui);
    }

    pub fn draw_resign_undo_buttons(&mut self, ui: &mut egui::Ui) {
        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(150.0);
        if ui.button("Draw").clicked() {
            self.mobile_win = Some(DrawRequest);
        }
        ui.add_space(20.0);
        if ui.button("Resign").clicked() {
            self.mobile_win = Some(Resign);
        }
        ui.add_space(150.0);
        if ui
            .add_enabled(self.mobile_win.is_none(), egui::Button::new("Undo"))
            .clicked()
        {
            self.mobile_win = Some(Undo);
        }
    }
}
