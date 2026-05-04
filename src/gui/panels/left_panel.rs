use crate::ChessApp;
use crate::Color::*;
use crate::engine::bot::PlayerType::*;
use crate::gui::chessapp::AppMode;
use crate::gui::chessapp::AppMode::*;

use egui::Context;

impl ChessApp {
    pub fn left_panel_desktop(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(270.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                if matches!(&self.app_mode, AppMode::Versus(None)) {
                    self.draw_resign_undo_desktop(ui);
                }
                self.engine_infos(ui, &self.game.active_player);
                self.new_game_replay(ui);
                if self.app_mode == Lobby {
                    self.undo_limit_hint(ui);
                    if self.settings.white_bot == Human && self.settings.black_bot == Human {
                        self.timer_increment(ui, ctx);
                    }
                }

                if self.app_mode == Replay {
                    // self.new_save_load(ui, ctx);
                    self.replay_buttons(ui);
                    if matches!(self.app_mode, AppMode::Replay) {
                        ui.horizontal(|ui| {
                            ui.add_space(40.0);
                            ui.vertical(|ui| {
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.replay_infos.sec_per_frame,
                                        0.1..=5.0,
                                    )
                                    .logarithmic(true),
                                );
                            });
                        });
                    }
                }
            });
    }

    pub fn undo_limit_hint(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.toggle_value(&mut self.settings.allow_hint, "Hint");

            if ui
                .toggle_value(&mut self.settings.allow_undo, "Allow undo")
                .changed()
            {
                if self.settings.allow_undo {
                    self.settings.white_undo = 0;
                    self.settings.black_undo = 0;
                } else {
                    self.settings.white_undo = 1;
                    self.settings.black_undo = 1;
                }
            }
            if self.settings.allow_undo {
                ui.menu_button(format!("{}", &self.settings.undo_limit), |ui| {
                    for i in 0..6 {
                        self.selectable_value_undo(ui, i);
                    }
                });
            }
        });
    }

    pub fn selectable_value_undo(&mut self, ui: &mut egui::Ui, value: u8) {
        if ui
            .selectable_value(&mut self.settings.undo_limit, value, format!("{}", value))
            .clicked()
        {
            self.settings.white_undo = value;
            self.settings.black_undo = value;
            if value == 0 {
                self.settings.allow_undo = false;
            }
        };
    }

    pub fn can_undo(&mut self) -> bool {
        match self.game.opponent() {
            White => self.settings.white_undo > 0,
            Black => self.settings.black_undo > 0,
        }
    }
}
