mod board;
pub mod gui;
mod pgn;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use crate::gui::chessapp::ChessApp;
use crate::gui::chessapp::UiType::*;
use board::Board;

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

    //the runner allow to execute our UI in a HTML canva
    let runner = WebRunner::new();

    //we try to catch the JS object window to interact with
    let window = web_sys::window().expect("no global `window`");
    //this is the html content of the page
    let document = window.document().expect("should have a document");
    let canvas = document
        .get_element_by_id("chessappid")
        .expect("Canvas not found")
        .dyn_into::<HtmlCanvasElement>() //exposes canvas API
        .expect("Failed to cast canvas");

    let is_mobile = {
        let ua = window.navigator().user_agent().unwrap_or_default();
        //this line detects a mobile or desktop environment
        ua.to_lowercase().contains("mobi")
            || window.inner_width().unwrap().as_f64().unwrap_or(1024.0) < 800.0
    };

    let ui_type = if is_mobile { Mobile } else { Desktop };

    spawn_local(async move {
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |_cc| Ok(Box::new(ChessApp::new(ui_type)))),
            )
            .await
            .unwrap();
    });

    Ok(())
}
