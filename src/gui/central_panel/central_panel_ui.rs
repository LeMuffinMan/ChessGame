use crate::ChessApp;

use eframe::egui;

impl ChessApp {
    pub fn central_panel_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        let board_rect = centered_square(rect);
        let inner = if self.widgets.show_coordinates {
            render_border(&painter, board_rect);
            board_rect.shrink(16.0)
        } else {
            board_rect
        };

        let sq = inner.width() / 8.0;

        if self.widgets.show_coordinates {
            self.display_coordinates(&painter, inner, sq);
        }
        self.render_board(&painter, inner, sq);
        self.render_pieces(&painter, inner, sq);
        self.render_dragged_piece(&painter, inner);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
        });
    }
}

fn centered_square(rect: egui::Rect) -> egui::Rect {
    let side = rect.width().min(rect.height());
    egui::Rect::from_center_size(rect.center(), egui::vec2(side, side))
}

fn render_border(p: &egui::Painter, rect: egui::Rect) {
    let border_color = egui::Color32::from_rgb(50, 50, 50);
    p.rect_filled(rect, 0.0, border_color);
}
