use crate::Board;
use crate::Color;
use crate::Coord;
use crate::cell::Piece;
use crate::mat_or_pat;
use crate::validate_move;
use eframe::{App, egui};

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

struct MoveOutcome {
    applied: bool,
    mate: bool,
    check: bool,
    messages: Vec<String>,
}

fn try_apply_move(
    board: &mut Board,
    color: &mut Color,
    turn: &mut u32,
    from: Coord,
    to: Coord,
) -> Option<MoveOutcome> {
    let mut msgs = vec![];
    if !board.is_legal_move(&from, &to, color) {
        msgs.push(format!("Illegal move : {from:?} -> {to:?}"));
        return Some(MoveOutcome { applied: false, mate: false, check: false, messages: msgs });
    }
    if validate_move::is_king_exposed(&from, &to, color, board) {
        msgs.push("King is exposed : illegal move".into());
        return Some(MoveOutcome { applied: false, mate: false, check: false, messages: msgs });
    }

    board.update_board(&from, &to, color);
    *turn += 1;
    *color = match *color { Color::White => Color::Black, Color::Black => Color::White };

    let mate = mat_or_pat(board, color);
    if mate {
        msgs.push("Checkmate or stalemate".into());
        return Some(MoveOutcome { applied: true, mate: true, check: false, messages: msgs });
    }

    msgs.push(format!("{color:?} to move"));
    let mut in_check = false;
    if let Some(k) = board.get_king(color) {
        if board.threaten_cells.contains(&k) {
            msgs.push("Check !".into());
            in_check = true;
        }
    }
    Some(MoveOutcome { applied: true, mate: false, check: in_check, messages: msgs })
}

fn centered_square(rect: egui::Rect) -> egui::Rect {
    let side = rect.width().min(rect.height());
    egui::Rect::from_center_size(rect.center(), egui::vec2(side, side))
}

fn draw_border(p: &egui::Painter, rect: egui::Rect) {
    let border_color = egui::Color32::from_rgb(50, 50, 50);
    p.rect_filled(rect, 0.0, border_color);
}

fn draw_board(p: &egui::Painter, inner: egui::Rect, sq: f32) {
    let colors = [
        egui::Color32::from_rgb(240, 217, 181),
        egui::Color32::from_rgb(181, 136, 99),
    ];
    for row in 0..8 {
        for col in 0..8 {
            let min = inner.min + egui::vec2(col as f32 * sq, row as f32 * sq);
            let cell = egui::Rect::from_min_size(min, egui::vec2(sq, sq));
            let idx = (row + col) % 2;
            p.rect_filled(cell, 0.0, colors[idx]);
        }
    }
}

fn draw_selection(
    p: &egui::Painter,
    inner: egui::Rect,
    sq: f32,
    flip: bool,
    from_cell: Option<Coord>,
) {
    if let Some(sel) = from_cell {
        let (row, col) = board_to_ui_row_col(sel.row as usize, sel.col as usize, flip);
        let min = inner.min + egui::vec2(col as f32 * sq, row as f32 * sq);
        let cell = egui::Rect::from_min_size(min, egui::vec2(sq, sq));
        p.rect(
            cell,
            0.0,
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(3.0, egui::Color32::YELLOW),
            egui::StrokeKind::Inside,
        );
    }
}

fn draw_pieces(
    p: &egui::Painter,
    inner: egui::Rect,
    sq: f32,
    board: &Board,
    flip: bool,
) {
    for row in 0..8 {
        for col in 0..8 {
            let board_row = if flip { 7 - row } else { row };
            let coord = Coord { row: board_row as u8, col: col as u8 };
            if let Some(color) = board.get(&coord).get_color() {
                if let Some(piece) = board.get(&coord).get_piece() {
                    let ch = piece_char(*color, piece);
                    let min = inner.min + egui::vec2(col as f32 * sq, row as f32 * sq);
                    let cell = egui::Rect::from_min_size(min, egui::vec2(sq, sq));
                    draw_piece_unicode(p, cell, ch, &color);
                }
            }
        }
    }
}

fn ui_to_board(
    inner: egui::Rect,
    sq: f32,
    flip: bool,
    pos: egui::Pos2,
) -> Option<Coord> {
    if !inner.contains(pos) { return None; }
    let local = pos - inner.min;
    let col_ui = (local.x / sq).floor() as i32;
    let row_ui = (local.y / sq).floor() as i32;
    if !(0..=7).contains(&col_ui) || !(0..=7).contains(&row_ui) { return None; }
    let row_board = if flip { 7 - row_ui } else { row_ui };
    Some(Coord { row: row_board as u8, col: col_ui as u8 })
}

fn board_to_ui_row_col(row: usize, col: usize, flip: bool) -> (usize, usize) {
    let ui_row = if flip { 7 - row } else { row };
    (ui_row, col)
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

fn piece_char(color: Color, piece: &Piece) -> char {
    match (color, piece) {
        (Color::Black, Piece::King) => '♔',
        (Color::Black, Piece::Queen) => '♕',
        (Color::Black, Piece::Rook) => '♖',
        (Color::Black, Piece::Bishop) => '♗',
        (Color::Black, Piece::Knight) => '♘',
        (Color::Black, Piece::Pawn) => '♙',
        (Color::White, Piece::King) => '♚',
        (Color::White, Piece::Queen) => '♛',
        (Color::White, Piece::Rook) => '♜',
        (Color::White, Piece::Bishop) => '♝',
        (Color::White, Piece::Knight) => '♞',
        (Color::White, Piece::Pawn) => '♟',
    }
}

fn draw_piece_unicode(painter: &egui::Painter, cell_rect: egui::Rect, ch: char, color: &Color) {
    let font_px = cell_rect.height() * 0.8;
    let font = egui::FontId::proportional(font_px);
    let color = if *color == Color::Black {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    };
    painter.text(
        cell_rect.center(),
        egui::Align2::CENTER_CENTER,
        ch,
        font,
        color,
    );
}

pub fn run_gui() {
    let app = ChessApp::default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 1000.0]) // fenêtre plus grande
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
