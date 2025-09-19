use crate::ChessApp;
use crate::gui::chessapp_struct::MobileGameMode::*;
use crate::gui::chessapp_struct::MobileTimer;
use crate::gui::chessapp_struct::MobileGameMode;

use egui::Context;

//a refacto
impl ChessApp {
    //This shows the list a presets for timings
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        if ui
            .add_enabled(self.mobile_timer.active, egui::Button::new("Timer OFF"))
            .clicked()
        {
            self.mobile_timer.mode = NoTime;
        }
        ui.separator();
        ui.label("Bullet");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.mobile_timer,
                    MobileTimer {
                        start: 20.0,
                        increment: 1.0,
                        active: false,
                        mode: MobileGameMode::Bullet,
                        white_time: 20.0,
                        black_time: 20.0,
                        start_of_turn: (0.0, None),
                    },
                    "0:20 + 1",
                );
                ui.selectable_value(
                    &mut self.mobile_timer,
                    MobileTimer {
                        start: 30.0,
                        increment: 0.0,
                        active: false,
                        mode: MobileGameMode::Bullet,
                        white_time: 30.0,
                        black_time: 30.0,
                        start_of_turn: (0.0, None),
                    },
                    "0:30 + 0",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(
                    &mut self.mobile_timer,
                    MobileTimer {
                        start: 60.0,
                        increment: 0.0,
                        active: false,
                        mode: MobileGameMode::Bullet,
                        white_time: 60.0,
                        black_time: 60.0,
                        start_of_turn: (0.0, None),
                    },
                    "1:00 + 0",
                );
                ui.selectable_value(
                    &mut self.mobile_timer,
                    MobileTimer {
                        start: 60.0,
                        increment: 1.0,
                        active: false,
                        mode: MobileGameMode::Bullet,
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                        &mut self.mobile_timer,
                        MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
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
                    &mut self.mobile_timer,
                    MobileTimer {
                        start: 3600.0,
                        increment: 0.0,
                        active: false,
                        mode: Rapid,
                        white_time: 1800.0,
                        black_time: 1800.0,
                        start_of_turn: (0.0, None),
                    },
                    "60:00 + 0",
                );
            });
        });
        ui.separator();
        // ui.horizontal(|ui| {
        ui.selectable_value(&mut self.mobile_timer.mode, Custom, "Custom");

        if self.mobile_timer.mode == Custom {
            ui.horizontal(|ui| {
                ui.label("Time :");
                ui.menu_button(format!("{:.0} min", (self.mobile_timer.start / 60.0).floor(),), |ui| {
                    ui.selectable_value(&mut self.mobile_timer.start, 20.0, " 0:20");
                    ui.selectable_value(&mut self.mobile_timer.start, 30.0, " 0:30");
                    ui.selectable_value(&mut self.mobile_timer.start, 60.0, " 1:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 180.0, " 3:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 300.0, " 5:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 600.0, "10:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 900.0, "15:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 1800.0, "30:00");
                    ui.selectable_value(&mut self.mobile_timer.start, 3600.0, "60:00");                });
            });
            ui.horizontal(|ui| {
                ui.label("Increment :");
                ui.menu_button(format!("{:.0} sec", self.mobile_timer.increment,), |ui| {
                    if ui.button("None").clicked() {
                        self.mobile_timer.increment = 0.0;
                    }
                    if ui.button("1 sec").clicked() {
                        self.mobile_timer.increment = 1.0;
                    }
                    if ui.button("2 sec").clicked() {
                        self.mobile_timer.increment = 2.0;
                    }
                    if ui.button("10 sec").clicked() {
                        self.mobile_timer.increment = 10.0;
                    }
                });
            });
            ui.separator();
        }
    }
}
