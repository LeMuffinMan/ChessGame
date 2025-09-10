use crate::Board;
use crate::Color;
use crate::Coord;
use eframe::{App, egui};
use egui::Pos2;
use crate::gui::gui_render::centered_square;
use crate::gui::gui_render::draw_border;
use crate::gui::gui_render::draw_board;
use crate::gui::gui_render::draw_pieces;
use crate::gui::gui_render::ui_to_board;
use crate::gui::gui_render::draw_dragged_piece;
use crate::gui::move_result::try_apply_move;


pub struct ChessApp {
    board: Board,
    turn: u32,
    from_cell: Option<Coord>,
    color: Color,
    flip: bool,
    checkmate: bool,
    drag_from: Option<Coord>,
    drag_pos: Option<Pos2>,
    selected_piece_legals_moves: Vec<Coord>,
    last_move: Option<(Coord, Coord)>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: Board::init_board(),
            turn: 1,
            from_cell: None,
            color: Color::White,
            flip: true,
            checkmate: false,
            drag_from: None,
            drag_pos: None,
            selected_piece_legals_moves: Vec::new(),
            last_move: None,
        }
    }
}

impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("ChessGame");
                ui.separator();

                ui.label(format!("Turn #{}", self.turn));
                ui.label(format!("{:?} to move", self.color));
                if ui.button("New game").clicked() {
                    *self = ChessApp::default();
                }
                if ui.button("Flip board").clicked() {
                    self.flip = !self.flip;
                }

                ui.separator();
                ui.label("last move:");
                if let Some((from, to)) = self.last_move {
                    ui.monospace(format!("{:?} -> {:?}", from, to));
                } else {
                    ui.monospace("—");
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // 1) Layout & painter
            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
            let rect = response.rect;

            let board_rect = centered_square(rect);              // cadre externe
            draw_border(&painter, board_rect);                   // bordure

            let inner = board_rect.shrink(12.0);
            let sq = inner.width() / 8.0;

            // --- Repères de lignes (A..H) à gauche du damier ---
            {
                let font = egui::FontId::monospace(14.0);
                let color = egui::Color32::from_gray(200);

                // Décalage latéral gauche pour laisser la place aux lettres
                let left_margin = 10.0;

                for r in 0..8 {
                    let idx = if self.flip { 7 - r + 1 } else { r + 1 };
                    let text = idx.to_string();
                    let galley = painter.layout_no_wrap(text, font.clone(), color); // fabrique la galée [1][3]
                    // Centre verticalement sur la case
                    let cy = inner.top() + r as f32 * sq + sq * 0.5;
                    let x = inner.left() - left_margin;

                    // Ancrer par la droite-centre pour coller au bord gauche du damier
                    let pos = egui::Align2::RIGHT_CENTER.align_size_within_rect(
                        galley.size(),
                        egui::Rect::from_center_size(egui::pos2(x, cy), galley.size()),
                    ).min; // calcule une position alignée [11][14]

                    painter.galley(pos, galley, color); // dessine la galée [1]
                }
            }

            // --- Repères de colonnes (0..9) en haut du damier ---
            {
                let font = egui::FontId::monospace(14.0);
                let color = egui::Color32::from_gray(200);

                // Hauteur de bande au-dessus du damier pour les chiffres
                let top_margin = 8.0;

                for c in 0..8 {
                    let label_idx = if self.flip { c } else { 7 - c };
                    let ch = (b'A' + label_idx as u8) as char;
                    let text = ch.to_string();
                    let galley = painter.layout_no_wrap(text, font.clone(), color); // [1][3]

                    // Centre horizontalement sur la colonne
                    let cx = inner.left() + c as f32 * sq + sq * 0.5;
                    let y = inner.top() - top_margin;

                    // Ancrer bas-centre pour coller au bord supérieur du damier
                    let pos = egui::Align2::CENTER_BOTTOM.align_size_within_rect(
                        galley.size(),
                        egui::Rect::from_center_size(egui::pos2(cx, y), galley.size()),
                    ).min; // [11][14]

                    painter.galley(pos, galley, color); // [1]
                }
            }

            // 2) Rendu
            draw_board(&painter, inner, sq, &self.selected_piece_legals_moves, &self.last_move, self.from_cell, self.flip);                     // damier
            draw_pieces(&painter, inner, sq, &self.board, self.flip, self.drag_from);   
            draw_dragged_piece(&painter, inner, self.drag_from, self.drag_pos, &self.board);

            // 3) Input gauche: sélection / déplacement
            if response.clicked() && !self.checkmate {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some(clicked) = ui_to_board(inner, sq, self.flip, pos) { 
                        if self.board.get(&clicked).is_color(&self.color) {
                            match self.from_cell {
                                None => {
                                    self.selected_piece_legals_moves.clear();
                                    for (from, to) in self.board.legals_moves.iter() {
                                        if from.row == clicked.row && from.col == clicked.col {
                                            println!("pushing {:?}", clicked);
                                            self.selected_piece_legals_moves.push(*to);
                                        }
                                    }
                                    self.from_cell = Some(clicked);
                                }
                                Some(from) => {
                                    self.selected_piece_legals_moves.clear();
                                    if from != clicked {
                                        if let Some(outcome) = try_apply_move(
                                            &mut self.board,
                                            &mut self.color,
                                            &mut self.turn,
                                            from,
                                            clicked,
                                        ) {
                                            self.last_move = Some((from, clicked));
                                            //revoir pat
                                            if outcome.mate { self.checkmate = true; }
                                            // logs optionnels
                                            for m in outcome.messages { println!("{m}"); }
                                        }
                                    }
                                    self.from_cell = None;
                                }
                            }
                        }
                    } else {
                        self.from_cell = None;
                    }
                }
            }

            // 4) Input droit: annuler sélection
            if response.clicked_by(egui::PointerButton::Secondary) {
                self.from_cell = None;
            }

            if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some(c) = ui_to_board(inner, sq, self.flip, pos){
                        if self.board.get(&c).is_color(&self.color) { 
                            self.drag_from = Some(c); 
                            self.from_cell = Some(c); 
                            self.drag_pos = Some(pos); 
                            if let Some(coord) = self.drag_from {
                                if self.selected_piece_legals_moves.is_empty() {
                                    for (from, to) in self.board.legals_moves.iter() {
                                        if from.row == coord.row && from.col == coord.col {
                                            println!("pushing {:?}", coord);
                                            self.selected_piece_legals_moves.push(*to);
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
                            if let Some(outcome) = try_apply_move(&mut self.board, &mut self.color, &mut self.turn, from, dst) {
                                if outcome.applied == true {
                                    self.last_move = Some((from, dst));
                                }
                                //pat a revoir
                                if outcome.mate { 
                                    self.checkmate = true; 
                                }
                                for m in outcome.messages {
                                    println!("{m}"); 
                                } 
                            } 
                        } 
                    } 
                } 
                self.selected_piece_legals_moves.clear();
                self.from_cell=None; 
            }
        });
    }
}


//TO DO
//
//side pannel with info
//   - color to play
//   - moves history
//   - pieces took
//Coords on sides
