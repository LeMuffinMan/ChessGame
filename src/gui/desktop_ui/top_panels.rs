use crate::ChessApp;
use crate::gui::desktop_ui::bot_panels::format_time;
use crate::gui::update_timer::MobileGameMode::NoTime;

use egui::RichText;

impl ChessApp {
    //Shows Black player name and its timer
    pub fn top_black_panel_desktop(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.history.is_empty() || self.current.end.is_some() {
                    ui.text_edit_singleline(&mut self.black_name);
                } else {
                    ui.label(&self.black_name);
                }
                if self.timer.mode != NoTime {
                    if self.timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.timer.black_time) + " ⏱")
                                .size(30.0),
                        );
                    } else {
                        ui.label(
                            RichText::new(
                                format_time(self.timer.black_time)
                                    + " ⏱ + "
                                    + &format_time(self.timer.increment).to_string(),
                            )
                            .size(30.0),
                        );
                    }
                }
            });
        });
    }
}
