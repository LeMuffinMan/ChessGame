use crate::ChessApp;
use crate::gui::chessapp_struct::End::TimeOut;
use crate::gui::top_bot_panels::bot_panels::format_time;
use crate::gui::widgets::undo_redo_replay::Timer;

use egui::TextEdit;

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

    pub fn top_black_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let now = ctx.input(|i| i.time);
                //set timer if needed
                if self.widgets.timer.is_none()
                    && let Some(gm) = &self.widgets.game_mode
                {
                    self.widgets.timer = Timer::build(Some(*gm));
                }

                if let Some(timer) = &self.widgets.timer {
                    let rem = {
                        if let Some(start) = timer.black.0 {
                            timer.black.1 - (now - start)
                        } else {
                            timer.black.1
                        }
                    }
                    .max(0.0);
                    if rem == 0.0 {
                        self.current.end = Some(TimeOut);
                        self.history_san.push_str("1-0");
                        self.widgets.timer = None;
                        self.widgets.game_mode = None;
                    }
                    ui.heading(format_time(rem));
                }
                if self.history.is_empty() || self.current.end.is_some() {
                    ui.add(TextEdit::singleline(&mut self.black_name));
                } else {
                    ui.heading(&self.black_name);
                }
            });
        });
    }
}
