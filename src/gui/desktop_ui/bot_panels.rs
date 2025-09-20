use crate::ChessApp;
use crate::gui::update_timer::MobileGameMode::NoTime;
use egui::RichText;
use egui::TextEdit;

impl ChessApp {
    //The bot pannels show the white player name and its timer
    pub fn bot_white_panel_desktop(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("spacer_bottom").show(ctx, |ui| {
            ui.vertical(|ui| {
                // let now = ctx.input(|i| i.time);

                if self.timer.mode != NoTime {
                    if self.timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.timer.white_time) + " ⏱")
                                .size(30.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(
                                format_time(self.timer.white_time)
                                    + " ⏱ + "
                                    + &format_time(self.timer.increment).to_string(),
                            )
                            .size(30.0),
                        );
                    }
                }

                if self.history.is_empty() || self.current.end.is_some() {
                    ui.add(TextEdit::singleline(&mut self.white_name));
                } else {
                    ui.label(&self.white_name);
                }
            });
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
