use crate::ChessApp;
use crate::Color::*;
use crate::gui::chessapp::AppMode::*;
use crate::gui::hooks::windows::WinDia;

impl ChessApp {
    // Desktop

    pub fn new_game_replay(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.game.end.is_some() || self.app_mode == Replay {
                self.new_game_button(ui);
                self.revenge_button(ui);
                self.replay_button(ui);
                ui.separator();
            }
        });
    }

    pub fn draw_resign_undo_desktop(&mut self, ui: &mut egui::Ui) {
        self.draw_buttons(ui);
        ui.separator();
        ui.horizontal(|ui| {
            self.resign_button(ui);
            if self.is_undoable() {
                self.undo_button(ui);
            }
        });
        ui.separator();
    }

    // Mobile

    pub fn lobby_buttons(&mut self, ui: &mut egui::Ui) {
        let gap = ((ui.available_width() - 350.0) / 4.0).max(8.0);
        ui.add_space(gap);
        self.settings_button(ui);
        ui.add_space(gap);
        if ui
            .add_enabled(self.win.is_none(), egui::Button::new("Timer"))
            .clicked()
        {
            self.win = Some(WinDia::Timer);
        }
        ui.add_space(gap);
        self.new_game_button(ui);
    }

    pub fn draw_endgame_buttons(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.horizontal_centered(|ui| {
                let gap = ((ui.available_width() - 460.0) / 4.0).max(8.0);
                ui.add_space(gap);
                if ui
                    .add_enabled(self.win.is_none(), egui::Button::new("Replay"))
                    .clicked()
                {
                    self.app_mode = Replay;
                    self.game.board = self.game.board_at(self.replay_infos.index);
                    self.game.active_player = if self.replay_infos.index % 2 == 0 {
                        White
                    } else {
                        Black
                    };
                }
                ui.add_space(gap);
                self.revenge_button(ui);
                ui.add_space(gap);
                self.new_game_button(ui);
            });
            ui.add_space(20.0);
            ui.horizontal_centered(|ui| {
                self.settings_button(ui);
            });
        });
    }

    pub fn draw_resign_undo_mobile(&mut self, ui: &mut egui::Ui) {
        let gap = ((ui.available_width() - 350.0) / 4.0).max(8.0);
        ui.add_space(gap);
        self.settings_button(ui);
        ui.add_space(gap);
        self.draw_buttons(ui);
        ui.add_space(gap);
        self.resign_button(ui);
        if self.is_undoable() {
            ui.add_space(gap);
            self.undo_button(ui);
        }
    }
}
