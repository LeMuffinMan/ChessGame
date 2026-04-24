use crate::ChessApp;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::engine::minimax::EASY_DEPTH;
use crate::engine::minimax::HARD_DEPTH;
use crate::engine::minimax::MEDIUM_DEPTH;
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
                // if self.ui_type == Mobile {
                //     self.engine_infos(ui, color);
                // }
                let both_bots =
                    self.settings.white_bot != Human && self.settings.black_bot != Human;

                if *color == White
                    && both_bots
                    && self.history.snapshots.is_empty()
                    && ui.button("▶ Start").clicked()
                {
                    self.start_bot_game();
                }
            });

            columns[2].with_layout(Layout::top_down(Align::Max), |ui| {
                let mut bot_setting = match color {
                    White => self.settings.white_bot,
                    Black => self.settings.black_bot,
                };

                let label = match &bot_setting {
                    Human => "Player".to_string(),
                    Bot(Easy) => format!("Bot (d = {})", EASY_DEPTH),
                    Bot(Medium) => format!("Bot (d = {})", MEDIUM_DEPTH),
                    Bot(Hard) => format!("Bot (d = {})", HARD_DEPTH),
                };

                ui.menu_button(label, |ui| {
                    if ui
                        .selectable_label(bot_setting == Human, "Player")
                        .clicked()
                    {
                        bot_setting = Human;
                    }
                    if ui
                        .selectable_label(
                            bot_setting == Bot(Easy),
                            format!("Bot (d = {})", EASY_DEPTH),
                        )
                        .clicked()
                    {
                        bot_setting = Bot(Easy);
                    }
                    if ui
                        .selectable_label(
                            bot_setting == Bot(Medium),
                            format!("Bot (d = {})", MEDIUM_DEPTH),
                        )
                        .clicked()
                    {
                        bot_setting = Bot(Medium);
                    }
                    if ui
                        .selectable_label(
                            bot_setting == Bot(Hard),
                            format!("Bot (d = {})", HARD_DEPTH),
                        )
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
