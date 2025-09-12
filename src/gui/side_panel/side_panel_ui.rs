use crate::ChessApp;
use crate::Color;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::validate_move::try_move::mat_or_pat;
use egui::Context;

impl ChessApp {
    pub fn side_panel_ui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.heading("ChessGame");
        if let None = self.current.board.pawn_to_promote {
            ui.separator();
            self.turn_infos(ui);
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
        self.side_panel_promote(ui);
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

}
