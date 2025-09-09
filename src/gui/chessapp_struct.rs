use crate::Board;
use crate::Color;
use crate::Coord;
use crate::cell::Piece;
use eframe::{App, egui};
use egui::Pos2;
use crate::gui::gui_render::centered_square;
use crate::gui::gui_render::draw_border;
use crate::gui::gui_render::draw_board;
use crate::gui::gui_render::draw_selection;
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
    piece_selected_legals_moves: Vec<(Coord, Coord)>,
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
            piece_selected_legals_moves: Vec::new(),
        }
    }
}

impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ChessGame");

            // 1) Layout & painter
            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
            let rect = response.rect;

            let board_rect = centered_square(rect);              // cadre externe
            draw_border(&painter, board_rect);                   // bordure

            let inner = board_rect.shrink(12.0);
            let sq = inner.width() / 8.0;

            // 2) Rendu

            let mut piece_legal_moves: Vec<Coord> = Vec::new();
            if let Some(coord) = self.drag_from {
                for (from, to) in self.board.legals_moves.iter() {
                    if from.row == coord.row && from.col == coord.col {
                        println!("pushing {:?}", coord);
                        piece_legal_moves.push(*to);
                    }
                }
            }
            draw_board(&painter, inner, sq, &piece_legal_moves, self.flip);                     // damier
            draw_selection(&painter, inner, sq, self.flip, self.from_cell); // surlignage
            draw_pieces(&painter, inner, sq, &self.board, self.flip, self.drag_from);       // pièces
            draw_dragged_piece(&painter, inner, self.drag_from, self.drag_pos, &self.board);
            // 3) Input gauche: sélection / déplacement
            if response.clicked() && !self.checkmate {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some(clicked) = ui_to_board(inner, sq, self.flip, pos) {
                        match self.from_cell {
                            None => self.from_cell = Some(clicked),
                            Some(from) => {
                                if from != clicked {
                                    if let Some(outcome) = try_apply_move(
                                        &mut self.board,
                                        &mut self.color,
                                        &mut self.turn,
                                        from,
                                        clicked,
                                    ) {
                                        if outcome.mate { self.checkmate = true; }
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
                self.from_cell=None; 
            }
        });
    }
}


//TO DO
//
//show legal moves on click
//red king if check
//side pannel with info
//   - color to play
//   - moves history
//   - pieces took
//Coords on sides
//button rotate
//button new game
