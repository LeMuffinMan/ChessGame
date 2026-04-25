use crate::ChessApp;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType::*;
use crate::engine::minimax::EASY_DEPTH;
use crate::engine::minimax::HARD_DEPTH;
use crate::engine::minimax::MEDIUM_DEPTH;
use crate::gui::features::timer::GameMode::NoTime;
use crate::gui::layout::UiType;
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
                    Bot(Random) => "Bot random".into(),
                    Bot(Easy) => "Bot easy".into(),
                    Bot(Medium) => "Bot medium".into(),
                    Bot(Hard) => "Bot hard".into(),
                };

                let max_depth = match &self.ui_type {
                    UiType::Desktop => 9,
                    UiType::Mobile => 7,
                };

                let mut bot_depth = match color {
                    White => self.settings.white_bot_depth,
                    Black => self.settings.black_bot_depth,
                };

                ui.horizontal_centered(|ui| {
                    ui.menu_button(label, |ui| {
                        if ui
                            .selectable_label(bot_setting == Human, "Player")
                            .clicked()
                        {
                            bot_setting = Human;
                        }
                        if ui
                            .selectable_label(bot_setting == Bot(Random), "Bot random")
                            .clicked()
                        {
                            bot_setting = Bot(Random);
                        }
                        if ui
                            .selectable_label(bot_setting == Bot(Easy), "Bot easy")
                            .clicked()
                        {
                            bot_setting = Bot(Easy);
                        }
                        if ui
                            .selectable_label(bot_setting == Bot(Medium), "Bot medium")
                            .clicked()
                        {
                            bot_setting = Bot(Medium);
                        }
                        if ui
                            .selectable_label(bot_setting == Bot(Hard), "Bot hard")
                            .clicked()
                        {
                            bot_setting = Bot(Hard);
                        }
                    });

                    // Menu déroulant pour la profondeur (depth)
                    if bot_setting != Human {
                        let combo_id = match color {
                            White => "white_depth_combo",
                            Black => "black_depth_combo",
                        };

                        egui::ComboBox::from_id_source(combo_id)
                            .selected_text(bot_depth.to_string())
                            .width(40.0) // Optionnel: pour éviter que le menu soit trop large
                            .show_ui(ui, |ui| {
                                for depth in 1..=max_depth {
                                    ui.selectable_value(&mut bot_depth, depth, depth.to_string());
                                }
                            });
                    }
                });

                // Sauvegarde des paramètres
                match color {
                    White => {
                        self.settings.white_bot = bot_setting;
                        self.settings.white_bot_depth = bot_depth;
                    }
                    Black => {
                        self.settings.black_bot = bot_setting;
                        self.settings.black_bot_depth = bot_depth;
                    }
                }
            });
        });
    }
}
