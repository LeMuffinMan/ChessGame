use crate::ChessApp;
use crate::board::cell::Color::*;

impl ChessApp {
    pub fn top_title_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.heading("ChessGame");
                },
            );
        });
    }
    //Shows Black player name and its timer
    pub fn top_black_panel_desktop(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            if !self.settings.flip {
                self.player_bar(ui, &Black);
            } else {
                self.player_bar(ui, &White);
            }
        });
    }
}
