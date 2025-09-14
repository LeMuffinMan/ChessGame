use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::chessapp_struct::Timer;

use egui::Context;

impl ChessApp {

    pub fn right_panel_ui(&mut self, _ui: &mut egui::Ui, _ctx: &Context) {
    }
    pub fn side_panel_ui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if self.current.board.pawn_to_promote.is_some() {
            self.side_panel_promote(ui);
        } else if let Some(draw) = &self.draw.draw_option
            && *draw == Request
        {
            self.side_panel_draw_request(ui);
        } else {
            self.turn_infos(ui);
            ui.separator();
            self.draw_resign(ui);
            ui.separator();
            self.new_save_load(ui, ctx);
            ui.separator();
            // self.side_panel_flip(ui);
            // ui.separator();
            ui.checkbox(&mut self.widgets.show_coordinates, "Coordinates")
                .changed();
            ui.label("Highlight :");
            ui.checkbox(&mut self.widgets.show_legals_moves, "Legals moves")
                .changed();
            ui.checkbox(&mut self.widgets.show_threaten_cells, "Threaten cells");
            ui.checkbox(&mut self.widgets.show_last_move, "Last move")
                .changed();
            ui.horizontal(|ui| {
                ui.label("Timer :");
                if self.widgets.timer.is_none() {
                    if ui.add_enabled(self.history.is_empty(), egui::Button::new("OFF")).clicked() {
                        let timer: Timer = Default::default();
                        self.widgets.timer = Some(timer);
                    }
                } else {
                    //Si on est pas en cour de game
                    if let Some(timer) = &mut self.widgets.timer {
                        if timer.white.0.is_none() && timer.black.0.is_none() {
                            let white_remaining_time: &mut f64 = &mut timer.white.1;
                            let black_remaining_time: &mut f64 = &mut timer.black.1;

                            let mut remove_timer = false;

                            ui.menu_button(
                                format!(
                                    "{:.0}:{:02.0}",
                                    (*white_remaining_time / 60.0).floor(),
                                    (*white_remaining_time % 60.0).floor(),
                                ),
                                |ui| {
                                    if ui.button("OFF").clicked() {
                                        remove_timer = true;
                                    }
                                    if ui.button("1 min").clicked() {
                                        *white_remaining_time = 60.0;
                                        *black_remaining_time = 60.0;
                                    }
                                    if ui.button("3 min").clicked() {
                                        *white_remaining_time = 180.0;
                                        *black_remaining_time = 180.0;
                                    }
                                    if ui.button("5 min").clicked() {
                                        *white_remaining_time = 300.0;
                                        *black_remaining_time = 300.0;
                                    }
                                    if ui.button("10 min").clicked() {
                                        *white_remaining_time = 600.0;
                                        *black_remaining_time = 600.0;
                                    }
                                    if ui.button("15 min").clicked() {
                                        *white_remaining_time = 900.0;
                                        *black_remaining_time = 900.0;
                                    }
                                    if ui.button("30 min").clicked() {
                                        *white_remaining_time = 1800.0;
                                        *black_remaining_time = 1800.0;
                                    }
                                },
                            );
                            //used flag cause of double borrow case
                            if remove_timer {
                                self.widgets.timer = None;
                            }
                        }
                    }               
                } 
            });
            ui.separator();
            self.side_panel_undo_redo_replay(ui);
            if !self.current.history_san.is_empty() {
                ui.monospace(&self.current.history_san);
            }
            // ui.separator();
            // if ui.button("Quit").clicked() {
            //     ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            // }
        }
    }

    fn side_panel_draw_request(&mut self, ui: &mut egui::Ui) {
        ui.label("Accept draw offer ?");
        ui.horizontal(|ui| {
            if ui.button("Accept").clicked() {
                self.current.end = Some(Draw);
                self.draw.draw_option = None;
            }
            if ui.button("Reject").clicked() {
                self.draw.draw_option = None;
            }
        });
    }

    fn side_panel_promote(&mut self, ui: &mut egui::Ui) {
        if let Some(coord) = self.current.board.pawn_to_promote {
            if let Some(piece) = self.current.board.promote {
                let color = if self.current.active_player == Color::White {
                    Black
                } else {
                    White
                };
                self.current.board.grid[coord.row as usize][coord.col as usize] =
                    Cell::Occupied(piece, color);

                let opponent = if self.current.active_player != White {
                    White
                } else {
                    Black
                };
                if let Some(k) = self.current.board.get_king(&opponent)
                    && self.current.board.threaten_cells.contains(&k)
                    && let Some(k) = self.current.board.get_king(&opponent)
                {
                    self.current.board.check = Some(k);
                    // println!("Check !");
                }
                self.check_endgame();
                if let Some(promoteinfo) = &self.promoteinfo {
                    let from = promoteinfo.from;
                    let to = promoteinfo.to;
                    let prev_board = promoteinfo.prev_board.clone();
                    self.history.push(self.current.clone());
                    self.widgets.replay_index += 1;
                    self.encode_move_to_san(&from, &to, &prev_board);
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
}
