use crate::ChessApp;
use crate::gui::desktop_ui::bot_panels::format_time;
use crate::gui::chessapp_struct::MobileGameMode::NoTime;

use egui::TextEdit;
use egui::RichText;

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
    pub fn top_black_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.history.is_empty() || self.current.end.is_some() {
                        ui.text_edit_singleline(&mut self.black_name);
                } else {
                    ui.label(&self.black_name);
                }
                if self.mobile_timer.mode != NoTime {
                    if self.mobile_timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.mobile_timer.white_time) + " ⏱")
                                .size(30.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(
                                format_time(self.mobile_timer.white_time)
                                    + " ⏱ + "
                                    + &format_time(self.mobile_timer.increment).to_string(),
                            )
                            .size(30.0),
                        );
                    }
                }

            });
        });
    }
}
