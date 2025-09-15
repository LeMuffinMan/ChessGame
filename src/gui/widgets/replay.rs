use crate::ChessApp;
use crate::gui::chessapp_struct::GameMode;
use crate::gui::chessapp_struct::GameMode::*;

use chrono::Utc;
use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Blob;
use web_sys::HtmlAnchorElement;
use web_sys::Url;
use web_sys::window;
// use crate::pgn::encode_pgn::export_pgn;

pub struct Timer {
    pub white: (Option<f64>, f64), //start of turn, remaining time
    pub black: (Option<f64>, f64),
    pub increment: f64,
}

impl Timer {
    pub fn build(game_mode: Option<GameMode>) -> Option<Self> {
        match game_mode {
            Some(Bullet(time, inc))
            | Some(Blitz(time, inc))
            | Some(Rapid(time, inc))
            | Some(Custom(time, inc)) => Some(Self {
                white: (None, time),
                black: (None, time),
                increment: inc,
            }),
            None => None,
        }
    }
}

impl ChessApp {
    pub fn export_pgn(&mut self) -> Result<(), JsValue> {
        let mut pgn = String::new();
        //ajouter type de game et rounds
        //time control
        //termination ?
        pgn.push_str("[Event \"Casual Game\"]\n[Site \"https://lemuffinman.github.io/ChessGame/\"]\n[UTCDate \"");
        pgn.push_str(Utc::now().format("%Y.%m.%d").to_string().as_str());
        pgn.push_str("\"]\n[UTCTime \"]");
        pgn.push_str(Utc::now().format("%H.%M.%S").to_string().as_str());
        pgn.push_str("\"]\n[White \"");
        pgn.push_str(&self.white_name.to_string());
        pgn.push_str("\"]\n[Black \"");
        pgn.push_str(&self.black_name.to_string());
        pgn.push_str("\"]\n");
        if let Some(result) = self.history_san.split_whitespace().last() {
            pgn.push_str("[Result : \"");
            if result == "0-1" || result == "1-0" || result == "1/2 - 1/2" {
                pgn.push_str(result);
            } else {
                pgn.push('*');
            }
            pgn.push_str("\"]\n");
        }
        pgn.push_str("\n");
        pgn.push_str(&self.history_san);
        //ici split l'history en plusieurs lignes de 80 chars
        pgn.push('\n');

        // Wrap PGN into a Blob
        let parts = Array::new();
        parts.push(&JsValue::from_str(&pgn));
        let blob = Blob::new_with_str_sequence(&parts.into())?;

        // Create object URL
        let url = Url::create_object_url_with_blob(&blob)?;

        // Access document
        let window = window().unwrap();
        let doc = window.document().unwrap();

        // Create <a> element
        let link = doc.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;
        link.set_href(&url);
        link.set_download("chessgame.pgn");
        link.style().set_property("display", "none")?;
        doc.body().unwrap().append_child(&link)?;

        // Trigger download
        link.click();

        // Cleanup
        doc.body().unwrap().remove_child(&link)?;
        Url::revoke_object_url(&url)?;

        Ok(())
    }

    pub fn replay_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.history.is_empty(), egui::Button::new("Save"))
                .clicked()
            {
                self.win_save = true;
            }
            let can_undo = self.widgets.replay_index != 0;
            let can_redo = self.widgets.replay_index + 1 < self.history.len();
            let can_replay = can_undo && self.widgets.next_replay_time.is_none();
            self.rewind(ui, can_undo);
            if can_replay {
                if ui
                    .add_enabled(!self.win_dialog, egui::Button::new("▶"))
                    .clicked()
                {
                    self.widgets.replay_index = 0;
                    self.current = self.history[0].clone();

                    let now = ui.input(|i| i.time);
                    let delay = self.widgets.replay_speed;
                    self.widgets.next_replay_time = Some(now + delay);
                }
            } else if self.widgets.next_replay_time.is_some() {
                if ui
                    .add_enabled(!self.win_dialog, egui::Button::new("⏸"))
                    .clicked()
                {
                    self.widgets.next_replay_time = None;
                }
            } else {
                ui.add_enabled(false, egui::Button::new("▶")).clicked();
            }
            self.replay_step(ui.ctx());
            self.redo(ui, can_redo);
            ui.add_enabled(false, egui::Button::new("Load"));
        });
        ui.separator();
        if self.widgets.next_replay_time.is_some() {
            ui.add(
                egui::Slider::new(&mut self.widgets.replay_speed, 0.1..=10.0)
                    .text("sec/move")
                    .logarithmic(true),
            );
        }
    }

    pub fn rewind(&mut self, ui: &mut egui::Ui, can_undo: bool) {
        if ui
            .add_enabled(can_undo && !self.win_dialog, egui::Button::new("|<"))
            .clicked()
        {
            self.widgets.replay_index = 0;
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
        if ui
            .add_enabled(can_undo && !self.win_dialog, egui::Button::new("<"))
            .clicked()
        {
            if self.widgets.replay_index == self.history.len() {
                self.widgets.replay_index -= 1;
            }
            self.widgets.replay_index -= 1;
            self.current = self.history[self.widgets.replay_index].clone();
            self.highlight.piece_legals_moves.clear();
        }
    }

    pub fn redo(&mut self, ui: &mut egui::Ui, can_redo: bool) {
        if ui.add_enabled(can_redo, egui::Button::new(">")).clicked() {
            self.widgets.replay_index += 1;
            self.current = self.history[self.widgets.replay_index].clone();
            if self.widgets.replay_index == self.history.len() - 1 {
                self.widgets.replay_index += 1;
            }
            self.highlight.piece_legals_moves.clear();
        }
        if ui.add_enabled(can_redo, egui::Button::new(">|")).clicked() {
            self.widgets.replay_index = self.history.len() - 1;
            self.current = self.history[self.widgets.replay_index].clone();
            self.widgets.replay_index += 1;
            self.highlight.piece_legals_moves.clear();
        }
    }

    fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.widgets.next_replay_time {
            let now = ctx.input(|i| i.time);
            if now >= next_time {
                if self.widgets.replay_index + 1 < self.history.len() {
                    self.widgets.replay_index += 1;
                    // log::debug!("Replay index = {}", self.widgets.replay_index);
                    self.current = self.history[self.widgets.replay_index].clone();
                    let delay = self.widgets.replay_speed;
                    self.widgets.next_replay_time = Some(now + delay);
                } else {
                    self.widgets.replay_index = self.history.len();
                    self.widgets.next_replay_time = None;
                }
            }
        }
        ctx.request_repaint();
    }
}
