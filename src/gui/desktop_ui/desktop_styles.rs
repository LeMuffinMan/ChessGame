use crate::ChessApp;

use egui::FontId;
use egui::TextStyle;

impl ChessApp {
    pub fn apply_desktop_styles(&mut self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(50.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Body,
                FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Monospace,
                FontId::new(18.0, egui::FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(18.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        ctx.set_style(style);
    }
}
