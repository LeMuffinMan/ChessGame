use crate::ChessApp;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::DrawRule::*;
use crate::gui::chessapp_struct::End::*;
// use crate::pgn::encode_pgn::export_pgn;

impl ChessApp {
    pub fn turn_infos(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Turn #{}", self.current.turn));
        if let Some(end) = &self.current.end {
            match end {
                Checkmate => ui.label(format!("Checkmate ! {:?} win", self.current.opponent)),
                Pat => ui.label("Pat !"),
                Draw => ui.label("Draw"),
                Resign => ui.label(format!(
                    "{:?} resigned : {:?} win",
                    self.current.active_player, self.current.opponent
                )),
            };
        } else {
            ui.horizontal(|ui| {
                if self.current.board.check.is_some() {
                    ui.label("Check !");
                }
                ui.label(format!("{:?} to move", self.current.active_player));
            });
        }
    }
    pub fn draw_resign(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(draw) = &self.draw.draw_option {
                match draw {
                    Available(TripleRepetition) => {
                        ui.label("Triple repetition :");
                    }
                    Available(FiftyMoves) => {
                        ui.label("%50 moves : ");
                    }
                    _ => {}
                };
                if ui.button("Claim draw").clicked() {
                    self.current.end = Some(Draw);
                    self.draw.draw_option = None;
                }
            } else if ui
                .add_enabled(self.current.end.is_none(), egui::Button::new("Draw"))
                .clicked()
            {
                self.draw.draw_option = Some(Request);
            }
            if ui
                .add_enabled(self.current.end.is_none(), egui::Button::new("Resign"))
                .clicked()
            {
                self.current.end = Some(Resign);
            }
        });
    }

    // pub fn new_save_load(&mut self, ui: &mut egui::Ui, ctx: &Context) {
    //     ui.horizontal(|ui| {
    //         if ui.button("New game").clicked() {
    //             *self = ChessApp::default();
    //         }
    //
    //         if ui
    //             .add_enabled(!(self.undo.is_empty()), egui::Button::new("Save"))
    //             .clicked()
    //         {
    //             self.file_dialog.save_file();
    //             ui.label(format!("save file: {:?}", self.file_path));
    //         }
    //         if let Some(path) = self.file_dialog.update(ctx).picked() {
    //             if let Some(path) = Some(path.to_path_buf()) {
    //                 println!("{:?}", path);
    //             }
    //             export_pgn(&self.current.history_san, path);
    //         }
    //         if ui.add_enabled(false, egui::Button::new("Load")).clicked() {
    //             println!("Load game");
    //         }
    //     });
    // }

    pub fn side_panel_flip(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Flip board").clicked() {
                self.widgets.flip = !self.widgets.flip;
            }
            if ui
                .toggle_value(&mut self.widgets.autoflip, "Autoflip")
                .changed()
            {}
        });
    }

    pub fn side_panel_undo_redo_replay(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let can_undo = self.widgets.replay_index != 0;
            let can_redo = self.widgets.replay_index + 1 < self.history.len();
            let can_replay = can_undo && self.widgets.next_replay_time.is_none();
            if ui
                .add_enabled(can_undo, egui::Button::new("|<"))
                .clicked()
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
                if self.widgets.replay_index == self.history.len() - 1 {
                    self.widgets.replay_index += 1;
                }
                self.highlight.piece_legals_moves.clear();
            }
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
