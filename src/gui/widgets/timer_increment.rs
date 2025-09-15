
use crate::ChessApp;
use crate::gui::chessapp_struct::Timer;
use crate::gui::chessapp_struct::GameMode::*;

use egui::Context;

impl ChessApp {
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            if ui.checkbox(&mut self.widgets.show_timer, "Timer").changed() {
                if self.widgets.show_timer {
                    self.widgets.timer = Some(Timer::default());
                } else {
                    self.widgets.timer = None;
                }
            } else {
                //Si on est pas en cour de game
                if let Some(timer) = &mut self.widgets.timer {
                    if timer.white.0.is_none() && timer.black.0.is_none() {
                        let white_remaining_time: &mut f64 = &mut timer.white.1;
                        let black_remaining_time: &mut f64 = &mut timer.black.1;

                        ui.menu_button(
                            format!(
                                "{:.0} min",
                                (*white_remaining_time / 60.0).floor(),
                            ),
                            |ui| {
                                if ui.button("1 min").clicked() {
                                    *white_remaining_time = 60.0;
                                    *black_remaining_time = 60.0;
                                }
                                if ui.button("3 min").clicked() {
                                    *white_remaining_time = 180.0;
                                    *black_remaining_time = 180.0;
                                }
                                if ui.button("5 min").clicked() {
                                    *white_remaining_time = 300.0;
                                    *black_remaining_time = 300.0;
                                }
                                if ui.button("10 min").clicked() {
                                    *white_remaining_time = 600.0;
                                    *black_remaining_time = 600.0;
                                }
                                if ui.button("15 min").clicked() {
                                    *white_remaining_time = 900.0;
                                    *black_remaining_time = 900.0;
                                }
                                if ui.button("30 min").clicked() {
                                    *white_remaining_time = 1800.0;
                                    *black_remaining_time = 1800.0;
                                }
                            },
                        );
                    }
                }               
            } 
        });
        if let Some(timer) = &mut self.widgets.timer {
            ui.horizontal(|ui| {
                ui.label("Increment :");
                ui.menu_button(
                    format!(
                        "{:.0} sec",
                        timer.increment,
                    ),
                    |ui| {
                        if ui.button("None").clicked() {
                            timer.increment = 0.0;
                        }
                        if ui.button("1 sec").clicked() {
                            timer.increment = 1.0;
                        }
                        if ui.button("2 sec").clicked() {
                            timer.increment = 2.0;
                        }
                        if ui.button("10 sec").clicked() {
                            timer.increment = 10.0;
                        }
                    },
                );
            });
            ui.separator();
            ui.label("Bullet");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Bullet(20.0, 1.0)), "20 sec | 1 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Bullet(30.0, 0.0)), "30 sec | 0 sec");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Bullet(60.0, 0.0)), "1 min | 0 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Bullet(60.0, 1.0)), "1 min | 1 sec");
            });
            ui.separator();
            ui.label("Blitz");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Blitz(180.0, 0.0)), "3 min | 0 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Blitz(180.0, 2.0)), "3 min | 2 sec");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Blitz(300.0, 0.0)), "5 min | 0 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Blitz(300.0, 5.0)), "5 min | 5 sec");
            });
            ui.selectable_value(&mut self.widgets.game_mode, Some(Blitz(300.0, 2.0)), "5 min | 2 sec");
            ui.separator();
            ui.label("Rapid");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(600.0, 0.0)), "10 min | 0 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(600.0, 5.0)), "10 min | 5 sec");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(900.0, 10.0)), "15 min | 10 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(1200.0, 0.0)), "20 min | 0 sec");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(1800.0, 0.0)), "30 min | 0 sec");
                ui.selectable_value(&mut self.widgets.game_mode, Some(Rapid(3600.0, 5.0)), "60 min | 0 sec");
            });
        }
    }

}
