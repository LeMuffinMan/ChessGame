use crate::ChessApp;
use crate::gui::chessapp_struct::History;
// use crate::gui::desktop_ui::bot_panels::format_time;
// use crate::gui::update_timer::GameMode;

// use chrono::Utc;
// use js_sys::Array;
// use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
// use web_sys::{Blob, HtmlAnchorElement, HtmlElement, Url, window};

impl ChessApp {

    pub fn import_pgn(&mut self) {
        let history = History::new(); 
        let lines: Vec<String> = self.pgn_input.split('\n').collect();
        if lines.iter().any(|l| l == "\n") {
            match history.parse_headers(lines) {
                Err(e) => log::debug!("parse_headers : e"),
                _ => {},
            };
            match history.parse_moves(lines) {
                Err(e) => log::debug!("parse_move : e"),
                _ => {},
            };
        } else {
            log ::debug!("No new line found to separate headers and san code");
        }
    }
}

impl History {
    fn parse_headers(&mut self, lines: Vec<String>) {

    }
}
