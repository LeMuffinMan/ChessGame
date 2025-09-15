use crate::ChessApp;
use crate::gui::chessapp_struct::GameMode::*;
// use crate::gui::widgets::undo_redo_replay::Timer;
use crate::Color::*;
use crate::gui::chessapp_struct::GameMode;
use crate::gui::chessapp_struct::End::TimeOut;

use egui::Context;

impl ChessApp {
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.label("Bullet");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Bullet(20.0, 1.0)),
                    "0:20 + 1",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Bullet(30.0, 0.0)),
                    "0:30 + 0",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Bullet(60.0, 0.0)),
                    " 1:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Bullet(60.0, 1.0)),
                    " 1:00 + 1",
                );
            });
        });
        ui.separator();
        ui.label("Blitz");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Blitz(180.0, 0.0)),
                    "3:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Blitz(180.0, 2.0)),
                    "3:00 + 2",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Blitz(300.0, 0.0)),
                    "5:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Blitz(300.0, 5.0)),
                    "5:00 + 5",
                );
            });
        });
        ui.selectable_value(
            &mut self.widgets.game_mode,
            Some(Blitz(300.0, 2.0)),
            "5:00 + 2",
        );
        ui.separator();
        ui.label("Rapid");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(600.0, 0.0)),
                    "10:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(600.0, 5.0)),
                    "10:00 + 5",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(900.0, 10.0)),
                    "15:00 + 10",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(1200.0, 0.0)),
                    "20:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(1800.0, 0.0)),
                    "30:00 + 0",
                );
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Some(Rapid(3600.0, 5.0)),
                    "60:00 + 0",
                )
            });
        });
        ui.separator();
        // ui.horizontal(|ui| {
            ui.selectable_value(&mut self.widgets.game_mode, Some(Custom(600.0, 0.0)), "Custom timer");
            //Si on est pas en cours de game
            // if self.widgets.custom_timer && let Some(timer) = &mut self.widgets.timer {
            //     if timer.white.0.is_none() && timer.black.0.is_none() {
            //         let white_remaining_time: &mut f64 = &mut timer.white.1;
            //         let black_remaining_time: &mut f64 = &mut timer.black.1;
            if let Some(gm) = &mut self.widgets.game_mode {
                match gm {
                    | GameMode::Custom(time, inc) => {
                        ui.horizontal(|ui| {
                            ui.label("Time :");
                            ui.menu_button(
                                format!("{:.0} min", (*time / 60.0).floor(),),
                                |ui| {
                                    if ui.button("1 min").clicked() {
                                        *time = 60.0;
                                    }
                                    if ui.button("3 min").clicked() {
                                        *time = 180.0;
                                    }
                                    if ui.button("5 min").clicked() {
                                        *time = 300.0;
                                    }
                                    if ui.button("10 min").clicked() {
                                        *time = 600.0;
                                    }
                                    if ui.button("15 min").clicked() {
                                        *time = 900.0;
                                    }
                                    if ui.button("30 min").clicked() {
                                        *time = 1800.0;
                                    }
                                },
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Increment :");
                            ui.menu_button(format!("{:.0} sec", *inc,), |ui| {
                                if ui.button("None").clicked() {
                                    *inc = 0.0;
                                }
                                if ui.button("1 sec").clicked() {
                                    *inc = 1.0;
                                }
                                if ui.button("2 sec").clicked() {
                                    *inc = 2.0;
                                }
                                if ui.button("10 sec").clicked() {
                                    *inc = 10.0;
                                }
                            });
                        });
                        ui.separator();
                    },
                    _ => { },
                    }
                }
                // }
            // }
        // });
    }
    pub fn update_timer(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);

        //Switching timers for each turn
        if let Some(timer) = &mut self.widgets.timer {
            if timer.white.0.is_none()
                && self.current.active_player == White
                && !self.history.is_empty()
            {
                timer.white.0 = Some(now);
                if let Some(black_start) = timer.black.0 {
                    timer.black.1 += timer.increment;
                    timer.black.1 -= now - black_start;
                }
                timer.black.0 = None;
                //start the timer at the beginning
            } else if timer.black.0.is_none() && self.current.active_player == Black {
                if self.history.len() == 1
                /* && let Some(timer) = &mut self.widgets.timer */
                {
                    if let Some(game_mode) = &self.widgets.game_mode {
                        match game_mode {
                            GameMode::Bullet(max_time, inc)
                            | GameMode::Blitz(max_time, inc)
                            | GameMode::Rapid(max_time, inc)
                            | GameMode::Custom(max_time, inc) => {
                                timer.white.1 = *max_time;
                                timer.black.1 = *max_time;
                                timer.increment = *inc;
                            }
                        }
                    }
                }
                timer.black.0 = Some(now);
                if let Some(white_start) = timer.white.0 {
                    timer.white.1 += timer.increment;
                    timer.white.1 -= now - white_start;
                }
                timer.white.0 = None;
            }
            if timer.white.1 < 0.0 {
                timer.white.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
            if timer.black.1 < 0.0 {
                timer.black.1 = 0.0;
                self.current.end = Some(TimeOut);
            }
        }
    }

}
