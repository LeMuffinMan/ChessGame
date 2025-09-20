use crate::ChessApp;
use crate::gui::update_timer::GameMode;
use crate::gui::update_timer::GameMode::*;
use crate::gui::update_timer::Timer;

use egui::RichText;

impl ChessApp {
    pub fn set_timer(&mut self, ctx: &egui::Context) {
        egui::Window::new("Timer")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                if self.timer.mode != Custom {
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(355.0);
                        if ui
                            .add_enabled(
                                self.timer.mode != GameMode::Custom,
                                egui::Button::new("Custom"),
                            )
                            .clicked()
                        {
                            self.timer.mode = Custom;
                        }
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        let col_width = total_width / 3.0;

                        ui.add_space(col_width / 5.0);
                        ui.add_space(20.0);
                        // Bullet
                        ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new("  Bullet").size(40.0));
                                ui.add_space(20.0);
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
                        ui.add_space(col_width / 6.0);
                        ui.separator();
                        ui.add_space(col_width / 6.0);
                        // Blitz
                        ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new("   Blitz").size(40.0));
                                ui.add_space(20.0);
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
                        ui.add_space(col_width / 6.0);
                        ui.separator();
                        ui.add_space(col_width / 6.0);
                        // Rapid
                        ui.allocate_ui(egui::Vec2::new(col_width, ui.available_height()), |ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new("    Rapid").size(40.0));
                                ui.add_space(20.0);
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
                            });
                        });
                        ui.add_space(col_width / 5.0);
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(230.0);
                        if ui.button("Save timer").clicked() {
                            self.win = None;
                        }
                        if ui.button("Timer OFF").clicked() {
                            self.timer.mode = GameMode::NoTime;
                            self.win = None;
                        }
                        ui.add_space(40.0);
                    });
                    ui.add_space(40.0);
                } else {
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(180.0);
                        if ui
                            .add_enabled(
                                self.timer.mode == GameMode::Custom,
                                egui::Button::new("Presets"),
                            )
                            .clicked()
                        {
                            self.timer.mode = GameMode::NoTime;
                        }
                    });
                    ui.add_space(60.0);
                    ui.horizontal_centered(|ui| {
                        ui.add_space(60.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(" Time").size(40.0));
                            ui.add_space(20.0);
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
                        });
                        ui.add_space(60.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(" Increment").size(40.0));
                            ui.add_space(20.0);
                            ui.selectable_value(
                                &mut self.timer.increment,
                                0.0,
                                "     0 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                1.0,
                                "     1 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                2.0,
                                "     2 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                3.0,
                                "     3 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                5.0,
                                "     5 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                10.0,
                                "    10 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                20.0,
                                "    15 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                30.0,
                                "    30 sec",
                            );
                            ui.selectable_value(
                                &mut self.timer.increment,
                                45.0,
                                "    30 sec",
                            );
                        });
                        ui.add_space(60.0);
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(60.0);
                        if ui.button("Save timer").clicked() {
                            self.win = None;
                        }
                        if ui.button("Timer OFF").clicked() {
                            self.timer.mode = GameMode::NoTime;
                            self.win = None;
                        }
                        ui.add_space(40.0);
                    });
                    ui.add_space(40.0);
                }
            });
    }
}
