use crate::Coord;
use crate::Color;
use crate::Board;
use crate::cell::Piece;
use eframe::{egui, App, NativeOptions};

struct ChessApp {
    board: Board,
    turn: u32,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: Board::init_board(),
            turn: 1,
        }
    }
}

impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ChessGame");

            // 1) Allouer une zone de peinture = toute la taille disponible du panel
            let size = ui.available_size();                      // taille dispo [5]
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click()); // [5]
            let rect = response.rect;                            // rect alloué [3]

            // 2) Calculer un carré centré (board_rect) dans rect
            let board_side = rect.width().min(rect.height());    // côté max qui tient [5]
            let board_rect = egui::Rect::from_center_size(
                rect.center(),                                   // centre du rect alloué [3]
                egui::vec2(board_side, board_side),
            );

            // 3) Bordure
            let border = 12.0;
            let border_color = egui::Color32::from_rgb(50, 50, 50);
            painter.rect_filled(board_rect, 0.0, border_color);  // fond de bordure [2]

            // 4) Rect intérieur (damier)
            let inner_rect = board_rect.shrink(border);          // marge égale sur 4 côtés [7]
            let board_size = inner_rect.width();                 // == height [7]
            let square_size = board_size / 8.0;

            // 5) Dessin du damier
            let colors = [
                egui::Color32::from_rgb(240, 217, 181),
                egui::Color32::from_rgb(181, 136, 99),
            ];
            
            for row in 0..8 {
                for col in 0..8 {
                    let min = inner_rect.min + egui::vec2(col as f32 * square_size, row as f32 * square_size);
                    let cell = egui::Rect::from_min_size(min, egui::vec2(square_size, square_size));
                    let color_index = (row + col) % 2;
                    painter.rect_filled(cell, 0.0, colors[color_index]); // fond de case

                    let coord = Coord { row: row as u8, col: col as u8 };

                    match self.board.get(&coord).get_color() {
                        Some(color) => {
                            match self.board.get(&coord).get_piece() {
                                Some(piece) => {
                                    let ch = piece_char(*color, piece);
                                    draw_piece_unicode(&painter, cell, ch, &color);
                                }
                                None => { }
                            }
                        }
                        None => { }
                    }
                }
            }
        });
    }
}

fn piece_char(color: Color, piece: &Piece) -> char {
    match (color, piece) {
        (Color::White, Piece::King)   => '♔',
        (Color::White, Piece::Queen)  => '♕',
        (Color::White, Piece::Rook)   => '♖',
        (Color::White, Piece::Bishop) => '♗',
        (Color::White, Piece::Knight) => '♘',
        (Color::White, Piece::Pawn)   => '♙',
        (Color::Black, Piece::King)   => '♚',
        (Color::Black, Piece::Queen)  => '♛',
        (Color::Black, Piece::Rook)   => '♜',
        (Color::Black, Piece::Bishop) => '♝',
        (Color::Black, Piece::Knight) => '♞',
        (Color::Black, Piece::Pawn)   => '♟',
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
