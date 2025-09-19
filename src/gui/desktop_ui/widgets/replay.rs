use crate::ChessApp;
use crate::gui::chessapp_struct::WinDia::*;

// use crate::pgn::encode_pgn::export_pgn;


impl ChessApp {
    //This panel holds the replay buttons
    pub fn replay_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            self.save_button(ui);
            let can_undo = self.replay_infos.index != 0;
            let can_redo = self.replay_infos.index + 1 < self.history.len();
            let can_replay = can_undo && self.replay_infos.next_step.is_none();
            self.play_pause_button(ui, can_replay);
            self.rewind(ui, can_undo);
            // self.replay_step(ui.ctx());
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
            self.mobile_win = Some(Pgn);
        }
    }

    pub fn replay_time_slider(&mut self, ui: &mut egui::Ui) {
        if self.replay_infos.next_step.is_some() {
            ui.add(
                egui::Slider::new(&mut self.replay_infos.sec_per_frame, 0.1..=10.0)
                    .text("sec/move")
                    .logarithmic(true),
            );
        }
    }

    pub fn play_pause_button(&mut self, ui: &mut egui::Ui, can_replay: bool) {
        if can_replay {
            if ui
                .add_enabled(self.mobile_win.is_some(), egui::Button::new("▶"))
                .clicked()
            {
                self.replay_infos.index = 0;
                self.current = self.history[0].clone();

                let now = ui.input(|i| i.time);
                let delay = self.replay_infos.sec_per_frame;
                self.replay_infos.next_step = Some(now + delay);
            }
        } else if self.replay_infos.next_step.is_some() {
            if ui
                .add_enabled(self.mobile_win.is_some(), egui::Button::new("⏸"))
                .clicked()
            {
                self.replay_infos.next_step = None;
            }
        } else {
            ui.add_enabled(false, egui::Button::new("▶")).clicked();
        }
    }

    pub fn rewind(&mut self, ui: &mut egui::Ui, can_undo: bool) {
        if ui
            .add_enabled(can_undo && self.mobile_win.is_some(), egui::Button::new("|<"))
            .clicked()
        {
            self.replay_infos.index = 0;
            self.current = self.history[self.replay_infos.index].clone();
            self.highlight.piece_legals_moves.clear();
        }
        if ui
            .add_enabled(can_undo && self.mobile_win.is_some(), egui::Button::new("<"))
            .clicked()
        {
            if self.replay_infos.index == self.history.len() {
                self.replay_infos.index -= 1;
            }
            self.replay_infos.index -= 1;
            self.current = self.history[self.replay_infos.index].clone();
            self.highlight.piece_legals_moves.clear();
        }
    }

    pub fn redo(&mut self, ui: &mut egui::Ui, can_redo: bool) {
        if ui.add_enabled(can_redo, egui::Button::new(">")).clicked() {
            self.replay_infos.index += 1;
            self.current = self.history[self.replay_infos.index].clone();
            if self.replay_infos.index == self.history.len() - 1 {
                self.replay_infos.index += 1;
            }
            self.highlight.piece_legals_moves.clear();
        }
        if ui.add_enabled(can_redo, egui::Button::new(">|")).clicked() {
            self.replay_infos.index = self.history.len() - 1;
            self.current = self.history[self.replay_infos.index].clone();
            self.replay_infos.index += 1;
            self.highlight.piece_legals_moves.clear();
        }
    }
}
