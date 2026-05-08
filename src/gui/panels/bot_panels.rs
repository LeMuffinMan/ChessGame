use crate::ChessApp;
use crate::Color::*;

impl ChessApp {
    pub fn bot_white_panel_desktop(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("spacer_bottom").show(ctx, |ui| {
            if self.settings.flip {
                self.player_bar(ui, &White);
            } else {
                self.player_bar(ui, &Black);
            }
        });
    }

    pub fn bot_source_code_panel_desktop(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("source code").show(ctx, |ui| {
            let content_width = 500.0;
            let rect = egui::Rect::from_center_size(
                ui.max_rect().center(),
                egui::vec2(content_width, ui.max_rect().height()),
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to(
                        "Benchmark",
                        "https://lemuffinman.github.io/ChessGame/bench.html",
                    );
                    ui.separator();
                    ui.hyperlink_to("Source code", "https://github.com/LeMuffinMan/ChessGame");
                    ui.separator();
                    ui.hyperlink_to("Lichess", "https://lichess.org/@/LeMuffinBot");
                });
            });
        });
    }
}

pub fn format_time(seconds_f64: f64) -> String {
    let total_secs = seconds_f64.max(0.0).floor() as i64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    if mins > 0 {
        format!("{}:{:02}", mins, secs)
    } else {
        format!("0:{:02}", secs)
    }
}
