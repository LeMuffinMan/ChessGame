use crate::Coord;
use crate::Color;
use crate::Board;
use crate::mat_or_pat;
use crate::validate_move;
use crate::cell::Piece;
use eframe::{egui, App};

struct ChessApp {
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
            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
            let rect = response.rect;

            // 2) Damier centré
            let board_side = rect.width().min(rect.height());
            let board_rect = egui::Rect::from_center_size(
                rect.center(),
                egui::vec2(board_side, board_side),
            );

            // 3) Bordure
            let border = 12.0;
            let border_color = egui::Color32::from_rgb(50, 50, 50);
            painter.rect_filled(board_rect, 0.0, border_color);

            // 4) Intérieur
            let inner_rect = board_rect.shrink(border);
            let board_size = inner_rect.width();
            let square_size = board_size / 8.0;

            // 5) Damier + pièces
            let colors = [
                egui::Color32::from_rgb(240, 217, 181),
                egui::Color32::from_rgb(181, 136, 99),
            ];

            
            for row in 0..8 {
                for col in 0..8 {
                    // dessin case
                    let min = inner_rect.min + egui::vec2(col as f32 * square_size, row as f32 * square_size);
                    let cell = egui::Rect::from_min_size(min, egui::vec2(square_size, square_size));
                    let color_index = (row + col) % 2;
                    painter.rect_filled(cell, 0.0, colors[color_index]);

                    // inversion ligne pour accéder au Board
                    let board_row = if self.flip { 7 - row } else { row };
                    let coord = Coord { row: board_row as u8, col: col as u8 };

                    // surlignage de la sélection (comparer avec coord "board", pas "ui")
                    if let Some(sel) = self.from_cell {
                        if sel.row as usize == board_row && sel.col as usize == col {
                            painter.rect(
                                cell,
                                0.0,
                                egui::Color32::TRANSPARENT,
                                egui::Stroke::new(3.0, egui::Color32::YELLOW),
                                egui::StrokeKind::Inside,
                            );
                        }
                    }

                    if let Some(color) = self.board.get(&coord).get_color() {
                        if let Some(piece) = self.board.get(&coord).get_piece() {
                            let ch = piece_char(*color, piece);
                            draw_piece_unicode(&painter, cell, ch, &color);
                        }
                    }
                }
            }

            // 6) Gestion des clics
            if response.clicked() && self.checkmate == false {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    if inner_rect.contains(pointer_pos) {
                        let local = pointer_pos - inner_rect.min;
                        let col_ui = (local.x / square_size).floor() as i32;
                        let row_ui = (local.y / square_size).floor() as i32;
                        if (0..=7).contains(&col_ui) && (0..=7).contains(&row_ui) {

                            let row_board = if self.flip { 7 - row_ui } else { row_ui };
                            
                            let clicked = Coord { row: row_board as u8, col: col_ui as u8 };

                            match self.from_cell {
                                None => {
                                    self.from_cell = Some(clicked);
                                }
                                Some(from) => {
                                    let to = clicked;
                                    if from != to {
                                        if self.board.is_legal_move(&from, &to, &self.color) {
                                            if !validate_move::is_king_exposed(&from, &to, &self.color, &self.board) {
                                                println!("Move validated");
                                                self.board.update_board(&from, &to, &self.color);
                                                self.turn += 1;
                                                if self.color == Color::White {
                                                    self.color = Color::Black;
                                                } else {
                                                    self.color = Color::White
                                                }
                                                if mat_or_pat(&mut self.board, &self.color) {
                                                    self.checkmate = true;
                                                    return;
                                                }
                                                println!("{:?} to move", self.color);
                                                if let Some(coord) = self.board.get_king(&self.color) {
                                                    if self.board.threaten_cells.contains(&coord) {
                                                        println!("Check !");
                                                    }
                                                }
                                            } else {
                                                println!("King is exposed : illegal move");
                                            }
                                        } else {
                                            println!("Illegal move : {from:?} -> {to:?}");
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

            // Optionnel: clic droit pour annuler la sélection
            if response.clicked_by(egui::PointerButton::Secondary) {
                self.from_cell = None;
            }
        });
    }
}
fn piece_char(color: Color, piece: &Piece) -> char {
    match (color, piece) {
        (Color::Black, Piece::King)   => '♔',
        (Color::Black, Piece::Queen)  => '♕',
        (Color::Black, Piece::Rook)   => '♖',
        (Color::Black, Piece::Bishop) => '♗',
        (Color::Black, Piece::Knight) => '♘',
        (Color::Black, Piece::Pawn)   => '♙',
        (Color::White, Piece::King)   => '♚',
        (Color::White, Piece::Queen)  => '♛',
        (Color::White, Piece::Rook)   => '♜',
        (Color::White, Piece::Bishop) => '♝',
        (Color::White, Piece::Knight) => '♞',
        (Color::White, Piece::Pawn)   => '♟',
    }
}

fn draw_piece_unicode(
    painter: &egui::Painter,
    cell_rect: egui::Rect,
    ch: char,
    color: &Color,
) {
    let font_px = cell_rect.height() * 0.8;
    let font = egui::FontId::proportional(font_px);
    let color = if *color == Color::Black { egui::Color32::BLACK } else { egui::Color32::WHITE };
    painter.text(cell_rect.center(), egui::Align2::CENTER_CENTER, ch, font, color);
}

pub fn run_gui() {
    let app = ChessApp::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 1000.0])  // fenêtre plus grande
            .with_min_inner_size([700.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("ChessGame", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
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
//button rotate
//button new game
