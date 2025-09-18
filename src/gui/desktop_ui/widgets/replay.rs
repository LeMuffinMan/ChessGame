use crate::ChessApp;
use crate::gui::chessapp_struct::GameMode;
use crate::gui::chessapp_struct::GameMode::*;

// use crate::pgn::encode_pgn::export_pgn;

pub struct Timer {
    pub white: (Option<f64>, f64), //start of turn, remaining time
    pub black: (Option<f64>, f64),
    pub increment: f64,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            white: (None, 600.0),
            black: (None, 600.0),
            increment: 0.0,
        }
    }
}

impl Timer {
    //build a timer folowwing the Gamemode provided or a default one
    pub fn build(game_mode: GameMode) -> Option<Self> {
        match game_mode {
            Bullet(time, inc)
            |Blitz(time, inc)
            |Rapid(time, inc)
            |Custom(time, inc) => Some(Self {
                white: (None, time),
                black: (None, time),
                increment: inc,
            }),
            NoTime(_, _) 
            | Replay (_, _) => None,
        }
    }
}

impl ChessApp {
    //This panel holds the replay buttons
    pub fn replay_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.save_button(ui);
            let can_undo = self.widgets.replay_index != 0;
            let can_redo = self.widgets.replay_index + 1 < self.history.len();
            let can_replay = can_undo && self.widgets.next_replay_time.is_none();
            self.play_pause_button(ui, can_replay);
            self.rewind(ui, can_undo);
            self.replay_step(ui.ctx());
            self.redo(ui, can_redo);
            ui.add_enabled(false, egui::Button::new("Load"));
        });
        ui.separator();
        self.replay_time_slider(ui);
    }

    //Replay buttons following :

    pub fn save_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(!self.history.is_empty(), egui::Button::new("Save"))
            .clicked()
        {
            self.win_save = true;
        }
    }

    pub fn replay_time_slider(&mut self, ui: &mut egui::Ui) {
        if self.widgets.next_replay_time.is_some() {
            ui.add(
                egui::Slider::new(&mut self.widgets.replay_speed, 0.1..=10.0)
                    .text("sec/move")
                    .logarithmic(true),
            );
        }
    }

    pub fn play_pause_button(&mut self, ui: &mut egui::Ui, can_replay: bool) {
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
    }

    pub fn rewind(&mut self, ui: &mut egui::Ui, can_undo: bool) {
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
            self.highlight.piece_legals_moves.clear();
        }
        if ui.add_enabled(can_redo, egui::Button::new(">|")).clicked() {
            self.widgets.replay_index = self.history.len() - 1;
            self.current = self.history[self.widgets.replay_index].clone();
            self.widgets.replay_index += 1;
            self.highlight.piece_legals_moves.clear();
        }
    }
}
