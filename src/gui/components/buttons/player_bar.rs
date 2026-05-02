use crate::ChessApp;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::engine::bot::BotDifficulty::*;
use crate::engine::bot::PlayerType;
use crate::engine::bot::PlayerType::*;
use crate::engine::minimax::timed_out_iterative_deepening;
use crate::engine::search_context::{SearchContext, SearchParams};
use crate::engine::search_stats::MAX_SEARCH_DEPTH;
use crate::gui::chessapp::AppMode::*;
use crate::gui::features::timer::GameMode::NoTime;
use crate::gui::panels::bot_panels::format_time;
use egui::Align;
use egui::Layout;
use egui::RichText;
use egui::Vec2;

const HINT_TIMEOUT: f64 = 100.0;

impl ChessApp {
    pub fn player_bar(&mut self, ui: &mut egui::Ui, color: &Color) {
        let left_w = ui.available_width() * 0.35;
        let row_h = ui.spacing().interact_size.y;

        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                Vec2::new(left_w, row_h),
                Layout::top_down(Align::LEFT),
                |ui| {
                    self.player_name_ui(ui, color);
                    self.player_timer_ui(ui, color);
                },
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let bot_setting = self.bot_selector_ui(ui, color);

                let remaining_w = ui.available_width();
                let available_h = ui.available_height();
                ui.allocate_ui_with_layout(
                    Vec2::new(remaining_w, available_h),
                    Layout::top_down(Align::Center),
                    |ui| {
                        self.player_actions_ui(ui, color);
                    },
                );

                match color {
                    White => self.settings.white_bot = bot_setting,
                    Black => self.settings.black_bot = bot_setting,
                }
            });
        });
    }

    fn player_name_ui(&mut self, ui: &mut egui::Ui, color: &Color) {
        if self.game.history.is_empty() || self.game.end.is_some() {
            match color {
                White => {
                    ui.text_edit_singleline(&mut self.settings.white_name);
                }
                Black => {
                    ui.text_edit_singleline(&mut self.settings.black_name);
                }
            };
        } else {
            match color {
                White => ui.label(&self.settings.white_name),
                Black => ui.label(&self.settings.black_name),
            };
        }
    }

    fn player_timer_ui(&self, ui: &mut egui::Ui, color: &Color) {
        if self.timer.mode == NoTime {
            return;
        }
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

    fn bot_selector_ui(&self, ui: &mut egui::Ui, color: &Color) -> PlayerType {
        let mut bot_setting = match color {
            White => self.settings.white_bot,
            Black => self.settings.black_bot,
        };

        let current_depth = match bot_setting {
            Bot(Depth(d)) => d,
            _ => 5,
        };

        let is_enabled = self.app_mode == Lobby || self.app_mode == Replay;

        ui.add_enabled_ui(is_enabled, |ui| {
            if let Bot(Depth(ref mut d)) = bot_setting {
                let depth_label = format!("d={}", d);
                ui.menu_button(depth_label, |ui| {
                    for depth in 6..=11_u8 {
                        if ui
                            .selectable_label(*d == depth, format!("d={}", depth))
                            .clicked()
                        {
                            *d = depth;
                        }
                    }
                });
            }

            let type_label = match &bot_setting {
                Human => "Player",
                Bot(Random) => "Bot random",
                Bot(Adaptive) => "Bot adaptive",
                Bot(Depth(_)) => "Bot",
            };

            ui.menu_button(type_label, |ui| {
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
                    .selectable_label(bot_setting == Bot(Adaptive), "Bot adaptive")
                    .clicked()
                {
                    bot_setting = Bot(Adaptive);
                }
                if ui
                    .selectable_label(matches!(bot_setting, Bot(Depth(_))), "Bot")
                    .clicked()
                {
                    bot_setting = Bot(Depth(current_depth));
                }
            });
        });

        bot_setting
    }

    fn player_actions_ui(&mut self, ui: &mut egui::Ui, color: &Color) {
        let both_bots = self.settings.white_bot != Human && self.settings.black_bot != Human;
        if *color == White
            && both_bots
            && self.game.history.is_empty()
            && ui.button("▶ Start").clicked()
        {
            self.start_bot_game();
        }

        if self.settings.allow_hint
            && *color == self.game.active_player
            && ui
                .add_enabled(self.hint_highlight < 2, egui::Button::new("Hint"))
                .clicked()
        {
            if self.hint_highlight == 0 {
                let mut hint_ctx = SearchContext::new();
                let mut params = SearchParams::new(
                    &mut hint_ctx,
                    &self.game.draw.board_hashs,
                    self.game.draw.draw_moves_count,
                );
                if let Some(hint_move) = timed_out_iterative_deepening(
                    &mut self.game.board,
                    self.game.active_player,
                    MAX_SEARCH_DEPTH as u8,
                    &mut self.game.depth,
                    HINT_TIMEOUT,
                    &mut params,
                ) {
                    self.game.hint = Some((hint_move.origin, hint_move.dest));
                }
            }
            self.hint_highlight += 1;
        }
    }
}
