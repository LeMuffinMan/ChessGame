use crate::ChessApp;
use crate::gui::central_panel::render::ui_to_board;

impl ChessApp {
    pub fn get_piece_legal_moves(&mut self) {
        if let Some(coord) = self.drag_from {
            for (from, to) in self.current.board.legals_moves.iter() {
                if from.row == coord.row && from.col == coord.col {
                    // println!("pushing {:?}", coord);
                    self.piece_legals_moves.push(*to);
                }
            }
        }
    }

    pub fn drag_and_drop(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(c) = ui_to_board(inner, sq, self.flip, pos) {
                    if self.is_active_player_piece(&c)
                        && !self.current.end.is_some()
                        && let None = self.current.board.pawn_to_promote
                    {
                        self.drag_from = Some(c);
                        self.from_cell = Some(c);
                        self.drag_pos = Some(pos);
                        if self.piece_legals_moves.is_empty() {
                            self.get_piece_legal_moves();
                        }
                    }
                }
            }
        }
        if response.dragged() {
            self.drag_pos = response.interact_pointer_pos();
        }

        if response.drag_stopped() {
            if let (Some(from), Some(pos)) = (self.drag_from.take(), self.drag_pos.take()) {
                if let Some(dst) = ui_to_board(inner, sq, self.flip, pos) {
                    if from != dst {
                        self.try_move(from, dst);
                    }
                }
            }
            self.piece_legals_moves.clear();
            self.from_cell = None;
        }
    }

    pub fn right_click(&mut self, response: &egui::Response) {
        if response.clicked_by(egui::PointerButton::Secondary) {
            self.from_cell = None;
        }
    }

    pub fn left_click(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.clicked()
            && !self.current.end.is_some()
            && let None = self.current.board.pawn_to_promote
        {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(clicked) = ui_to_board(inner, sq, self.flip, pos) {
                    match self.from_cell {
                        None => {
                            if self
                                .current
                                .board
                                .get(&clicked)
                                .is_color(&self.current.active_player)
                            {
                                self.piece_legals_moves.clear();
                                for (from, to) in self.current.board.legals_moves.iter() {
                                    if from.row == clicked.row && from.col == clicked.col {
                                        // println!("pushing {:?}", clicked);
                                        self.piece_legals_moves.push(*to);
                                    }
                                }
                                self.from_cell = Some(clicked);
                            }
                        }
                        Some(from) => {
                            self.piece_legals_moves.clear();
                            if from != clicked {
                                self.try_move(from, clicked);
                            }
                            self.from_cell = None;
                        }
                    }
                } else {
                    self.from_cell = None;
                }
            }
        }
    }
}
