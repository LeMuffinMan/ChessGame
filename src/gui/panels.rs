use crate::ChessApp;

use crate::Color;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::render::{centered_square, draw_border};
use crate::mat_or_pat;

use eframe::egui;
use egui::Context;
use std::time::{Duration, Instant};
use chrono::Utc;
use std::fs;
use std::path::Path;

fn export_pgn(san: &String, path: &Path) {
    let mut pgn = String::new();
    pgn.push_str("[Event \"ChessGame\"]\n[Site \"ChessGame\"]\n[Date \"");
    pgn.push_str(Utc::now().to_string().as_str());
    pgn.push_str("\"]\n[White \"White_player\"]\n[Black \"Black_player\"]\n");
    if let Some(result) = san.split_whitespace().last() {
        pgn.push_str("[Result : \"");
        if result == "0-1" || result == "1-0" || result == "1/2 - 1/2" {
            pgn.push_str(result);
        } else {
            pgn.push('*');
        }
        pgn.push_str("\"]\n\n");
        pgn.push_str(san);
        pgn.push('\n');
        match fs::write(path, &pgn) {
            Ok(_) => println!("File saved with success"),
            Err(e) => eprintln!("Error saving file : {}", e),
        }
        println!("{}", pgn);
    }
}

impl ChessApp {
    fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.next_replay_time {
            if Instant::now() >= next_time {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(self.current.clone());
                    self.current = next;
                    self.next_replay_time =
                        Some(Instant::now() + Duration::from_millis(self.replay_speed));
                    ctx.request_repaint();
                } else {
                    self.next_replay_time = None;
                }
            } else {
                ctx.request_repaint();
            }
        }
    }

    pub fn side_panel_ui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.heading("ChessGame");
        if let None = self.current.board.pawn_to_promote {
            self.side_panel_top(ui, ctx);
            ui.separator();
            self.side_panel_flip(ui);
            ui.separator();
            ui.checkbox(&mut self.show_coordinates, "Coordinates")
                .changed();
            ui.label("Highlight :");
            ui.checkbox(&mut self.show_legals_moves, "Legals moves")
                .changed();
            ui.checkbox(&mut self.show_threaten_cells, "Threaten cells");
            ui.checkbox(&mut self.show_last_move, "Last move").changed();
            ui.separator();
            self.side_panel_undo_redo_replay(ui);
            // ui.monospace(&self.current.last_move_san);
            if !self.current.history_san.is_empty() {
                ui.monospace(&self.current.history_san);
            }
            ui.separator();
            if ui.button("Quit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        self.side_panel_promote(ui);
    }

    pub fn central_panel_ui(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        let board_rect = centered_square(rect);
        let inner = if self.show_coordinates {
            draw_border(&painter, board_rect);
            board_rect.shrink(16.0)
        } else {
            board_rect
        };

        let sq = inner.width() / 8.0;

        if self.show_coordinates {
            self.show_coordinates(&painter, inner, sq);
        }
        self.draw_board(&painter, inner, sq);
        self.draw_pieces(&painter, inner, sq);
        self.draw_dragged_piece(&painter, inner,);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }

    pub fn side_panel_top(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.separator();
        ui.label(format!("Turn #{}", self.current.turn));
        if self.current.checkmate {
            let color = if self.current.active_player == Color::White {
                Color::Black
            } else {
                Color::White
            };
            ui.label(format!("Checkmate ! {:?} win", color));
        } else if self.current.pat {
            ui.label(format!("Pat !"));
        } else if self.current.board.check.is_some() {
            ui.label("Check !");
        } else {
            ui.label(format!("{:?} to move", self.current.active_player));
        }
        // if self.current.last_move.is_some() {
        //     ui.label(format!("last move: {}", self.current.last_move_san));
        // }
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("New game").clicked() {
                *self = ChessApp::default();
            }

            if ui.add_enabled(!(self.undo.is_empty()), egui::Button::new("Save")).clicked() {
                self.file_dialog.save_file();
                ui.label(format!("save file: {:?}", self.file_path));
            }
            if let Some(path) = self.file_dialog.update(ctx).picked() {
                if let Some(path) = Some(path.to_path_buf()) {

                    println!("{:?}", path);
                }
                export_pgn(&self.current.history_san, path);
            }
            if ui.add_enabled(false, egui::Button::new("Load")).clicked() {
                println!("Load game");
            }
        });
    }

    pub fn side_panel_promote(&mut self, ui: &mut egui::Ui) {
        if let Some(coord) = self.current.board.pawn_to_promote {
            if let Some(piece) = self.current.board.promote {
                let color = if self.current.active_player == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                self.current.board.grid[coord.row as usize][coord.col as usize] =
                    Cell::Occupied(piece, color);

                // println!("{:?} to move", self.current.active_player);
                if let Some(k) = self.current.board.get_king(&self.current.active_player) {
                    if self.current.board.threaten_cells.contains(&k) {
                        self.current.board.check = Some(k);
                        // println!("Check !");
                    }
                }
                let (end, mate) = mat_or_pat(&mut self.current.board, &self.current.active_player);
                if end {
                    if mate {
                        self.current.checkmate = true;
                    } else {
                        self.current.pat = true;
                    }
                }
                if let Some(promoteinfo) = &self.promoteinfo {
                    let from = promoteinfo.from;
                    let to = promoteinfo.to;
                    let prev_board = promoteinfo.prev_board.clone();
                    self.from_move_to_san(&from, &to, &prev_board);

                }


                self.current.board.pawn_to_promote = None;
                self.current.board.promote = None;
            } else {
                ui.separator();
                ui.label("Promote pawn to : ");
                ui.vertical(|ui| {
                    ui.radio_value(&mut self.current.board.promote, Some(Queen), "Queen");
                    ui.radio_value(&mut self.current.board.promote, Some(Bishop), "Bishop");
                    ui.radio_value(&mut self.current.board.promote, Some(Knight), "Knight");
                    ui.radio_value(&mut self.current.board.promote, Some(Rook), "Rook");
                });
                ui.separator();
            }
        }
    }

    pub fn side_panel_flip(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Flip board").clicked() {
                self.flip = !self.flip;
            }
            if ui.toggle_value(&mut self.autoflip, "Autoflip").changed() {}
        });
    }

    pub fn side_panel_undo_redo_replay(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let can_undo = !self.undo.is_empty();
            let can_redo = !self.redo.is_empty();
            if ui
                .add_enabled(can_undo, egui::Button::new("Undo"))
                .clicked()
            {
                if let Some(prev) = self.undo.pop() {
                    self.redo.push(self.current.clone());
                    self.current = prev;
                    self.piece_legals_moves.clear();
                }
            }
            if ui
                .add_enabled(can_redo, egui::Button::new("Redo"))
                .clicked()
            {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(self.current.clone());
                    self.current = next;
                }
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
                self.next_replay_time =
                    Some(Instant::now() + Duration::from_millis(self.replay_speed));
            }
            self.replay_step(ui.ctx());
        });
        ui.separator();
        if let Some(_) = self.next_replay_time {
            ui.add(
                egui::Slider::new(&mut self.replay_speed, 100..=2000)
                    .text("ms/move")
                    .logarithmic(true),
            );
        }
    }
}
