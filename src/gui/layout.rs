use crate::gui::chessapp::ChessApp;

#[derive(Clone, PartialEq)]
pub enum UiType {
    Desktop,
    Mobile,
}

impl ChessApp {
    pub fn mobile_layout(&mut self, ctx: &egui::Context) {
        self.apply_styles(ctx);
        self.top_title_panel(ctx);
        self.central_panel(ctx);
    }
    pub fn desktop_layout(&mut self, ctx: &egui::Context) {
        self.apply_desktop_styles(ctx);
        self.top_title_panel(ctx);
        self.bot_source_code_panel_desktop(ctx);
        self.left_panel_desktop(ctx);
        self.right_panel_desktop(ctx);
        self.top_black_panel_desktop(ctx);
        self.bot_white_panel_desktop(ctx);
        self.central_panel(ctx);
    }
}
