use crate::ChessApp;
// use crate::pgn::encode_pgn::export_pgn;

impl ChessApp {
    pub fn undo_redo_replay(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_enabled(false, egui::Button::new("Save"));
            let can_undo = self.widgets.replay_index != 0;
            let can_redo = self.widgets.replay_index + 1 < self.history.len();
            let can_replay = can_undo && self.widgets.next_replay_time.is_none();
            self.undo(ui, can_undo);
            if can_replay {
                if ui.button("▶").clicked()
                {
                    self.widgets.replay_index = 0;
                    self.current = self.history[0].clone();

                    let now = ui.input(|i| i.time);
                    let delay = self.widgets.replay_speed as f64 / 1000.0;
                    self.widgets.next_replay_time = Some(now + delay);
                }
            } else if self.widgets.next_replay_time.is_some() {
                if ui.button("⏸").clicked() {
                    self.widgets.next_replay_time = None;
                }
            } else {
                ui
                    .add_enabled(false, egui::Button::new("▶"))
                    .clicked();
            }
            self.replay_step(ui.ctx());
            self.redo(ui, can_redo);
            ui.add_enabled(false, egui::Button::new("Load"));
            });
        ui.separator();
        if self.widgets.next_replay_time.is_some() {
            ui.add(
                egui::Slider::new(&mut self.widgets.replay_speed, 100..=5000)
                    .text("ms/move")
                    .logarithmic(true),
            );
        }
    }

    pub fn undo(&mut self, ui: &mut egui::Ui, can_undo: bool) {
        if ui.add_enabled(can_undo, egui::Button::new("|<")).clicked()
        {
            self.widgets.replay_index = 0;
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
        if ui
            .add_enabled(can_undo, egui::Button::new("<"))
            .clicked()
        {
            if self.widgets.replay_index == self.history.len() {
                self.widgets.replay_index -= 1;
            }
            self.widgets.replay_index -= 1;
            log::info!("after button < {} {}", self.widgets.replay_index, self.history.len());
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
    }

    pub fn redo(&mut self, ui: &mut egui::Ui, can_redo: bool) {
        if ui
            .add_enabled(can_redo, egui::Button::new(">"))
            .clicked()
        {
            self.widgets.replay_index += 1;
            self.current = self.history[self.widgets.replay_index].clone();
            if self.widgets.replay_index == self.history.len() - 1 {
                self.widgets.replay_index += 1;
            }
            log::info!("after button > {} {}", self.widgets.replay_index, self.history.len());
            self.highlight.piece_legals_moves.clear();
        }
        if ui
            .add_enabled(can_redo, egui::Button::new(">|"))
            .clicked()
        {
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
                    log::debug!("Replay index = {}", self.widgets.replay_index);
                    self.current = self.history[self.widgets.replay_index].clone();
                    let delay = self.widgets.replay_speed as f64 / 1000.0;
                    self.widgets.next_replay_time = Some(now + delay);               
                } else {
                    self.widgets.next_replay_time = None;
                }
            }
        }
        ctx.request_repaint();
    }
}
