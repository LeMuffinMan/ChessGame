use crate::ChessApp;
use crate::gui::central_panel::render::ui_to_board;

impl ChessApp {
    pub fn get_piece_legal_moves(&mut self) {
        if let Some(coord) = self.highlight.drag_from {
            for (from, to) in self.current.board.legals_moves.iter() {
                if from.row == coord.row && from.col == coord.col {
                    // println!("pushing {:?}", coord);
                    self.highlight.piece_legals_moves.push(*to);
                }
            }
        }
    }

    pub fn drag_and_drop(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.drag_started()
            && let Some(pos) = response.interact_pointer_pos()
            && let Some(c) = ui_to_board(inner, sq, self.widgets.flip, pos)
            && self.is_active_player_piece(&c)
            && self.current.end.is_none()
            && let None = self.current.board.pawn_to_promote
        {
            self.highlight.drag_from = Some(c);
            self.highlight.from_cell = Some(c);
            self.highlight.drag_pos = Some(pos);
            if self.highlight.piece_legals_moves.is_empty() {
                self.get_piece_legal_moves();
            }
        }
        if response.dragged() {
            self.highlight.drag_pos = response.interact_pointer_pos();
        }

        if response.drag_stopped()
            && let (Some(from), Some(pos)) = (self.highlight.drag_from.take(), self.highlight.drag_pos.take())
            && let Some(dst) = ui_to_board(inner, sq, self.widgets.flip, pos)
            && from != dst
        {
            self.try_move(from, dst);
            self.highlight.piece_legals_moves.clear();
            self.highlight.from_cell = None;
        }
    }

    pub fn right_click(&mut self, response: &egui::Response) {
        if response.clicked_by(egui::PointerButton::Secondary) {
            self.highlight.from_cell = None;
        }
    }

    pub fn left_click(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.clicked()
            && self.current.end.is_none()
            && self.current.board.pawn_to_promote.is_none()
            && let Some(pos) = response.interact_pointer_pos()
        {
            if let Some(clicked) = ui_to_board(inner, sq, self.widgets.flip, pos) {
                match self.highlight.from_cell {
                    None => {
                        if self
                            .current
                            .board
                            .get(&clicked)
                            .is_color(&self.current.active_player)
                        {
                            self.highlight.piece_legals_moves.clear();
                            for (from, to) in self.current.board.legals_moves.iter() {
                                if from.row == clicked.row && from.col == clicked.col {
                                    // println!("pushing {:?}", clicked);
                                    self.highlight.piece_legals_moves.push(*to);
                                }
                            }
                            self.highlight.from_cell = Some(clicked);
                        }
                    }
                    Some(from) => {
                        self.highlight.piece_legals_moves.clear();
                        if from != clicked {
                            self.try_move(from, clicked);
                        }
                        self.highlight.from_cell = None;
                    }
                }
            } else {
                self.highlight.from_cell = None;
            }
        }
    }
}
