use crate::ChessApp;
use crate::gui::chessapp_struct::UiType;

pub struct ReplayInfos {
    pub index: usize,
    pub sec_per_frame: f64,
    pub next_step: Option<f64>,
}

impl ReplayInfos {
    pub fn new() -> Self {
        Self {
            index: 0,
            sec_per_frame: 1.0,
            next_step: None,
        }
    }
}

impl ChessApp {
    pub fn replay_buttons(&mut self, ui: &mut egui::Ui) {
        match &self.ui_type {
            UiType::Mobile => {
                ui.add_space(60.0);
                self.settings_button(ui);
                ui.add_space(50.0);

                self.first_state(ui);
                ui.add_space(25.0);
                self.prev_state(ui);
                ui.add_space(25.0);
                self.play_pause(ui);
                ui.add_space(25.0);
                self.next_state(ui);
                ui.add_space(25.0);
                self.last_state(ui);
                ui.add_space(50.0);
                self.new_game_button(ui);
            }
            UiType::Desktop => {
                ui.horizontal(|ui| {
                    self.first_state(ui);
                    ui.add_space(10.0);
                    self.prev_state(ui);
                    ui.add_space(10.0);
                    self.play_pause(ui);
                    ui.add_space(10.0);
                    self.next_state(ui);
                    ui.add_space(10.0);
                    self.last_state(ui);
                });
            }
        }
    }

    fn first_state(&mut self, ui: &mut egui::Ui) {
        if ui.button("|<").clicked() {
            self.replay_infos.index = 0;
            self.current = self.history.snapshots[0].clone();
        }
    }

    fn prev_state(&mut self, ui: &mut egui::Ui) {
        if ui.button("<").clicked() && self.replay_infos.index > 0 {
            self.replay_infos.index -= 1;
            self.current = self.history.snapshots[self.replay_infos.index].clone();
        }
    }

    fn play_pause(&mut self, ui: &mut egui::Ui) {
        if self.replay_infos.next_step.is_none() {
            if ui
                .add_enabled(self.win.is_none(), egui::Button::new("▶"))
                .clicked()
            {
                self.replay_infos.index = 0;
                self.current = self.history.snapshots[0].clone();

                let now = ui.input(|i| i.time);
                let delay = self.replay_infos.sec_per_frame;
                self.replay_infos.next_step = Some(now + delay);
            }
        } else if ui
            .add_enabled(self.win.is_none(), egui::Button::new("⏸"))
            .clicked()
        {
            self.replay_infos.next_step = None;
        }
    }

    fn next_state(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.replay_infos.index < self.history.snapshots.len() - 1,
                egui::Button::new(">"),
            )
            .clicked()
            && self.replay_infos.index < self.history.snapshots.len() - 1
        {
            self.replay_infos.index += 1;
            self.current = self.history.snapshots[self.replay_infos.index].clone();
        }
    }

    fn last_state(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.replay_infos.index < self.history.snapshots.len() - 1,
                egui::Button::new(">|"),
            )
            .clicked()
        {
            self.replay_infos.index = self.history.snapshots.len() - 1;
            self.current = self.history.snapshots[self.replay_infos.index].clone();
        }
    }
}
