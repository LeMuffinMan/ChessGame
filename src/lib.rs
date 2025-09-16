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

//Documenter
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
//      - refacto
//          - encode pgn
//          - export pgn
//          move in board :
//          - get_threaten
//          - update_threaten
//          - is_legal
//          - piece_case
//          - try_move
//
//          - draw rules : hash
//          - hooks
//          - window_dialog
//          pgn folder
//          thread & validate to move in board
//      - documenter
//      - readme / release
//
//      - pgn decoder
//          - load
//
//Fix :
//      - save as pgn
//          - finir les metadonnees et les 80 chars
//      - undo avec promotion
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
