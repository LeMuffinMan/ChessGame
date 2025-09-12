use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;

pub fn centered_square(rect: egui::Rect) -> egui::Rect {
    let side = rect.width().min(rect.height());
    egui::Rect::from_center_size(rect.center(), egui::vec2(side, side))
}

pub fn draw_border(p: &egui::Painter, rect: egui::Rect) {
    let border_color = egui::Color32::from_rgb(50, 50, 50);
    p.rect_filled(rect, 0.0, border_color);
}

impl ChessApp {
    pub fn draw_board(&self, p: &egui::Painter, inner: egui::Rect, sq: f32) {
        let colors = [
            egui::Color32::from_rgb(240, 217, 181),
            egui::Color32::from_rgb(181, 136, 99),
        ];
        let green = [
            egui::Color32::from_rgb(240, 240, 181),
            egui::Color32::from_rgb(181, 160, 99),
        ];
        let blue = [
            egui::Color32::from_rgb(200, 200, 230),
            egui::Color32::from_rgb(150, 170, 200),
        ];
        let red = [
            egui::Color32::from_rgb(240, 200, 200),
            egui::Color32::from_rgb(200, 150, 150),
        ];
        for row in 0..8 {
            for col in 0..8 {
                let min = inner.min + egui::vec2(col as f32 * sq, row as f32 * sq);
                let cell = egui::Rect::from_min_size(min, egui::vec2(sq, sq));

                let board_row = if self.flip { 7 - row } else { row };
                let coord = Coord {
                    row: board_row,
                    col,
                };
                let idx = (row + col) % 2;
                if let Some(_) = &self.current.board.check
                    && let Some(k) = self.current.board.get_king(&self.current.active_player)
                    && k.row == board_row
                    && k.col == col
                    && self.current.board.threaten_cells.contains(&k)
                {
                    p.rect_filled(cell, 0.0, red[idx as usize]);
                    continue;
                }
                if self.show_threaten_cells && self.current.board.threaten_cells.contains(&coord) {
                    p.rect_filled(cell, 0.0, red[idx as usize]);
                } else if self.piece_legals_moves.contains(&coord) && self.show_legals_moves {
                    p.rect_filled(cell, 0.0, green[idx as usize]);
                } else if let Some((from, to)) = self.current.last_move
                    && (coord == from || coord == to)
                    && self.show_last_move
                {
                    p.rect_filled(cell, 0.0, blue[idx as usize]);
                } else if let Some(from) = self.from_cell
                    && coord == from
                {
                    p.rect_filled(cell, 0.0, blue[idx as usize]);
                } else {
                    p.rect_filled(cell, 0.0, colors[idx as usize]);
                }
            }
        }
    }

    pub fn draw_dragged_piece(&self, painter: &egui::Painter, inner: egui::Rect) {
        if let (Some(from), Some(pos)) = (self.drag_from, self.drag_pos)
            && let (Some(piece), Some(color)) = (
                self.current.board.get(&from).get_piece(),
                self.current.board.get(&from).get_color(),
            )
        {
            let ch: char = piece_char(*color, piece);

            let font_px = (inner.width() / 8.0) * 0.8;
            let font = egui::FontId::proportional(font_px);
            let egui_color = if *color == Black {
                egui::Color32::BLACK
            } else {
                egui::Color32::WHITE
            };

            painter.text(pos, egui::Align2::CENTER_CENTER, ch, font, egui_color);
        }
    }

    pub fn draw_pieces(&self, p: &egui::Painter, inner: egui::Rect, sq: f32) {
        for row in 0..8 {
            for col in 0..8 {
                let board_row = if self.flip { 7 - row } else { row };
                let board_col = if self.flip { col } else { 7 - col };
                let coord = Coord {
                    row: board_row as u8,
                    col: board_col as u8,
                };
                if let Some(coord_dragged) = self.drag_from
                    && coord == coord_dragged
                {
                    continue;
                }
                if let Some(color) = self.current.board.get(&coord).get_color()
                    && let Some(piece) = self.current.board.get(&coord).get_piece()
                {
                    let ch = piece_char(*color, piece);
                    let min = inner.min + egui::vec2(col as f32 * sq, row as f32 * sq);
                    let cell = egui::Rect::from_min_size(min, egui::vec2(sq, sq));
                    draw_piece_unicode(p, cell, ch, color);
                }
            }
        }
    }
}

fn piece_char(color: Color, piece: &Piece) -> char {
    match (color, piece) {
        (Black, Piece::King) => '♔',
        (Black, Piece::Queen) => '♕',
        (Black, Piece::Rook) => '♖',
        (Black, Piece::Bishop) => '♗',
        (Black, Piece::Knight) => '♘',
        (Black, Piece::Pawn) => '♙',
        (White, Piece::King) => '♚',
        (White, Piece::Queen) => '♛',
        (White, Piece::Rook) => '♜',
        (White, Piece::Bishop) => '♝',
        (White, Piece::Knight) => '♞',
        (White, Piece::Pawn) => '♟',
    }
}

pub fn ui_to_board(inner: egui::Rect, sq: f32, flip: bool, pos: egui::Pos2) -> Option<Coord> {
    if !inner.contains(pos) {
        return None;
    }
    let local = pos - inner.min;
    let col_ui = (local.x / sq).floor() as i32;
    let row_ui = (local.y / sq).floor() as i32;
    if !(0..=7).contains(&col_ui) || !(0..=7).contains(&row_ui) {
        return None;
    }
    let row_board = if flip { 7 - row_ui } else { row_ui };
    Some(Coord {
        row: row_board as u8,
        col: col_ui as u8,
    })
}

pub fn draw_piece_unicode(painter: &egui::Painter, cell_rect: egui::Rect, ch: char, color: &Color) {
    let font_px = cell_rect.height() * 0.8;
    let font = egui::FontId::proportional(font_px);
    let color = if *color == Black {
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

impl ChessApp {
    pub fn show_coordinates(&mut self, painter: &egui::Painter, inner: egui::Rect, sq: f32) {
        {
            let font = egui::FontId::monospace(14.0);
            let color = egui::Color32::from_gray(200);

            let left_margin = 10.0;

            for r in 0..8 {
                let idx = if self.flip { 7 - r + 1 } else { r + 1 };
                let text = idx.to_string();
                let galley = painter.layout_no_wrap(text, font.clone(), color);
                let cy = inner.top() + r as f32 * sq + sq * 0.5;
                let x = inner.left() - left_margin;

                let pos = egui::Align2::RIGHT_CENTER
                    .align_size_within_rect(
                        galley.size(),
                        egui::Rect::from_center_size(egui::pos2(x, cy), galley.size()),
                    )
                    .min;

                painter.galley(pos, galley, color);
            }
        }
        {
            let font = egui::FontId::monospace(14.0);
            let color = egui::Color32::from_gray(200);

            let top_margin = 8.0;

            for c in 0..8 {
                let label_idx = if self.flip { c } else { 7 - c };
                let ch = (b'A' + label_idx as u8) as char;
                let text = ch.to_string();
                let galley = painter.layout_no_wrap(text, font.clone(), color);

                let cx = inner.left() + c as f32 * sq + sq * 0.5;
                let y = inner.top() - top_margin;

                let pos = egui::Align2::CENTER_BOTTOM
                    .align_size_within_rect(
                        galley.size(),
                        egui::Rect::from_center_size(egui::pos2(cx, y), galley.size()),
                    )
                    .min;
                painter.galley(pos, galley, color);
            }
        }
    }
}
