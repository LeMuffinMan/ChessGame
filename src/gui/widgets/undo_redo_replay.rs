use crate::ChessApp;
use crate::gui::chessapp_struct::GameMode;
use crate::gui::chessapp_struct::GameMode::*;
// use crate::pgn::encode_pgn::export_pgn;

pub struct Timer {
    pub white: (Option<f64>, f64), //start of turn, remaining time
    pub black: (Option<f64>, f64),
    pub increment: f64,
}

impl Timer {
    pub fn build(game_mode: Option<GameMode>) -> Option<Self> {
        match game_mode {
            Some(Bullet(time, inc))
            | Some(Blitz(time, inc))
            | Some(Rapid(time, inc))
            | Some(Custom(time, inc)) => Some(Self {
                white: (None, time),
                black: (None, time),
                increment: inc,
            }),
            None => None,
        }
    }
}

impl ChessApp {
    pub fn undo_redo_replay(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_enabled(false, egui::Button::new("Save"));
            let can_undo = self.widgets.replay_index != 0;
            let can_redo = self.widgets.replay_index + 1 < self.history.len();
            let can_replay = can_undo && self.widgets.next_replay_time.is_none();
            self.undo(ui, can_undo);
            if can_replay {
                if ui
                    .add_enabled(!self.win_dialog, egui::Button::new("▶"))
                    .clicked()
                {
                    self.widgets.replay_index = 0;
                    self.current = self.history[0].clone();

                    let now = ui.input(|i| i.time);
                    let delay = self.widgets.replay_speed;
                    self.widgets.next_replay_time = Some(now + delay);
                }
            } else if self.widgets.next_replay_time.is_some() {
                if ui
                    .add_enabled(!self.win_dialog, egui::Button::new("⏸"))
                    .clicked()
                {
                    self.widgets.next_replay_time = None;
                }
            } else {
                ui.add_enabled(false, egui::Button::new("▶")).clicked();
            }
            self.replay_step(ui.ctx());
            self.redo(ui, can_redo);
            ui.add_enabled(false, egui::Button::new("Load"));
        });
        ui.separator();
        if self.widgets.next_replay_time.is_some() {
            ui.add(
                egui::Slider::new(&mut self.widgets.replay_speed, 0.1..=10.0)
                    .text("sec/move")
                    .logarithmic(true),
            );
        }
    }

    pub fn undo(&mut self, ui: &mut egui::Ui, can_undo: bool) {
        if ui
            .add_enabled(can_undo && !self.win_dialog, egui::Button::new("|<"))
            .clicked()
        {
            self.widgets.replay_index = 0;
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
        if ui
            .add_enabled(can_undo && !self.win_dialog, egui::Button::new("<"))
            .clicked()
        {
            if self.widgets.replay_index == self.history.len() {
                self.widgets.replay_index -= 1;
            }
            self.widgets.replay_index -= 1;
            log::info!(
                "after button < {} {}",
                self.widgets.replay_index,
                self.history.len()
            );
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
    }

    pub fn redo(&mut self, ui: &mut egui::Ui, can_redo: bool) {
        if ui.add_enabled(can_redo, egui::Button::new(">")).clicked() {
            self.widgets.replay_index += 1;
            self.current = self.history[self.widgets.replay_index].clone();
            if self.widgets.replay_index == self.history.len() - 1 {
                self.widgets.replay_index += 1;
            }
            log::info!(
                "after button > {} {}",
                self.widgets.replay_index,
                self.history.len()
            );
            self.highlight.piece_legals_moves.clear();
        }
        if ui.add_enabled(can_redo, egui::Button::new(">|")).clicked() {
            self.widgets.replay_index = self.history.len() - 1;
            self.current = self.history[self.widgets.replay_index].clone();
            self.widgets.replay_index += 1;
            self.highlight.piece_legals_moves.clear();
        }
    }

    fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.widgets.next_replay_time {
            let now = ctx.input(|i| i.time);
            if now >= next_time {
                if self.widgets.replay_index + 1 < self.history.len() {
                    self.widgets.replay_index += 1;
                    // log::debug!("Replay index = {}", self.widgets.replay_index);
                    self.current = self.history[self.widgets.replay_index].clone();
                    let delay = self.widgets.replay_speed;
                    self.widgets.next_replay_time = Some(now + delay);
                } else {
                    self.widgets.replay_index = self.history.len();
                    self.widgets.next_replay_time = None;
                }
            }
        }
        ctx.request_repaint();
    }
}
