use crate::ChessApp;
use crate::board::cell::Color::{Black, White};
use crate::gui::layout::UiType;

pub struct ReplayInfos {
    pub index: usize,
    pub sec_per_frame: f64,
    pub next_step: Option<f64>,
}

impl ReplayInfos {
    pub fn new() -> Self {
        Self {
            index: 0,
            sec_per_frame: 1.0,
            next_step: None,
        }
    }
}

impl Default for ReplayInfos {
    fn default() -> Self {
        Self::new()
    }
}

impl ChessApp {
    pub fn replay_buttons(&mut self, ui: &mut egui::Ui) {
        match &self.ui_type {
            UiType::Mobile => {
                let gap1 = 20.0;
                let indent1 = {
                    let font_id = egui::TextStyle::Button.resolve(ui.style());
                    let tw = |t: &str| {
                        ui.fonts(|f| {
                            f.layout_no_wrap(t.to_owned(), font_id.clone(), egui::Color32::WHITE)
                                .size()
                                .x
                        }) + ui.spacing().button_padding.x * 2.0
                    };
                    let total = tw("Settings") + tw("New") + gap1;
                    ((ui.available_width() - total) / 2.0).max(0.0)
                };
                ui.horizontal(|ui| {
                    ui.add_space(indent1);
                    self.settings_button(ui);
                    ui.add_space(gap1);
                    self.new_game_button(ui);
                });
                ui.add_space(8.0);
                let gap2 = 8.0;
                let indent2 = {
                    let font_id = egui::TextStyle::Button.resolve(ui.style());
                    let tw = |t: &str| {
                        ui.fonts(|f| {
                            f.layout_no_wrap(t.to_owned(), font_id.clone(), egui::Color32::WHITE)
                                .size()
                                .x
                        }) + ui.spacing().button_padding.x * 2.0
                    };
                    let total = tw("|<") + tw("<") + tw("▶") + tw(">") + tw(">|") + gap2 * 4.0;
                    ((ui.available_width() - total) / 2.0).max(0.0)
                };
                ui.horizontal(|ui| {
                    ui.add_space(indent2);
                    self.first_state(ui);
                    ui.add_space(gap2);
                    self.prev_state(ui);
                    ui.add_space(gap2);
                    self.play_pause(ui);
                    ui.add_space(gap2);
                    self.next_state(ui);
                    ui.add_space(gap2);
                    self.last_state(ui);
                });
            }
            UiType::Desktop => {
                ui.horizontal(|ui| {
                    self.first_state(ui);
                    ui.add_space(10.0);
                    self.prev_state(ui);
                    ui.add_space(10.0);
                    self.play_pause(ui);
                    ui.add_space(10.0);
                    self.next_state(ui);
                    ui.add_space(10.0);
                    self.last_state(ui);
                });
            }
        }
    }

    fn first_state(&mut self, ui: &mut egui::Ui) {
        if ui.button("|<").clicked() {
            self.replay_infos.index = 0;
            self.game.board = self.game.board_at(0);
            self.game.active_player = White;
        }
    }

    fn prev_state(&mut self, ui: &mut egui::Ui) {
        if ui.button("<").clicked() && self.replay_infos.index > 0 {
            self.replay_infos.index -= 1;
            self.game.board = self.game.board_at(self.replay_infos.index);
            self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
                White
            } else {
                Black
            };
        }
    }

    fn play_pause(&mut self, ui: &mut egui::Ui) {
        if self.replay_infos.next_step.is_none() {
            if ui
                .add_enabled(self.win.is_none(), egui::Button::new("▶"))
                .clicked()
            {
                self.replay_infos.index = 0;
                self.game.board = self.game.board_at(0);
                self.game.active_player = White;

                let now = ui.input(|i| i.time);
                let delay = self.replay_infos.sec_per_frame;
                self.replay_infos.next_step = Some(now + delay);
            }
        } else if ui
            .add_enabled(self.win.is_none(), egui::Button::new("⏸"))
            .clicked()
        {
            self.replay_infos.next_step = None;
        }
    }

    fn next_state(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.replay_infos.index < self.game.history.len() - 1,
                egui::Button::new(">"),
            )
            .clicked()
            && self.replay_infos.index < self.game.history.len() - 1
        {
            self.replay_infos.index += 1;
            self.game.board = self.game.board_at(self.replay_infos.index);
            self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
                White
            } else {
                Black
            };
        }
    }

    fn last_state(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.replay_infos.index < self.game.history.len() - 1,
                egui::Button::new(">|"),
            )
            .clicked()
        {
            self.replay_infos.index = self.game.history.len() - 1;
            self.game.board = self.game.board_at(self.replay_infos.index);
            self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
                White
            } else {
                Black
            };
        }
    }

    pub fn speed_replay_slider(&mut self, ui: &mut egui::Ui) {
        if matches!(self.app_mode, crate::gui::chessapp::AppMode::Replay) {
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

    pub fn mobile_replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_step) = self.replay_infos.next_step {
            let now = ctx.input(|i| i.time);
            if now >= next_step {
                if self.replay_infos.index + 1 < self.game.history.len() {
                    self.replay_infos.index += 1;
                    self.game.board = self.game.board_at(self.replay_infos.index);
                    self.game.active_player = if self.replay_infos.index.is_multiple_of(2) {
                        White
                    } else {
                        Black
                    };
                    let delay = self.replay_infos.sec_per_frame;
                    self.replay_infos.next_step = Some(now + delay);
                } else {
                    self.replay_infos.index = self.game.history.len();
                    self.replay_infos.next_step = None;
                }
            }
        }
        ctx.request_repaint();
    }
}
