use crate::Board;
use crate::Color;
use crate::Coord;
use eframe::{App, egui};
use crate::gui::gui_render::centered_square;
use crate::gui::gui_render::draw_border;
use crate::gui::gui_render::draw_board;
use crate::gui::gui_render::draw_selection;
use crate::gui::gui_render::draw_pieces;
use crate::gui::gui_render::ui_to_board;
use crate::gui::move_result::try_apply_move;

pub struct ChessApp {
    board: Board,
    turn: u32,
    from_cell: Option<Coord>,
    color: Color,
    flip: bool,
    checkmate: bool,
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
        }
    }
}

impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ChessGame");

            // 1) Layout & painter
            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
            let rect = response.rect;

            let board_rect = centered_square(rect);              // cadre externe
            draw_border(&painter, board_rect);                   // bordure

            let inner = board_rect.shrink(12.0);
            let sq = inner.width() / 8.0;

            // 2) Rendu
            draw_board(&painter, inner, sq);                     // damier
            draw_selection(&painter, inner, sq, self.flip, self.from_cell); // surlignage
            draw_pieces(&painter, inner, sq, &self.board, self.flip);       // pièces

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
        });
    }
}


//TO DO
//
//Drag and drop
//show legal moves on click
//red king if check
//side pannel with info
//   - color to play
//   - moves history
//   - pieces took
//Coords on sides
//button rotate
//button new game
