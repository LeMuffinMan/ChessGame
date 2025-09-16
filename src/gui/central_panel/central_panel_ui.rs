use crate::ChessApp;

use eframe::egui;

impl ChessApp {
    //The central panels holds the board : there is the most of the rendering stuff here
    pub fn central_panel_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            //egui uses a painter to paint shapes : we ask for a painter, to draw shapes sensitive
            //to click and drag for user inputs
            let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());

            //we draw a big rectangle as background
            let rect = response.rect;
            let board_rect = centered_square(rect);
            let inner = if self.widgets.show_coordinates {
                render_border(&painter, board_rect);
                board_rect.shrink(16.0)
            } else {
                board_rect
            };

            //each cell is a square, 8x8 cells is a grid
            let sq = inner.width() / 8.0;

            if self.widgets.show_coordinates {
                self.display_coordinates(&painter, inner, sq);
            }
            //render the grid and the cells
            self.render_board(&painter, inner, sq);
            //get pieces position from Board.grid and draw pieces on board
            self.render_pieces(&painter, inner, sq);
            //if a piece is dragged, hide it on it cell, and display it under the pointer
            self.render_dragged_piece(&painter, inner);

            //Inputs functions :
            //left to select / try move
            //right to deselect (to fix)
            //drag_and_drop to select / try move
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
