use crate::ChessApp;
use crate::gui::chessapp_struct::MobileGameMode::*;

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
                ui.selectable_value(&mut self.mobile_timer.mode, Bullet, "0:20 + 1");
                ui.selectable_value(&mut self.mobile_timer.mode, Bullet, "0:30 + 0");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.mobile_timer.mode, Bullet, " 1:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Bullet, " 1:00 + 1");
            });
        });
        ui.separator();
        ui.label("Blitz");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.mobile_timer.mode, Blitz, "3:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Blitz, "3:00 + 2");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.mobile_timer.mode, Blitz, "5:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Blitz, "5:00 + 5");
            });
        });
        ui.selectable_value(&mut self.mobile_timer.mode, Blitz, "5:00 + 2");
        ui.separator();
        ui.label("Rapid");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.mobile_timer.mode, Rapid, " 10:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Rapid, " 10:00 + 5");
                ui.selectable_value(
                    &mut self.mobile_timer.mode,
                    Rapid,
                    "15:00 + 10",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.mobile_timer.mode, Rapid, "20:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Rapid, "30:00 + 0");
                ui.selectable_value(&mut self.mobile_timer.mode, Rapid, "60:00 + 0");
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
