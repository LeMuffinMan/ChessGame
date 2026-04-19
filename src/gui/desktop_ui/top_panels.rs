use crate::ChessApp;
use crate::gui::bot_difficulty::BotDifficulty::*;
use crate::gui::desktop_ui::bot_panels::format_time;
use crate::gui::player_type::PlayerType::*;
use crate::gui::update_timer::GameMode::NoTime;

use egui::RichText;

impl ChessApp {
    //Shows Black player name and its timer
    pub fn top_black_panel_desktop(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("spacer_top").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if self.history.snapshots.is_empty() || self.current.end.is_some() {
                        ui.text_edit_singleline(&mut self.settings.black_name);
                    } else {
                        ui.label(&self.settings.black_name);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let label = match &self.settings.black_bot {
                            Human => "Player",
                            Bot(Easy) => "Bot - Easy",
                            Bot(Medium) => "Bot - Medium",
                            Bot(Hard) => "Bot - Hard",
                        };
                        ui.menu_button(label, |ui| {
                            if ui
                                .selectable_label(self.settings.black_bot == Human, "Player")
                                .clicked()
                            {
                                self.settings.black_bot = Human;
                                ui.close_menu();
                            }
                            if ui
                                .selectable_label(
                                    self.settings.black_bot == Bot(Easy),
                                    "Bot - Easy",
                                )
                                .clicked()
                            {
                                self.settings.black_bot = Bot(Easy);
                                ui.close_menu();
                            }
                            if ui
                                .selectable_label(
                                    self.settings.black_bot == Bot(Medium),
                                    "Bot - Medium",
                                )
                                .clicked()
                            {
                                self.settings.black_bot = Bot(Medium);
                                ui.close_menu();
                            }
                            if ui
                                .selectable_label(
                                    self.settings.black_bot == Bot(Hard),
                                    "Bot - Hard",
                                )
                                .clicked()
                            {
                                self.settings.black_bot = Bot(Hard);
                                ui.close_menu();
                            }
                        });
                    });
                });
                if self.timer.mode != NoTime {
                    if self.timer.increment == 0.0 {
                        ui.label(
                            RichText::new(format_time(self.timer.black_time) + " ⏱").size(30.0),
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
