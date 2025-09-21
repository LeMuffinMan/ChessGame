use crate::ChessApp;
use crate::gui::render::ui_to_board;

impl ChessApp {
    pub fn drag_and_drop(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.drag_started()
            && let Some(pos) = response.interact_pointer_pos()
            && let Some(c) = ui_to_board(inner, sq, self.settings.flip, pos)
            && self.current.is_active_player_piece(&c)
            && self.current.end.is_none()
            && let None = self.current.board.pawn_to_promote
            && self.replay_infos.index == self.history.snapshots.len()
        {
            self.settings.drag_from = Some(c);
            self.settings.from_cell = Some(c);
            self.settings.drag_pos = Some(pos);
            if self.settings.piece_legals_moves.is_empty() {
                self.get_piece_legal_moves();
            }
        }
        if response.dragged() {
            self.settings.drag_pos = response.interact_pointer_pos();
        }

        if response.drag_stopped()
            && let (Some(from), Some(pos)) = (
                self.settings.drag_from.take(),
                self.settings.drag_pos.take(),
            )
            && let Some(dst) = ui_to_board(inner, sq, self.settings.flip, pos)
            && from != dst
        {
            self.try_move(from, dst);
            self.settings.piece_legals_moves.clear();
            self.settings.from_cell = None;
        }
    }

    pub fn right_click(&mut self, response: &egui::Response) {
        if response.clicked_by(egui::PointerButton::Secondary) {
            self.settings.from_cell = None;
        }
    }

    pub fn left_click(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.clicked()
            && self.current.end.is_none()
            && self.current.board.pawn_to_promote.is_none()
            && let Some(pos) = response.interact_pointer_pos()
            && self.replay_infos.index == self.history.snapshots.len()
        {
            if let Some(clicked) = ui_to_board(inner, sq, self.settings.flip, pos) {
                match self.settings.from_cell {
                    None => {
                        if self
                            .current
                            .board
                            .get(&clicked)
                            .is_color(&self.current.active_player)
                        {
                            self.settings.piece_legals_moves.clear();
                            for (from, to) in self.current.board.legals_moves.iter() {
                                if from.row == clicked.row && from.col == clicked.col {
                                    // println!("pushing {:?}", clicked);
                                    self.settings.piece_legals_moves.push(*to);
                                }
                            }
                            self.settings.from_cell = Some(clicked);
                        }
                    }
                    Some(from) => {
                        self.settings.piece_legals_moves.clear();
                        if from != clicked {
                            self.try_move(from, clicked);
                        }
                        self.settings.from_cell = None;
                    }
                }
            } else {
                self.settings.from_cell = None;
            }
        }
    }
}
