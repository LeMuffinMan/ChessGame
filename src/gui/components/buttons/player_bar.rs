use crate::ChessApp;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::gui::features::timer::GameMode::NoTime;
use crate::gui::layout::UiType::*;
use crate::gui::panels::bot_panels::format_time;
use egui::Align;
use egui::Layout;

use egui::RichText;

impl ChessApp {
    pub fn player_bar(&mut self, ui: &mut egui::Ui, color: &Color) {
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| {
                if self.history.snapshots.is_empty() || self.current.end.is_some() {
                    match color {
                        White => ui.text_edit_singleline(&mut self.settings.white_name),
                        Black => ui.text_edit_singleline(&mut self.settings.black_name),
                    };
                } else {
                    match color {
                        White => ui.label(&self.settings.white_name),
                        Black => ui.label(&self.settings.black_name),
                    };
                }

                if self.timer.mode != NoTime {
                    let time_val = match color {
                        White => self.timer.white_time,
                        Black => self.timer.black_time,
                    };

                    let text = if self.timer.increment == 0.0 {
                        format_time(time_val) + " ⏱"
                    } else {
                        format!(
                            "{} ⏱ + {}",
                            format_time(time_val),
                            format_time(self.timer.increment)
                        )
                    };
                    ui.label(RichText::new(text).size(20.0));
                }
            });

            columns[1].vertical_centered(|ui| {
                if self.ui_type == Mobile {
                    self.engine_infos(ui, color);
                }
            });

            columns[2].with_layout(Layout::top_down(Align::Max), |ui| {
                let mut bot_setting = match color {
                    White => self.settings.white_bot,
                    Black => self.settings.black_bot,
                };

                let label = match &bot_setting {
                    Human => "Player",
                    Bot(Easy) => "Bot - depth = 2",
                    Bot(Medium) => "Bot - depth = 3",
                    Bot(Hard) => "Bot - depth = 4",
                };

                ui.menu_button(label, |ui| {
                    if ui
                        .selectable_label(bot_setting == Human, "Player")
                        .clicked()
                    {
                        bot_setting = Human;
                    }
                    if ui
                        .selectable_label(bot_setting == Bot(Easy), "Bot (d = 2)")
                        .clicked()
                    {
                        bot_setting = Bot(Easy);
                    }
                    if ui
                        .selectable_label(bot_setting == Bot(Medium), "Bot (d = 3)")
                        .clicked()
                    {
                        bot_setting = Bot(Medium);
                    }
                    if ui
                        .selectable_label(bot_setting == Bot(Hard), "Bot (d = 4)")
                        .clicked()
                    {
                        bot_setting = Bot(Hard);
                    }
                });

                match color {
                    White => self.settings.white_bot = bot_setting,
                    Black => self.settings.black_bot = bot_setting,
                }
            });
        });
    }
}
