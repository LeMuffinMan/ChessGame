pub mod gui;
mod pgn;
use crate::gui::chessapp_struct::ChessApp;

mod board;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use board::Board;

#[cfg(target_arch = "wasm32")]
use eframe::WebRunner;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

//Todo : Error handling
#[wasm_bindgen(start)]
#[cfg(target_arch = "wasm32")]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    //to collect logs and display it in the browser inspect mode
    eframe::WebLogger::init(log::LevelFilter::Debug).ok(); //error to handle

    //the runner allow to execute our UI in a HTML canva
    let runner = WebRunner::new();

    //we try to catch the JS object window to interact with
    let window = web_sys::window().expect("no global `window`");
    //this is the html content of the page
    let document = window.document().expect("should have a document");
    //we seek for our element chessappid in the DOM : we must set it too in the html file at the
    //root
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
    let ui_type = if is_mobile {
        UiType::Mobile
    } else {
        UiType::Desktop
    };

    //execute through the js our chessapp
    spawn_local(async move {
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                //added move to build the mobile/desktop app
                Box::new(move |_cc| Ok(Box::new(ChessApp::new(ui_type)))),
            )
            .await
            .unwrap();
    });

    Ok(())
}

//Todo
//      -error handling
//
//      - pgn decoder
//          - load
//
//Fix :
//      - promote bug desktop 
//
//      - include in legal move for triple repetition
//      - save as pgn
//          - finir les metadonnees et les 80 chars
//      - undo avec promotion
//
//      - Refacto Draw + triple R
//
//      - Mobile : Flip / autoflip
//
//Tests :
//      - unit
//      - end to end
//
//
//
// -- Later --
//
//
//Back :
//      Settings :
//          - regroupe
//          - link blitz bullet rapid
//      Rounds :
//          - revenge button
//      AI : minmax et +
//          - Evaluation
//          - multithread
//          - profondeur
//      UCI compatibility ?
//      Multiplayer (web socket + serveur web ?)
//          - matchmaking
//          - elo
//      Analyse mode
//      Didactitiel mode
//      daily puzzle
//
//Front :
//      Sounds
//      Animations
//      Chat
//      Themes board / pieces
//
//
//      Doc
//          Objectives :
//
//          - Learning rust :
//              - why rust :
//                  - interested in system prog
//                  - Like low level prog
//                  - modern syntax and memory safety promess without perf loss
//              - highlight certains bouts de code
//              - fmt clippy et trunk
//
//          - learn in practice :
//             - Network :
//                 - multiplayer
//                 - chat
//             - Algo :
//                 - evaluation algorithm and research of best sequences of moves
//             - DevOps :
//                 - tools in situation
//                  - CI/CD
//                  - webasm as portability solution
//              - Front :
//                  - gui
//              - Back :
//                  - validation engine
//              - DB
//                  - users and elo ..
//              - encoding / decoding / reading data :
//                  - pgn module
//              - Security
//                  - Network
//                  - DB
//                  - Back
//
//          - Rust difficulties
//              - first : Theory
//              - Cargo makes it easier
//              - Doc is amazing
//              - longer to compile but easier to refact, restructure and maintain
//
//          - Dependencies :
//              - chrono : temps ?
//              - console_log / log : doublon, debug
//              - eframe && egui : interface
//              - wasm : Un des objectifs du projet
//              - web-sys : open windos and html and blob and url ...
//              - js-sys : DL file : pourquoi besoin du JS que pour ca ?
//          lib crrate-type cdylib rlib ?
//
//          - Crates
//          - Structure de mon code
//          -
