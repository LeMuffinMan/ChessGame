use crate::ChessApp;
use crate::gui::features::timer::GameMode::*;
use crate::gui::features::timer::Timer;

use egui::Context;

impl ChessApp {
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        self.timer_header(ui);

        if self.timer.mode == Custom {
            self.timer_time_selector(ui);
            self.timer_increment_selector(ui);
            ui.separator();
        }
    }
    fn increment_option(&mut self, ui: &mut egui::Ui, value: f64, label: &str) {
        if ui.button(label).clicked() {
            self.timer.increment = value;
        }
    }
    fn timer_increment_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Increment :");
            ui.menu_button(format!("{:.0} sec", self.timer.increment), |ui| {
                self.increment_option(ui, 0.0, "None");
                self.increment_option(ui, 1.0, "1 sec");
                self.increment_option(ui, 2.0, "2 sec");
                self.increment_option(ui, 10.0, "10 sec");
            });
        });
    }

    fn time_option(&mut self, ui: &mut egui::Ui, value: f64, label: &str) {
        if ui
            .selectable_value(&mut self.timer.start, value, label)
            .clicked()
        {
            self.timer.white_time = value;
            self.timer.black_time = value;
        }
    }

    fn timer_time_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Time :");
            ui.menu_button(
                format!("{:.0} min", (self.timer.start / 60.0).floor()),
                |ui| {
                    self.time_option(ui, 20.0, " 0:20");
                    self.time_option(ui, 30.0, " 0:30");
                    self.time_option(ui, 60.0, " 1:00");
                    self.time_option(ui, 180.0, " 3:00");
                    self.time_option(ui, 300.0, " 5:00");
                    self.time_option(ui, 600.0, "10:00");
                    self.time_option(ui, 900.0, "15:00");
                    self.time_option(ui, 1800.0, "30:00");
                    self.time_option(ui, 3600.0, "60:00");
                },
            );
        });
    }
    fn timer_header(&mut self, ui: &mut egui::Ui) {
        if self.timer_switch(ui) {
            self.timer.mode = NoTime;
        }
        ui.separator();
        self.timer_presets(ui);
    }
    fn timer_switch(&mut self, ui: &mut egui::Ui) -> bool {
        ui.add_enabled(self.timer.mode != NoTime, egui::Button::new("Timer OFF"))
            .clicked()
    }

    fn timer_presets(&mut self, ui: &mut egui::Ui) {
        self.bullet_presets(ui);
        self.blitz_presets(ui);
        self.rapid_presets(ui);
        ui.selectable_value(&mut self.timer.mode, Custom, "Custom");
    }

    fn bullet_presets(&mut self, ui: &mut egui::Ui) {
        ui.label("Bullet");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(20.0, 1.0, Bullet), "0:20 + 1");
                ui.selectable_value(&mut self.timer, Timer::new(30.0, 0.0, Bullet), "0:30 + 1");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(60.0, 0.0, Bullet), "1:00 + 0");
                ui.selectable_value(&mut self.timer, Timer::new(60.0, 1.0, Bullet), "1:00 + 1");
            });
        });
        ui.separator();
    }

    fn blitz_presets(&mut self, ui: &mut egui::Ui) {
        ui.label("Blitz");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(180.0, 2.0, Blitz), "3:00 + 2");
                ui.selectable_value(&mut self.timer, Timer::new(300.0, 0.0, Blitz), "5:00 + 0");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(300.0, 5.0, Blitz), "5:00 + 5");
            });
        });
        ui.separator();
    }

    fn rapid_presets(&mut self, ui: &mut egui::Ui) {
        ui.label("Rapid");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(600.0, 0.0, Rapid), "10:00 + 0");
                ui.selectable_value(&mut self.timer, Timer::new(600.0, 5.0, Rapid), "10:00 + 5");
                ui.selectable_value(
                    &mut self.timer,
                    Timer::new(900.0, 10.0, Rapid),
                    "15:00 + 10",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.timer, Timer::new(1800.0, 0.0, Rapid), "30:00 + 0");
                ui.selectable_value(&mut self.timer, Timer::new(3600.0, 0.0, Rapid), "60:00 + 0");
            });
        });
        ui.separator();
    }
}
