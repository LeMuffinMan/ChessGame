use crate::ChessApp;
use crate::gui::chessapp_struct::GameMode;
use crate::gui::chessapp_struct::GameMode::*;

use egui::Context;

//a refacto
impl ChessApp {
    //This shows the list a presets for timings
    pub fn timer_increment(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        if ui
            .add_enabled(self.widgets.timer.is_some(), egui::Button::new("Timer OFF"))
            .clicked()
        {
            self.widgets.game_mode = NoTime(0.0, 0.0);
            self.widgets.timer = None;
        }
        ui.separator();
        ui.label("Bullet");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Bullet(20.0, 1.0), "0:20 + 1");
                ui.selectable_value(&mut self.widgets.game_mode, Bullet(30.0, 0.0), "0:30 + 0");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Bullet(60.0, 0.0), " 1:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Bullet(60.0, 1.0), " 1:00 + 1");
            });
        });
        ui.separator();
        ui.label("Blitz");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Blitz(180.0, 0.0), "3:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Blitz(180.0, 2.0), "3:00 + 2");
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 0.0), "5:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 5.0), "5:00 + 5");
            });
        });
        ui.selectable_value(&mut self.widgets.game_mode, Blitz(300.0, 2.0), "5:00 + 2");
        ui.separator();
        ui.label("Rapid");
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Rapid(600.0, 0.0), " 10:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Rapid(600.0, 5.0), " 10:00 + 5");
                ui.selectable_value(
                    &mut self.widgets.game_mode,
                    Rapid(900.0, 10.0),
                    "15:00 + 10",
                );
            });
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.widgets.game_mode, Rapid(1200.0, 0.0), "20:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Rapid(1800.0, 0.0), "30:00 + 0");
                ui.selectable_value(&mut self.widgets.game_mode, Rapid(3600.0, 5.0), "60:00 + 0");
            });
        });
        ui.separator();
        // ui.horizontal(|ui| {
        ui.selectable_value(&mut self.widgets.game_mode, Custom(600.0, 0.0), "Custom");
        self.widgets.timer = None;
        if let GameMode::Custom(mut time, mut inc) = self.widgets.game_mode {
            ui.horizontal(|ui| {
                ui.label("Time :");
                ui.menu_button(format!("{:.0} min", (time / 60.0).floor(),), |ui| {
                    if ui.button("1 min").clicked() {
                        time = 60.0;
                    }
                    if ui.button("3 min").clicked() {
                        time = 180.0;
                    }
                    if ui.button("5 min").clicked() {
                        time = 300.0;
                    }
                    if ui.button("10 min").clicked() {
                        time = 600.0;
                    }
                    if ui.button("15 min").clicked() {
                        time = 900.0;
                    }
                    if ui.button("30 min").clicked() {
                        time = 1800.0;
                    }
                });
            });
            ui.horizontal(|ui| {
                ui.label("Increment :");
                ui.menu_button(format!("{:.0} sec", inc,), |ui| {
                    if ui.button("None").clicked() {
                        inc = 0.0;
                    }
                    if ui.button("1 sec").clicked() {
                        inc = 1.0;
                    }
                    if ui.button("2 sec").clicked() {
                        inc = 2.0;
                    }
                    if ui.button("10 sec").clicked() {
                        inc = 10.0;
                    }
                });
            });
            ui.separator();
        }
    }
}
