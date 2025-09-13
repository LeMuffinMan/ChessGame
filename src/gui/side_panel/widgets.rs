use crate::ChessApp;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::DrawRule::*;
use crate::gui::chessapp_struct::End::*;
use crate::pgn::encode_pgn::export_pgn;
use egui::Context;
use std::time::{Duration, Instant};

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
            let can_undo = !self.undo.is_empty();
            let can_redo = !self.redo.is_empty();
            if ui
                .add_enabled(can_undo, egui::Button::new("Undo"))
                .clicked()
                && let Some(prev) = self.undo.pop()
            {
                self.redo.push(self.current.clone());
                self.current = prev;
                self.highlight.piece_legals_moves.clear();
            }
            if ui
                .add_enabled(can_redo, egui::Button::new("Redo"))
                .clicked()
                && let Some(next) = self.redo.pop()
            {
                self.undo.push(self.current.clone());
                self.current = next;
            }
            if ui
                .add_enabled(!self.undo.is_empty(), egui::Button::new("Replay"))
                .clicked()
            {
                self.redo.clear();
                self.redo.push(self.current.clone());
                while let Some(prev) = self.undo.pop() {
                    self.redo.push(prev.clone());
                    if self.undo.is_empty() {
                        self.current = prev;
                    }
                }
                self.widgets.next_replay_time =
                    Some(Instant::now() + Duration::from_millis(self.widgets.replay_speed));
            }
            self.replay_step(ui.ctx());
        });
        ui.separator();
        if self.widgets.next_replay_time.is_some() {
            ui.add(
                egui::Slider::new(&mut self.widgets.replay_speed, 100..=2000)
                    .text("ms/move")
                    .logarithmic(true),
            );
        }
    }
    fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.widgets.next_replay_time {
            if Instant::now() >= next_time {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(self.current.clone());
                    self.current = next;
                    self.widgets.next_replay_time =
                        Some(Instant::now() + Duration::from_millis(self.widgets.replay_speed));
                    ctx.request_repaint();
                } else {
                    self.widgets.next_replay_time = None;
                }
            } else {
                ctx.request_repaint();
            }
        }
    }
}
