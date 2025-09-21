use crate::ChessApp;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::GameMode::*;
use crate::gui::update_timer::Timer;

use egui::Context;

//a refacto
impl ChessApp {
    //This shows the list a presets for timings
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        if ui
            .add_enabled(self.timer.mode != NoTime, egui::Button::new("Timer OFF"))
            .clicked()
        {
            self.timer.mode = NoTime;
        }
        ui.separator();
        ui.label("Bullet");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 20.0,
                        increment: 1.0,
                        active: false,
                        mode: GameMode::Bullet,
                        white_time: 20.0,
                        black_time: 20.0,
                        start_of_turn: (0.0, None),
                    },
                    "0:20 + 1",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 30.0,
                        increment: 0.0,
                        active: false,
                        mode: GameMode::Bullet,
                        white_time: 30.0,
                        black_time: 30.0,
                        start_of_turn: (0.0, None),
                    },
                    "0:30 + 0",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 60.0,
                        increment: 0.0,
                        active: false,
                        mode: GameMode::Bullet,
                        white_time: 60.0,
                        black_time: 60.0,
                        start_of_turn: (0.0, None),
                    },
                    "1:00 + 0",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 60.0,
                        increment: 1.0,
                        active: false,
                        mode: GameMode::Bullet,
                        white_time: 60.0,
                        black_time: 60.0,
                        start_of_turn: (0.0, None),
                    },
                    "1:00 + 1",
                );
            });
        });
        ui.separator();
        ui.label("Blitz");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 180.0,
                        increment: 2.0,
                        active: false,
                        mode: Blitz,
                        white_time: 180.0,
                        black_time: 180.0,
                        start_of_turn: (0.0, None),
                    },
                    "3:00 + 2",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 300.0,
                        increment: 0.0,
                        active: false,
                        mode: Blitz,
                        white_time: 300.0,
                        black_time: 300.0,
                        start_of_turn: (0.0, None),
                    },
                    "5:00 + 0",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 300.0,
                        increment: 5.0,
                        active: false,
                        mode: Blitz,
                        white_time: 300.0,
                        black_time: 300.0,
                        start_of_turn: (0.0, None),
                    },
                    "5:00 + 5",
                );
            });
        });
        ui.separator();
        ui.label("Rapid");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 600.0,
                        increment: 0.0,
                        active: false,
                        mode: Rapid,
                        white_time: 600.0,
                        black_time: 600.0,
                        start_of_turn: (0.0, None),
                    },
                    "10:00 + 0",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 600.0,
                        increment: 5.0,
                        active: false,
                        mode: Rapid,
                        white_time: 600.0,
                        black_time: 600.0,
                        start_of_turn: (0.0, None),
                    },
                    "10:00 + 5",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 900.0,
                        increment: 10.0,
                        active: false,
                        mode: Rapid,
                        white_time: 900.0,
                        black_time: 900.0,
                        start_of_turn: (0.0, None),
                    },
                    "15:00 + 10",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 1800.0,
                        increment: 0.0,
                        active: false,
                        mode: Rapid,
                        white_time: 1800.0,
                        black_time: 1800.0,
                        start_of_turn: (0.0, None),
                    },
                    "30:00 + 0",
                );
                ui.selectable_value(
                    &mut self.timer,
                    Timer {
                        start: 3600.0,
                        increment: 0.0,
                        active: false,
                        mode: Rapid,
                        white_time: 3600.0,
                        black_time: 3600.0,
                        start_of_turn: (0.0, None),
                    },
                    "60:00 + 0",
                );
            });
        });
        ui.separator();
        // ui.horizontal(|ui| {
        ui.selectable_value(&mut self.timer.mode, Custom, "Custom");

        if self.timer.mode == Custom {
            ui.horizontal(|ui| {
                ui.label("Time :");
                ui.menu_button(
                    format!("{:.0} min", (self.timer.start / 60.0).floor(),),
                    |ui| {
                        if ui
                            .selectable_value(&mut self.timer.start, 20.0, " 0:20")
                            .clicked()
                        {
                            self.timer.white_time = 20.0;
                            self.timer.black_time = 20.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 30.0, " 0:30")
                            .clicked()
                        {
                            self.timer.white_time = 30.0;
                            self.timer.black_time = 30.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 60.0, " 1:00")
                            .clicked()
                        {
                            self.timer.white_time = 60.0;
                            self.timer.black_time = 60.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 180.0, " 3:00")
                            .clicked()
                        {
                            self.timer.white_time = 180.0;
                            self.timer.black_time = 180.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 300.0, " 5:00")
                            .clicked()
                        {
                            self.timer.white_time = 300.0;
                            self.timer.black_time = 300.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 600.0, "10:00")
                            .clicked()
                        {
                            self.timer.white_time = 600.0;
                            self.timer.black_time = 600.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 900.0, "15:00")
                            .clicked()
                        {
                            self.timer.white_time = 900.0;
                            self.timer.black_time = 900.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 1800.0, "30:00")
                            .clicked()
                        {
                            self.timer.white_time = 1800.0;
                            self.timer.black_time = 1800.0;
                        }
                        if ui
                            .selectable_value(&mut self.timer.start, 3600.0, "60:00")
                            .clicked()
                        {
                            self.timer.white_time = 3600.0;
                            self.timer.black_time = 3600.0;
                        }
                    },
                );
            });
            ui.horizontal(|ui| {
                ui.label("Increment :");
                ui.menu_button(format!("{:.0} sec", self.timer.increment,), |ui| {
                    if ui.button("None").clicked() {
                        self.timer.increment = 0.0;
                    }
                    if ui.button("1 sec").clicked() {
                        self.timer.increment = 1.0;
                    }
                    if ui.button("2 sec").clicked() {
                        self.timer.increment = 2.0;
                    }
                    if ui.button("10 sec").clicked() {
                        self.timer.increment = 10.0;
                    }
                });
            });
            ui.separator();
        }
    }
}
