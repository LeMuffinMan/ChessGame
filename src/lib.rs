
pub mod gui;
use crate::gui::chessapp_struct::ChessApp;

mod threat;
use threat::get_threaten_cells;

mod board;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use board::Board;

mod pgn;
mod validate_move;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;
use eframe::WebRunner;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let runner = WebRunner::new();

    let window = web_sys::window().expect("no global `window`");
    let document = window.document().expect("should have a document");
    let canvas = document
        .get_element_by_id("chessappid")
        .expect("Canvas not found")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Failed to cast canvas");

    spawn_local(async move {
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(ChessApp::default()))),
            )
            .await
            .unwrap();
    });

    Ok(())
}

