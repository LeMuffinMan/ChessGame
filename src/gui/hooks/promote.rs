use crate::Board;
use crate::Color;
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
    pub fn update_promote(&mut self) {
        let (from, to, prev_board, piece) = match &self.promoteinfo {
            Some(info)
                if info.pawn_to_promote.is_some()
                    && info.promote.is_some()
                    && self.replay_infos.index == self.game.history.len() =>
            {
                (
                    info.from,
                    info.to,
                    info.prev_board.clone(),
                    info.promote.unwrap(),
                )
            }
            _ => return,
        };

        self.promoteinfo = None;
        self.win = None;

        if let Some(event) = self.game.try_move_promotion(from, to, piece) {
            use crate::game::End;
            use crate::game::GameEvent::*;
            use crate::gui::chessapp::AppMode::Versus;

            self.last_move = Some((from, to));
            match event {
                Checkmate => {
                    self.app_mode = Versus(Some(End::Checkmate));
                    self.timer.active = false;
                }
                Stalemate => {
                    self.app_mode = Versus(Some(End::Pat));
                    self.timer.active = false;
                }
                Draw => {
                    self.app_mode = Versus(Some(End::Draw));
                }
                _ => {}
            }
            self.add_history_san(&from, &to, &prev_board);
            if self.game.end.is_none() && self.is_bot_turn() {
                self.bot_pending = true;
            }
            self.hint_highlight = 0;
            self.game.hint = None;
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
