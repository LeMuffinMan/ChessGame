use crate::ChessApp;
use crate::Color;
use crate::Color::*;
use crate::board::cell::Cell;
use crate::board::cell::Piece::*;
use crate::gui::chessapp_struct::DrawOption::*;
use crate::gui::chessapp_struct::End::*;

use egui::Context;

impl ChessApp {
    pub fn left_panel_ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(180.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                self.draw_resign(ui);
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.history.is_empty() || self.current.end.is_some() {
                    self.timer_increment(ui, ctx);
                }
            });
    }

    //panels that open on special event and force input
    fn side_panel_draw_request(&mut self, ui: &mut egui::Ui) {
        ui.label("Accept draw offer ?");
        ui.horizontal(|ui| {
            if ui.button("Accept").clicked() {
                self.current.end = Some(Draw);
                self.draw.draw_option = None;
            }
            if ui.button("Reject").clicked() {
                self.draw.draw_option = None;
            }
        });
    }

}
