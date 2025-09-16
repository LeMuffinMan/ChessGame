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
        link.style().set_property("display", "none")?;
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
        //ajouter type de game et rounds
        //time control
        //termination ?
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
        pgn.push('\n');
        pgn.push_str(&self.history_san);
        //ici split l'history en plusieurs lignes de 80 chars
        pgn.push('\n');

        pgn
    }
}
