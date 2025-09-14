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

#[cfg(target_arch = "wasm32")]
use eframe::WebRunner;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
#[cfg(target_arch = "wasm32")]
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

//Todo 
//
//Fix : 
//      - Flip + autoflip
//      - save as pgn
//      - promotion in replay
//
//New game 
//      - ask for names
//      - count rounds
//      - mode in game / mode analyse
//
//Timer
//
//Settings 
//  - dark mode
//  - regroupe
//
//Decode pgn 
//      - load as pgn
//
//Chat 
