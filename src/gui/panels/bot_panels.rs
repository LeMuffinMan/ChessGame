use crate::ChessApp;
use crate::Color::*;

impl ChessApp {
    //The bot pannels show the white player name and its timer
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
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.hyperlink_to("Source code", "https://github.com/LeMuffinMan/ChessGame");
                },
            );
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
