
use crate::ChessApp;
// use crate::gui::chessapp_struct::GameMode::*;
use crate::gui::chessapp_struct::MobileTimer;
use crate::gui::chessapp_struct::MobileGameMode;
use crate::gui::chessapp_struct::MobileGameMode::*;

use egui::RichText;


impl ChessApp {
    pub fn set_timer(&mut self, ctx: &egui::Context) {
        egui::Window::new("Timer")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                if self.mobile_timer.mode != Custom {
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(355.0);
                        if ui.add_enabled(self.mobile_timer.mode != MobileGameMode::Custom, egui::Button::new("Custom"))
                        .clicked() {
                            self.mobile_timer.mode = Custom;
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
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 20.0, increment: 1.0, active: false, mode: MobileGameMode::Bullet }, "0:20 + 1");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 30.0, increment: 0.0, active: false, mode: MobileGameMode::Bullet }, "0:30 + 0");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 60.0, increment: 0.0, active: false, mode: MobileGameMode::Bullet }, "1:00 + 0");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 60.0, increment: 1.0, active: false, mode: MobileGameMode::Bullet }, "1:00 + 1");
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
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 180.0, increment: 2.0, active: false, mode: Blitz }, "3:00 + 2");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 300.0, increment: 0.0, active: false, mode: Blitz }, "5:00 + 0");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer {start: 300.0, increment: 5.0, active: false, mode: Blitz }, "5:00 + 5");
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
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer { start: 600.0, increment: 0.0, active: false, mode: Rapid }, "10:00 + 0");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer { start: 600.0, increment: 5.0, active: false, mode: Rapid }, "10:00 + 5");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer { start: 900.0, increment: 10.0, active: false, mode: Rapid }, "15:00 + 10");
                                ui.selectable_value(&mut self.mobile_timer, MobileTimer { start: 1800.0, increment: 0.0, active: false, mode: Rapid }, "30:00 + 0");
                            });
                        });
                        ui.add_space(col_width / 5.0);
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(230.0);
                        if ui.button("Save timer").clicked() {
                            self.mobile_win = None;
                        } 
                        if ui.button("Timer OFF").clicked() {
                            self.mobile_timer.mode = MobileGameMode::NoTime; 
                            self.mobile_win = None;
                        } 
                        ui.add_space(40.0);
                    });
                    ui.add_space(40.0);
                } else {
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                    ui.add_space(180.0);
                        if ui.add_enabled(self.mobile_timer.mode == MobileGameMode::Custom, egui::Button::new("Presets"))
                        .clicked()  {
                            self.mobile_timer.mode = MobileGameMode::NoTime; 
                        }
                    });
                    ui.add_space(60.0);
                    ui.horizontal_centered(|ui| {
                        ui.add_space(60.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(" Time").size(40.0));
                            ui.add_space(20.0);
                            ui.selectable_value(&mut self.mobile_timer.start, 20.0, " 0:20");
                            ui.selectable_value(&mut self.mobile_timer.start, 30.0, " 0:30");
                            ui.selectable_value(&mut self.mobile_timer.start, 60.0, " 1:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 180.0, " 3:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 300.0, " 5:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 600.0, "10:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 900.0, "15:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 1800.0, "30:00");
                            ui.selectable_value(&mut self.mobile_timer.start, 3600.0, "60:00");
                        });
                        ui.add_space(60.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(" Increment").size(40.0));
                            ui.add_space(20.0);
                            ui.selectable_value(&mut self.mobile_timer.increment, 0.0, "     0 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 1.0, "     1 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 2.0, "     2 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 3.0, "     3 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 5.0, "     5 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 10.0, "    10 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 20.0, "    15 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 30.0, "    30 sec");
                            ui.selectable_value(&mut self.mobile_timer.increment, 45.0, "    30 sec");
                        });
                        ui.add_space(60.0);
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.add_space(60.0);
                        if ui.button("Save timer").clicked() {
                            self.mobile_win = None;
                        } 
                        if ui.button("Timer OFF").clicked() {
                            self.mobile_timer.mode = MobileGameMode::NoTime; 
                            self.mobile_win = None;
                        } 
                        ui.add_space(40.0);
                    });
                    ui.add_space(40.0);
                }
        });
    }
}
