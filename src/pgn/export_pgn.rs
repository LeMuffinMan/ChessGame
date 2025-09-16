use crate::ChessApp;

use chrono::Utc;
use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Blob;
use web_sys::HtmlAnchorElement;
use web_sys::Url;
use web_sys::window;

impl ChessApp {
    //Documenter
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
        pgn.push('\n');
        pgn.push_str(&self.history_san);
        //ici split l'history en plusieurs lignes de 80 chars
        pgn.push('\n');

        let parts = Array::new();
        parts.push(&JsValue::from_str(&pgn));
        let blob = Blob::new_with_str_sequence(&parts.into())?;
        let url = Url::create_object_url_with_blob(&blob)?;

        // access document
        let window = window().unwrap();
        let doc = window.document().unwrap();

        let link = doc.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;
        link.set_href(&url);
        link.set_download("chessgame.pgn");
        link.style().set_property("display", "none")?;
        doc.body().unwrap().append_child(&link)?;

        // trigger download
        link.click();

        // Cleanup
        doc.body().unwrap().remove_child(&link)?;
        Url::revoke_object_url(&url)?;

        Ok(())
    }
}
