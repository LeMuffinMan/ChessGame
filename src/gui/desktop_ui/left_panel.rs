use crate::ChessApp;
use crate::Color::*;
use crate::gui::chessapp_struct::AppMode;
use crate::gui::chessapp_struct::AppMode::*;

use egui::Context;

impl ChessApp {
    //Shows turn infos, resign / draw options, new game option and timer options
    pub fn left_panel_desktop(&mut self, ctx: &Context) {
        egui::SidePanel::left("left_panel")
            .default_width(270.0)
            .show(ctx, |ui| {
                self.turn_infos(ui);
                if matches!(&self.app_mode, AppMode::Versus(None)) {
                    self.draw_resign_undo(ui);
                }
                // ui.separator();
                self.new_save_load(ui, ctx);
                if self.app_mode == Lobby {
                    self.undo_limit(ui);
                    self.timer_increment(ui, ctx);
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

    pub fn undo_limit(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
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
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 0, "No limit")
                        .clicked()
                    {
                        self.settings.white_undo = 0;
                        self.settings.black_undo = 0;
                        self.settings.allow_undo = false;
                    };
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 1, "1")
                        .clicked()
                    {
                        self.settings.white_undo = 1;
                        self.settings.black_undo = 1;
                    };
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 2, "2")
                        .clicked()
                    {
                        self.settings.white_undo = 2;
                        self.settings.black_undo = 2;
                    };
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 3, "3")
                        .clicked()
                    {
                        self.settings.white_undo = 3;
                        self.settings.black_undo = 3;
                    };
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 4, "4")
                        .clicked()
                    {
                        self.settings.white_undo = 4;
                        self.settings.black_undo = 4;
                    };
                    if ui
                        .selectable_value(&mut self.settings.undo_limit, 5, "5")
                        .clicked()
                    {
                        self.settings.white_undo = 5;
                        self.settings.black_undo = 5;
                    };
                });
            }
        });
    }

    pub fn can_undo(&mut self) -> bool {
        match self.current.opponent {
            White => self.settings.white_undo > 0,
            Black => self.settings.black_undo > 0,
        }
    }
}
