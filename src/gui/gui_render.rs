use crate::Coord;
use crate::Color;
use crate::cell::Piece;
use crate::Board;

pub fn centered_square(rect: egui::Rect) -> egui::Rect {
    let side = rect.width().min(rect.height());
    egui::Rect::from_center_size(rect.center(), egui::vec2(side, side))
}

pub fn draw_border(p: &egui::Painter, rect: egui::Rect) {
    let border_color = egui::Color32::from_rgb(50, 50, 50);
    p.rect_filled(rect, 0.0, border_color);
}

pub fn draw_board(p: &egui::Painter, inner: egui::Rect, sq: f32) {
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

pub fn draw_dragged_piece(
    painter: &egui::Painter, 
    inner: egui::Rect, 
    drag_from: Option<Coord>, 
    drag_pos: Option<egui::Pos2>, 
    board: &Board) {
    if let (Some(from), Some(pos)) = (drag_from, drag_pos) {
        if let (Some(piece), Some(color)) = (board.get(&from).get_piece(), board.get(&from).get_color()) {
        let ch: char = piece_char(*color, &piece);

        let font_px = (inner.width() / 8.0) * 0.8;
        let font = egui::FontId::proportional(font_px);
        let egui_color = if *color == Color::Black { egui::Color32::BLACK } else { egui::Color32::WHITE };

        painter.text(pos, egui::Align2::CENTER_CENTER, ch, font, egui_color);
        }
    }
}


pub fn draw_selection(
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

pub fn draw_pieces(
    p: &egui::Painter,
    inner: egui::Rect,
    sq: f32,
    board: &Board,
    flip: bool,
    drag_from: Option<Coord>,
) {
    for row in 0..8 {
        for col in 0..8 {
            let board_row = if flip { 7 - row } else { row };
            let coord = Coord { row: board_row as u8, col: col as u8 };
            if let Some(coord_dragged) = drag_from {
                if coord == coord_dragged {
                    continue;
                }
            }
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

pub fn ui_to_board(
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

pub fn board_to_ui_row_col(row: usize, col: usize, flip: bool) -> (usize, usize) {
    let ui_row = if flip { 7 - row } else { row };
    (ui_row, col)
}

pub fn draw_piece_unicode(painter: &egui::Painter, cell_rect: egui::Rect, ch: char, color: &Color) {
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

