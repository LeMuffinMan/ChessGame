
use chrono::Utc;
use std::path::Path;
use std::fs;

pub fn export_pgn(san: &String, path: &Path) {
    let mut pgn = String::new();
    pgn.push_str("[Event \"ChessGame\"]\n[Site \"ChessGame\"]\n[Date \"");
    pgn.push_str(Utc::now().to_string().as_str());
    pgn.push_str("\"]\n[White \"White_player\"]\n[Black \"Black_player\"]\n");
    if let Some(result) = san.split_whitespace().last() {
        pgn.push_str("[Result : \"");
        if result == "0-1" || result == "1-0" || result == "1/2 - 1/2" {
            pgn.push_str(result);
        } else {
            pgn.push('*');
        }
        pgn.push_str("\"]\n\n");
        pgn.push_str(san);
        pgn.push('\n');
        match fs::write(path, &pgn) {
            Ok(_) => println!("File saved with success"),
            Err(e) => eprintln!("Error saving file : {}", e),
        }
        println!("{}", pgn);
    }
}
