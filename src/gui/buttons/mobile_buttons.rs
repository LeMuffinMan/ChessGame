
use crate::ChessApp;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::WinDia::*;
use crate::gui::chessapp_struct::AppMode::Replay;

impl ChessApp {

    pub fn settings_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.mobile_win.is_none(),
                egui::Button::new("Settings"),
            )
            .clicked()
        {
            self.mobile_win = Some(Options);
        }
    }

    pub fn draw_endgame_buttons(&mut self, ui: &mut egui::Ui) {

        ui.add_space(60.0);
        self.settings_button(ui);
        ui.add_space(180.0);
        if ui.add_enabled(
            self.mobile_win.is_none(),
            egui::Button::new("Replay"),
        )
        .clicked()
        {
            self.app_mode = Replay;
            self.current = self.history[self.replay_infos.index - 1].clone();
        }
        ui.add_space(170.0);
        if ui.add_enabled(
            self.mobile_win.is_none(),
            egui::Button::new("New Game"),
        ).clicked() {
            *self = ChessApp::new(true);
        }

    }

    pub fn display_bottom_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            match &self.app_mode {
                AppMode::Replay => {
                    self.replay_buttons(ui);
                }
                AppMode::Lobby => {
                    self.lobby_buttons(ui);
                }
                AppMode::Versus(Some(_end)) => {
                    self.draw_endgame_buttons(ui);
                }
                AppMode::Versus(None) => {
                    self.draw_resign_buttons(ui);
                }
            }
        });
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
        if ui.button("<").clicked() {
            if self.replay_infos.index > 0 {
                self.replay_infos.index -= 1;
                self.current = self.history[self.replay_infos.index].clone();
            }
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
        } else {
            if ui
                .add_enabled(!self.win_dialog, egui::Button::new("⏸"))
                .clicked()
            {
                self.widgets.next_replay_time = None;
            }
        }
        ui.add_space(25.0);
        if ui
            .add_enabled(self.replay_infos.index < self.history.len() - 1, egui::Button::new(">"))
            .clicked()
        {
            if self.replay_infos.index < self.history.len() - 1 {
                self.replay_infos.index += 1;
                self.current = self.history[self.replay_infos.index].clone();
            }
        }
        ui.add_space(25.0);
        if ui
            .add_enabled(self.replay_infos.index < self.history.len() - 1, egui::Button::new(">|"))
            .clicked()
        {
            self.replay_infos.index = self.history.len() - 1;
            self.current = self.history[self.replay_infos.index].clone();
        }
        ui.add_space(50.0);
        self.new_game_button(ui);
    }

    pub fn new_game_button(&mut self, ui: &mut egui::Ui) {
        if ui.add_enabled(
            self.mobile_win.is_none(),
            egui::Button::new("New Game"),
        ).clicked() {
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

    pub fn draw_resign_buttons(&mut self, ui: &mut egui::Ui) {
        self.settings_button(ui);
        ui.add_space(400.0);
        if ui.button("Draw").clicked() {
            self.mobile_win = Some(Draw);
        }
        ui.add_space(40.0);
        if ui.button("Resign").clicked() {
            self.mobile_win = Some(Resign);
        }
    }
}

