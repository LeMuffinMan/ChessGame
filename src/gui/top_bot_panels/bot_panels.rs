
use crate::ChessApp;
use crate::gui::chessapp_struct::End::TimeOut;

impl ChessApp {
    pub fn bot_white_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("spacer_bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let now = ctx.input(|i| i.time);
                if let Some(timer) = &self.widgets.timer {
                    let rem = {
                        if let Some(start) = timer.white.0 {
                            timer.white.1 - (now - start)
                        } else {
                            timer.white.1
                        }
                    }.max(0.0);
                    if rem == 0.0 {
                        self.current.end = Some(TimeOut);
                    }
                    ui.label(format_time(rem));
                } 
                ui.label("White");
            });
        });
    }

    pub fn bot_source_code_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("source code")
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.label("source code : https://github.com/LeMuffinMan/ChessGame");
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
