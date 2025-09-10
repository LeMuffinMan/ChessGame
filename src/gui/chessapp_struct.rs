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

#[derive(Clone)]
pub struct GameState {
    board: Board,
    turn: u32,
    from_cell: Option<Coord>,
    active_player: Color,
    flip: bool,
    checkmate: bool,
    drag_from: Option<Coord>,
    drag_pos: Option<Pos2>,
    selected_piece_legals_moves: Vec<Coord>,
    last_move: Option<(Coord, Coord)>,
}

pub struct ChessApp {
    current: GameState,
    undo: Vec<GameState>,
    redo: Vec<GameState>,
    autoflip: bool,
    show_coordinates: bool,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            current: GameState {
                board: Board::init_board(),
                turn: 1,
                from_cell: None,
                active_player: Color::White,
                flip: true,
                checkmate: false,
                drag_from: None,
                drag_pos: None,
                selected_piece_legals_moves: Vec::new(),
                last_move: None,
            },
            undo: Vec::new(),
            redo: Vec::new(),
            autoflip: false,
            show_coordinates: false,
        }
    }
}

impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .default_width(150.0)
            .show(ctx, |ui| {
                self.side_panel_ui(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
            });
    }
}

impl ChessApp {
    fn side_panel_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("ChessGame");
        
        ui.separator();

        ui.label(format!("Turn #{}", self.current.turn));
        ui.label(format!("{:?} to move", self.current.active_player));
        
        ui.separator();

        if ui.button("New game").clicked() {
            *self = ChessApp::default();
        }

        ui.separator();

        if ui.button("Flip board").clicked() {
            self.current.flip = !self.current.flip;
        }
        if ui.toggle_value(&mut self.autoflip, "Autoflip").changed() {
        }

        ui.separator();

        if ui.checkbox(&mut self.show_coordinates, "Coordinates").changed() {

        }

        ui.separator();
        ui.horizontal(|ui| {
            let can_undo = !self.undo.is_empty();
            let can_redo = !self.redo.is_empty();
                if ui.add_enabled(can_undo, egui::Button::new("Undo")).clicked() {
                    if let Some(prev) = self.undo.pop() {
                        self.redo.push(self.current.clone());
                        self.current = prev;
                        self.current.selected_piece_legals_moves.clear();
                    }
                }
                if ui.add_enabled(can_redo, egui::Button::new("Redo")).clicked() {
                    if let Some(next) = self.redo.pop() {
                        self.undo.push(self.current.clone());
                        self.current = next;
                    }
                }
        });
        //
        ui.separator();
        ui.label("last move:");
        if let Some((from, to)) = self.current.last_move {
            ui.monospace(format!("{:?} -> {:?}", from, to));
        } else {
            ui.monospace("—");
        }
    }
    fn show_coordinates(&mut self, painter: &egui::Painter, inner: egui::Rect, sq: f32) {
        // --- Repères de lignes (A..H) à gauche du damier ---
        {
            let font = egui::FontId::monospace(14.0);
            let color = egui::Color32::from_gray(200);

            // Décalage latéral gauche pour laisser la place aux lettres
            let left_margin = 10.0;

            for r in 0..8 {
                let idx = if self.current.flip { 7 - r + 1 } else { r + 1 };
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
                let label_idx = if self.current.flip { c } else { 7 - c };
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
    } 

    fn central_panel_ui(&mut self, ui: &mut egui::Ui) {
        // 1) Layout & painter
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        let board_rect = centered_square(rect);              // cadre externe
        let inner = if self.show_coordinates {
            draw_border(&painter, board_rect);                   // bordure
            board_rect.shrink(16.0)
        } else { board_rect };

        let sq = inner.width() / 8.0;

        if self.show_coordinates { self.show_coordinates(&painter, inner, sq); }
        // 2) Rendu
        draw_board(&painter, inner, sq, &self.current.selected_piece_legals_moves, &self.current.last_move, self.current.from_cell, self.current.flip);                     // damier
        draw_pieces(&painter, inner, sq, &self.current.board, self.current.flip, self.current.drag_from);   
        draw_dragged_piece(&painter, inner, self.current.drag_from, self.current.drag_pos, &self.current.board);

        // 3) Input gauche: sélection / déplacement
        if response.clicked() && !self.current.checkmate {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(clicked) = ui_to_board(inner, sq, self.current.flip, pos) { 
                    match self.current.from_cell {
                        None => {
                            if self.current.board.get(&clicked).is_color(&self.current.active_player) {
                                self.current.selected_piece_legals_moves.clear();
                                for (from, to) in self.current.board.legals_moves.iter() {
                                    if from.row == clicked.row && from.col == clicked.col {
                                        println!("pushing {:?}", clicked);
                                        self.current.selected_piece_legals_moves.push(*to);
                                    }
                                }
                                self.current.from_cell = Some(clicked);
                            }
                        }
                        Some(from) => {
                            self.current.selected_piece_legals_moves.clear();
                            if from != clicked {
                                // self.current.redo.clear();
                                self.undo.push(self.current.clone());
                                if let Some(outcome) = try_apply_move(
                                    &mut self.current.board,
                                    &mut self.current.active_player,
                                    &mut self.current.turn,
                                    from,
                                    clicked,
                                ) {
                                    if outcome.applied == true {
                                        self.redo.clear();
                                        self.current.last_move = Some((from, clicked));
                                        if self.autoflip {
                                            self.current.flip = !self.current.flip;
                                        }
                                    }
                                    //revoir pat
                                    if outcome.mate { self.current.checkmate = true; }
                                    // logs optionnels
                                    for m in outcome.messages { println!("{m}"); }
                                }
                            }
                            self.current.from_cell = None;
                        }
                    }
                    
                } else {
                    self.current.from_cell = None;
                }
            }
        }

        // 4) Input droit: annuler sélection
        if response.clicked_by(egui::PointerButton::Secondary) {
            self.current.from_cell = None;
        }

        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(c) = ui_to_board(inner, sq, self.current.flip, pos){
                    if self.current.board.get(&c).is_color(&self.current.active_player) { 
                        self.current.drag_from = Some(c); 
                        self.current.from_cell = Some(c); 
                        self.current.drag_pos = Some(pos); 
                        if let Some(coord) = self.current.drag_from {
                            if self.current.selected_piece_legals_moves.is_empty() {
                                for (from, to) in self.current.board.legals_moves.iter() {
                                    if from.row == coord.row && from.col == coord.col {
                                        println!("pushing {:?}", coord);
                                        self.current.selected_piece_legals_moves.push(*to);
                                    }
                                }
                            }
                        }
                    } 
                } 
            } 
        }
        if response.dragged() {
            self.current.drag_pos = response.interact_pointer_pos(); 
        }

        if response.drag_stopped() { 
            if let (Some(from), Some(pos)) = (self.current.drag_from.take(), self.current.drag_pos.take()) {
                if let Some(dst) = ui_to_board(inner, sq, self.current.flip, pos) {
                    if from != dst {
                        self.undo.push(self.current.clone());
                        if let Some(outcome) = try_apply_move(&mut self.current.board, &mut self.current.active_player, &mut self.current.turn, from, dst) {
                            if outcome.applied == true {
                                self.redo.clear();
                                self.current.last_move = Some((from, dst));
                                if self.autoflip {
                                    self.current.flip = !self.current.flip;
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
            self.current.selected_piece_legals_moves.clear();
            self.current.from_cell=None; 
        }
    }
}



//TO DO
//interface promotion
//moves history
//   - pgn ?
//   - moves history
//   - pieces took
//Replay : rejoue depuis le debut chaque snapshot
