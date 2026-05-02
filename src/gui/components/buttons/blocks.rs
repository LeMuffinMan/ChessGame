use crate::ChessApp;
use crate::Color::*;
use crate::gui::chessapp::AppMode::*;
use crate::gui::hooks::windows::WinDia;

fn btn_w(ui: &egui::Ui, text: &str) -> f32 {
    let font_id = egui::TextStyle::Button.resolve(ui.style());
    ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font_id, egui::Color32::WHITE)
            .size()
            .x
    }) + ui.spacing().button_padding.x * 2.0
}

fn centered_indent(ui: &egui::Ui, labels: &[&str], spacing: f32) -> f32 {
    let total = labels.iter().map(|l| btn_w(ui, l)).sum::<f32>()
        + spacing * (labels.len().saturating_sub(1)) as f32;
    ((ui.available_width() - total) / 2.0).max(0.0)
}

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
        let gap = 20.0;
        let indent = centered_indent(ui, &["Settings", "Timer", "New"], gap);
        ui.horizontal(|ui| {
            ui.add_space(indent);
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
        });
    }

    pub fn draw_endgame_buttons(&mut self, ui: &mut egui::Ui) {
        let gap = 16.0;
        let indent = centered_indent(ui, &["Replay", "Revenge", "New"], gap);
        ui.horizontal(|ui| {
            ui.add_space(indent);
            if ui
                .add_enabled(self.win.is_none(), egui::Button::new("Replay"))
                .clicked()
            {
                self.app_mode = Replay;
                self.game.board = self.game.board_at(self.replay_infos.index);
                self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
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
        ui.add_space(16.0);
        let indent_s = ((ui.available_width() - btn_w(ui, "Settings")) / 2.0).max(0.0);
        ui.horizontal(|ui| {
            ui.add_space(indent_s);
            self.settings_button(ui);
        });
    }

    pub fn draw_resign_undo_mobile(&mut self, ui: &mut egui::Ui) {
        let gap = 16.0;
        let has_undo = self.is_undoable();
        let labels: &[&str] = if has_undo {
            &["Settings", "Draw", "Resign", "Undo"]
        } else {
            &["Settings", "Draw", "Resign"]
        };
        let indent = centered_indent(ui, labels, gap);
        ui.horizontal(|ui| {
            ui.add_space(indent);
            self.settings_button(ui);
            ui.add_space(gap);
            self.draw_buttons(ui);
            ui.add_space(gap);
            self.resign_button(ui);
            if has_undo {
                ui.add_space(gap);
                self.undo_button(ui);
            }
        });
    }
}
