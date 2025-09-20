use crate::ChessApp;
use crate::gui::update_timer::GameMode;
use crate::gui::desktop_ui::bot_panels::format_time;

use chrono::Utc;
use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Blob, HtmlAnchorElement, HtmlElement, Url, window};

impl ChessApp {
    //error handling todo
    pub fn export_pgn(&mut self) -> Result<(), JsValue> {
        let pgn = self.generate_pgn_content();
        self.url_with_blob_export(pgn)
    }
    //error handling
    //Documenter
    fn url_with_blob_export(&mut self, pgn: String) -> Result<(), JsValue> {
        //we create a Js to build the sequence needed for the blob
        let parts = Array::new();
        parts.push(&JsValue::from_str(&pgn));

        //we create a Binary Large OBject representing our pgn content, in memory
        let blob = Blob::new_with_str_sequence(&parts.into())?;

        //return a string of type blob:<origin>/<id> usable as href.
        //needed to reference the blob Blob from the Document Object Model.
        let url = Url::create_object_url_with_blob(&blob)?;

        // get window and docuemnt
        // unwrap is not enough we need to handle it here
        let window = window().unwrap();
        let doc = window.document().unwrap();

        //we create the "link" to our blob
        let link = doc.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;
        link.set_href(&url);
        link.set_download("chessgame.pgn");

        //hide the link we created
        let elem: &HtmlElement = link.unchecked_ref(); // cast en HtmlElement
        elem.style().set_property("display", "none")?;
        //put the element in the DOM
        doc.body().unwrap().append_child(&link)?;
        // trigger download, as if we clicked a download link
        link.click();

        // delete the DOM link for cleaning
        doc.body().unwrap().remove_child(&link)?;
        //free blob browser ressources alocated
        Url::revoke_object_url(&url)?;

        Ok(())
    }

    fn generate_pgn_content(&mut self) -> String {
        let mut pgn = String::new();
        pgn.push_str("[Event \"Casual Game\"]\n[Site \"https://lemuffinman.github.io/ChessGame/\"]\n[UTCDate \"");
        pgn.push_str(Utc::now().format("%Y.%m.%d").to_string().as_str());
        pgn.push_str("\"]\n[UTCTime \"");
        pgn.push_str(Utc::now().format("%H.%M.%S").to_string().as_str());
        pgn.push_str("\"]\n[White \"");
        pgn.push_str(&self.white_name.to_string());
        pgn.push_str("\"]\n[Black \"");
        pgn.push_str(&self.black_name.to_string());
        pgn.push_str("\"]\n");
        if let Some(result) = self.history_san.split_whitespace().last() {
            pgn.push_str("[Result : \"");
            match result {
                "0-1" | "1-0" | "1/2 - 1/2" => {
                    pgn.push_str(result);
                }
                _ => {
                    pgn.push('*');
                }
            }
            pgn.push_str("\"]\n");
        }
        pgn.push_str("[TimeControl \"");
        match &self.timer.mode {
            GameMode::NoTime => {
                pgn.push_str("No time\"]\n");
            }, 
            GameMode::Rapid => {
                pgn.push_str(&format!(
                    "Rapid: {} + {}\"]\n",
                    format_time(self.timer.start),
                    self.timer.increment
                ));

            }, 
            GameMode::Blitz => {
                pgn.push_str(&format!(
                    "Blitz: {} + {}\"]\n",
                    format_time(self.timer.start),
                    self.timer.increment
                ));

            },
            GameMode::Bullet => {
                pgn.push_str(&format!(
                    "Bullet: {} + {}\"]\n",
                    format_time(self.timer.start),
                    self.timer.increment
                ));

            }
            GameMode::Custom => {
                pgn.push_str(&format!(
                    "Custom: {} + {}\"]\n",
                    format_time(self.timer.start),
                    self.timer.increment
                ));
            }
        }
        pgn.push_str("[Termination \"");
        if self.timer.white_time == 0.0 || self.timer.black_time == 0.0 {
            pgn.push_str("Time forfeit\"]\n");
        } else if !pgn.contains('#') && !pgn.contains("1/2 - 1/2") {
            pgn.push_str("Abandoned\"]\n");
        } else {
            pgn.push_str("Normal\"]\n");
        }
        pgn.push('\n');
        let mut wrapped = String::new();
        for (i, c) in self.history_san.chars().enumerate() {
            if i > 0 && i % 80 == 0 {
                wrapped.push('\n');
            }
            wrapped.push(c);
        }

        pgn.push_str(&wrapped);
        pgn.push('\n');

        pgn
    }
}
