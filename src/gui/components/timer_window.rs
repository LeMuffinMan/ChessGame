use crate::ChessApp;

impl ChessApp {
    pub fn timer_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Timer")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -365.0])
            .show(ctx, |ui| {
                self.timer_increment(ui, ctx);
                ui.separator();
                ui.vertical_centered(|ui| {
                    if ui.button("Save").clicked() {
                        self.win = None;
                    }
                });
            });
    }
}
