
use crate::ChessApp;
use crate::gui::chessapp_struct::Timer;

use egui::Context;

impl ChessApp {
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            ui.label("Timer :");
            if self.widgets.timer.is_none() {
                if ui.add_enabled(self.history.is_empty(), egui::Button::new("OFF")).clicked() {
                    let timer: Timer = Default::default();
                    self.widgets.timer = Some(timer);
                }
            } else {
                //Si on est pas en cour de game
                if let Some(timer) = &mut self.widgets.timer {
                    if timer.white.0.is_none() && timer.black.0.is_none() {
                        let white_remaining_time: &mut f64 = &mut timer.white.1;
                        let black_remaining_time: &mut f64 = &mut timer.black.1;

                        let mut remove_timer = false;

                        ui.menu_button(
                            format!(
                                "{:.0} min",
                                (*white_remaining_time / 60.0).floor(),
                            ),
                            |ui| {
                                if ui.button("OFF").clicked() {
                                    remove_timer = true;
                                }
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
                        //used flag cause of double borrow case
                        if remove_timer {
                            self.widgets.timer = None;
                        }
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
        }
    }

}
