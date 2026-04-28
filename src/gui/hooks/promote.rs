use crate::Board;
use crate::Color;
use crate::board::cell::Cell;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::gui::chessapp::ChessApp;
use crate::gui::layout::UiType::*;

#[derive(Clone)]
pub struct PromoteInfo {
    pub from: Coord,
    pub to: Coord,
    pub prev_board: Board,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
}

impl ChessApp {
    //When a player wants to promote a piece, we need to get out of try move so egui can request an input
    //This function prepare it : if it find a pawn to promote at an end  of turn, try move would stop before commiting the board
    // The player will then be prompted to input a piece for promotion, once done, the function hooks.rs/update_promote
    pub fn promote_pawn(
        &mut self,
        color: &Color,
        from: &Coord,
        to: &Coord,
        prev_board: &Board,
    ) -> Option<PromoteInfo> {
        let promote_row = if *color == White { 7 } else { 0 };
        for y in 0..8 {
            if self.game.board[(promote_row as usize, y as usize)].is_color(color)
                && let Some(piece) = self.game.board[(promote_row as usize, y as usize)].get_piece()
                && *piece == Pawn
            {
                return Some(PromoteInfo {
                    from: *from,
                    to: *to,
                    prev_board: prev_board.clone(),
                    pawn_to_promote: Some(*to),
                    promote: None, // this field will be filled by user through hooks()
                });
            }
        }
        None
    }
    //if a player promoted a pawn, try_move didnt finished it's work, so we do it here
    pub fn update_promote(&mut self) {
        if let Some(promote_info) = &self.promoteinfo
            && let Some(coord) = promote_info.pawn_to_promote
            && let Some(piece) = promote_info.promote
            && self.replay_infos.index == self.game.history.len()
        {
            let color = if self.game.active_player == White {
                Black
            } else {
                White
            };
            self.game.board[(coord.row as usize, coord.col as usize)] =
                Cell::Occupied(piece, color);

            self.update_threaten_cells();
            self.update_legals_moves();

            let k = match self.game.active_player {
                White => self.game.board.white_king,
                Black => self.game.board.black_king,
            };
            if self.game.threaten_cells.contains(&k) {
                self.game.board.check = Some(k);
            }
            if self.game.legals_moves.is_empty() {
                use crate::game::End;
                if self.game.threaten_cells.contains(&k) {
                    self.game.end = Some(End::Checkmate);
                } else {
                    self.game.end = Some(End::Pat);
                }
            }
            if let Some(promoteinfo) = &self.promoteinfo {
                let from = promoteinfo.from;
                let to = promoteinfo.to;
                let prev_board = promoteinfo.prev_board.clone();
                self.add_history_san(&from, &to, &prev_board);
                if self.game.end.is_none() && self.is_bot_turn() {
                    self.bot_pending = true;
                }
            }
            self.promoteinfo = None;
            self.win = None;
        }
    }
    pub fn get_promotion_input(&mut self, ctx: &egui::Context) {
        match self.ui_type {
            Mobile => {
                egui::Window::new("Promotion")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(140.0);
                            ui.vertical(|ui| {
                                if let Some(ref mut info) = self.promoteinfo {
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Queen), "Queen");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Bishop), "Bishop");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Knight), "Knight");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Rook), "Rook");
                                }
                            });
                        });
                        ui.add_space(20.0);
                    });
            }
            Desktop => {
                egui::Window::new("Promotion")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(100.0);
                            ui.vertical(|ui| {
                                if let Some(ref mut info) = self.promoteinfo {
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Queen), "Queen");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Bishop), "Bishop");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Knight), "Knight");
                                    ui.add_space(20.0);
                                    ui.selectable_value(&mut info.promote, Some(Rook), "Rook");
                                }
                            });
                        });
                        ui.add_space(20.0);
                    });
            }
        }
        self.update_promote();
    }
}
