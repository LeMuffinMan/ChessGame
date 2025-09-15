use crate::ChessApp;
// use crate::Color;
// use crate::Color::*;
// use crate::board::cell::Cell;
// use crate::board::cell::Piece::*;
// use crate::gui::chessapp_struct::DrawOption::*;
// use crate::gui::chessapp_struct::End::*;

use egui::Context;

impl ChessApp {
    pub fn left_panel_ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                if !self.history.is_empty() {
                    self.draw_resign(ui);
                }
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.history.is_empty() || self.current.end.is_some() {
                    self.timer_increment(ui, ctx);
                }
            });
    }



}
