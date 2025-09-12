use crate::ChessApp;
use crate::gui::central_panel::render::{centered_square, draw_border};

use eframe::egui;

impl ChessApp {
    pub fn central_panel_ui(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        let board_rect = centered_square(rect);
        let inner = if self.show_coordinates {
            draw_border(&painter, board_rect);
            board_rect.shrink(16.0)
        } else {
            board_rect
        };

        let sq = inner.width() / 8.0;

        if self.show_coordinates {
            self.show_coordinates(&painter, inner, sq);
        }
        self.draw_board(&painter, inner, sq);
        self.draw_pieces(&painter, inner, sq);
        self.draw_dragged_piece(&painter, inner,);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }


}
