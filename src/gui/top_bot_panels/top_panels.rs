

use crate::ChessApp;
use crate::gui::chessapp_struct::End::TimeOut;
use crate::gui::top_bot_panels::bot_panels::format_time;

impl ChessApp {
    pub fn top_title_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("title")
            .show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.heading("ChessGame");
            });
        });
    }

    pub fn top_black_panel(&mut self, ctx: &egui::Context) {
        // ui.add(TextEdit::singleline(&mut self.white_name));
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let now = ctx.input(|i| i.time);
                if let Some(timer) = &self.widgets.timer {
                    let rem = {
                        if let Some(start) = timer.black.0 {
                            timer.black.1 - (now - start)
                        } else {
                            timer.black.1
                        }
                    }.max(0.0);
                    if rem == 0.0 {
                        self.current.end = Some(TimeOut);
                        //mettre a jour le pgn ici
                        self.widgets.timer = None;
                    }
                    ui.heading(format_time(rem));
                }
                ui.heading("Black");
            });
        });
    }
}
