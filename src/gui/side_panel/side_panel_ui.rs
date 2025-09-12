use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::End::*;
use crate::gui::chessapp_struct::DrawOption::*;
use egui::Context;

impl ChessApp {
    pub fn side_panel_ui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.heading("ChessGame");
        if self.current.board.pawn_to_promote.is_some() {
            self.side_panel_promote(ui);
        } else if let Some(draw) = &self.draw_option && *draw == Request {
            self.side_panel_draw_request(ui);
        } else {
            ui.separator();
            self.turn_infos(ui);
            ui.separator();
            self.draw_resign(ui);
            ui.separator();
            self.new_save_load(ui, ctx);
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
    }
    
    fn side_panel_draw_request(&mut self, ui: &mut egui::Ui) {
        ui.label("Accept draw offer ?");
        ui.horizontal(|ui| {
            if ui.button("Accept").clicked() {
                self.current.end = Some(Draw);
                self.draw_option = None;
            }
            if ui.button("Reject").clicked() {
                self.draw_option = None;
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
                if let Some(k) = self.current.board.get_king(&opponent) {
                    if self.current.board.threaten_cells.contains(&k) {
                        if let Some(k) = self.current.board.get_king(&opponent) {
                            self.current.board.check = Some(k);
                        }
                        // println!("Check !");
                    }
                }
                self.check_endgame();
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
}
