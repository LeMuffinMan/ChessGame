use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::End::*;

use egui::Context;

impl ChessApp {
    pub fn left_panel_ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
            if self.widgets.replay_index == self.history.len()
                && self.current.board.pawn_to_promote.is_some() {
                self.side_panel_promote(ui);
            } else if let Some(draw) = &self.draw.draw_option
                && *draw == Request
            {
                self.side_panel_draw_request(ui);
            } else {
                self.turn_infos(ui);
                self.draw_resign(ui);
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.history.is_empty() || self.current.end.is_some() {
                    self.timer_increment(ui, ctx);
                }
            }
        });
    }

    //panels that open on special event and force input
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
        if self.widgets.replay_index == self.history.len() 
            && let Some(coord) = self.current.board.pawn_to_promote {
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
