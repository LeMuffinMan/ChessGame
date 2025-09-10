use crate::ChessApp;
use crate::gui::render::ui_to_board;
use crate::gui::move_result::try_apply_move;

impl ChessApp {
    pub fn drag_and_drop(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response) {
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(c) = ui_to_board(inner, sq, self.flip, pos){
                    if self.current.board.get(&c).is_color(&self.current.active_player) { 
                        self.drag_from = Some(c); 
                        self.from_cell = Some(c); 
                        self.drag_pos = Some(pos); 
                        if let Some(coord) = self.drag_from {
                            if self.piece_legals_moves.is_empty() {
                                for (from, to) in self.current.board.legals_moves.iter() {
                                    if from.row == coord.row && from.col == coord.col {
                                        println!("pushing {:?}", coord);
                                        self.piece_legals_moves.push(*to);
                                    }
                                }
                            }
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
                        self.undo.push(self.current.clone());
                        if let Some(outcome) = try_apply_move(&mut self.current.board, &mut self.current.active_player, &mut self.turn, from, dst) {
                            if outcome.applied == true {
                                self.redo.clear();
                                self.last_move = Some((from, dst));
                                if self.autoflip {
                                    self.flip = !self.flip;
                                }
                            }
                            //pat a revoir
                            if outcome.mate { 
                                self.current.checkmate = true; 
                            }
                            for m in outcome.messages {
                                println!("{m}"); 
                            } 
                        } 
                    } 
                } 
            } 
            self.piece_legals_moves.clear();
            self.from_cell=None; 
        }
    }

    pub fn right_click(&mut self, response: &egui::Response) {
        if response.clicked_by(egui::PointerButton::Secondary) {
            self.from_cell = None;
        }
    }

    pub fn left_click(&mut self, inner: egui::Rect, sq: f32, response: &egui::Response)
    {
        if response.clicked() && !self.current.checkmate {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(clicked) = ui_to_board(inner, sq, self.flip, pos) { 
                    match self.from_cell {
                        None => {
                            if self.current.board.get(&clicked).is_color(&self.current.active_player) {
                                self.piece_legals_moves.clear();
                                for (from, to) in self.current.board.legals_moves.iter() {
                                    if from.row == clicked.row && from.col == clicked.col {
                                        println!("pushing {:?}", clicked);
                                        self.piece_legals_moves.push(*to);
                                    }
                                }
                                self.from_cell = Some(clicked);
                            }
                        }
                        Some(from) => {
                            self.piece_legals_moves.clear();
                            if from != clicked {
                                self.redo.clear();
                                self.undo.push(self.current.clone());
                                if let Some(outcome) = try_apply_move(
                                    &mut self.current.board,
                                    &mut self.current.active_player,
                                    &mut self.turn,
                                    from,
                                    clicked,
                                ) {
                                    if outcome.applied == true {
                                        self.redo.clear();
                                        self.last_move = Some((from, clicked));
                                        if self.autoflip {
                                            self.flip = !self.flip;
                                        }
                                    }
                                    //revoir pat
                                    if outcome.mate { self.current.checkmate = true; }
                                    // logs optionnels
                                    for m in outcome.messages { println!("{m}"); }
                                }
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
