use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Piece;
use crate::gui::chessapp_struct::End::*;

impl ChessApp {
    pub fn render_board(&self, p: &egui::Painter, inner: egui::Rect, sq: f32) {
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

                let board_row = if self.widgets.flip { 7 - row } else { row };
                let board_col = if self.widgets.flip { col } else { 7 - col };
                let coord = Coord {
                    row: board_row,
                    col: board_col,
                };
                let idx = (row + col) % 2;
                if let Some(_) = &self.current.board.check
                    && let Some(k) = self.current.board.get_king(&self.current.active_player)
                    && k.row == board_row
                    && k.col == board_col
                    && self.current.board.threaten_cells.contains(&k)
                {
                    if let Some(end) = &self.current.end
                        && *end == Checkmate
                    {
                        p.rect_filled(cell, 0.0, egui::Color32::from_rgb(255, 100, 100));
                    } else {
                        p.rect_filled(cell, 0.0, red[idx as usize]);
                    }
                    continue;
                }
                if self.widgets.show_threaten_cells
                    && self.current.board.threaten_cells.contains(&coord)
                {
                    p.rect_filled(cell, 0.0, red[idx as usize]);
                } else if self.highlight.piece_legals_moves.contains(&coord)
                    && self.widgets.show_legals_moves
                {
                    p.rect_filled(cell, 0.0, green[idx as usize]);
                } else if let Some((from, to)) = self.current.last_move
                    && (coord == from || coord == to)
                    && self.widgets.show_last_move
                {
                    p.rect_filled(cell, 0.0, blue[idx as usize]);
                } else if let Some(from) = self.highlight.from_cell
                    && coord == from
                {
                    p.rect_filled(cell, 0.0, blue[idx as usize]);
                } else {
                    p.rect_filled(cell, 0.0, colors[idx as usize]);
                }
            }
        }
    }

    pub fn render_dragged_piece(&self, painter: &egui::Painter, inner: egui::Rect) {
        if let (Some(from), Some(pos)) = (self.highlight.drag_from, self.highlight.drag_pos)
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

    pub fn render_pieces(&self, p: &egui::Painter, inner: egui::Rect, sq: f32) {
        for row in 0..8 {
            for col in 0..8 {
                let board_row = if self.widgets.flip { 7 - row } else { row };
                let board_col = if self.widgets.flip { col } else { 7 - col };
                let coord = Coord {
                    row: board_row as u8,
                    col: board_col as u8,
                };
                if let Some(coord_dragged) = self.highlight.drag_from
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
                    render_piece_unicode(p, cell, ch, color);
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
    let col_board = if flip { col_ui } else { 7 - col_ui };
    Some(Coord {
        row: row_board as u8,
        col: col_board as u8,
    })
}

pub fn render_piece_unicode(
    painter: &egui::Painter,
    cell_rect: egui::Rect,
    ch: char,
    color: &Color,
) {
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
    pub fn display_coordinates(&mut self, painter: &egui::Painter, inner: egui::Rect, sq: f32) {
        let font = egui::FontId::monospace(14.0);
        let color = egui::Color32::from_gray(200);

        let draw_labels = |count: usize,
                           pos_fn: &dyn Fn(usize) -> egui::Pos2,
                           align: egui::Align2,
                           text_fn: &dyn Fn(usize) -> String| {
            for i in 0..count {
                let text = text_fn(i);
                let galley = painter.layout_no_wrap(text, font.clone(), color);
                let pos = align
                    .align_size_within_rect(
                        galley.size(),
                        egui::Rect::from_center_size(pos_fn(i), galley.size()),
                    )
                    .min;
                painter.galley(pos, galley, color);
            }
        };

        let left_margin = 10.0;
        draw_labels(
            8,
            &|r| {
                let cy = inner.top() + r as f32 * sq + sq * 0.5;
                let x = inner.left() - left_margin;
                egui::pos2(x, cy)
            },
            egui::Align2::RIGHT_CENTER,
            &|r| {
                let idx = if self.widgets.flip { 7 - r + 1 } else { r + 1 };
                idx.to_string()
            },
        );

        let right_margin = 10.0;
        draw_labels(
            8,
            &|r| {
                let cy = inner.top() + r as f32 * sq + sq * 0.5;
                let x = inner.right() + right_margin;
                egui::pos2(x, cy)
            },
            egui::Align2::LEFT_CENTER,
            &|r| {
                let idx = if self.widgets.flip { 7 - r + 1 } else { r + 1 };
                idx.to_string()
            },
        );

        let top_margin = 8.0;
        draw_labels(
            8,
            &|c| {
                let cx = inner.left() + c as f32 * sq + sq * 0.5;
                let y = inner.top() - top_margin;
                egui::pos2(cx, y)
            },
            egui::Align2::CENTER_BOTTOM,
            &|c| {
                let idx = if self.widgets.flip { c } else { 7 - c };
                ((b'A' + idx as u8) as char).to_string()
            },
        );

        let bottom_margin = 8.0;
        draw_labels(
            8,
            &|c| {
                let cx = inner.left() + c as f32 * sq + sq * 0.5;
                let y = inner.bottom() + bottom_margin;
                egui::pos2(cx, y)
            },
            egui::Align2::CENTER_TOP,
            &|c| {
                let idx = if self.widgets.flip { c } else { 7 - c };
                ((b'A' + idx as u8) as char).to_string()
            },
        );
    }
}
