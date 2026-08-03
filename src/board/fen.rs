use crate::Board;
use crate::board::CastleRights;
use crate::board::cell::Cell;
use crate::board::cell::Cell::Occupied;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Coord;
use crate::board::cell::Piece::*;
use crate::engine::evaluator::{get_piece_value_at, non_pawn_raw};

pub struct FenInfo {
    pub board: Board,
    pub active_color: Color,
    pub halfmove_clock: u32,
    pub fullmove: u32,
}

impl Board {
    pub fn board_from_fen(fen: &str) -> FenInfo {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: CastleRights {
                long: false,
                short: false,
            },
            black_castle: CastleRights {
                long: false,
                short: false,
            },
            white_king: Coord { row: 0, col: 4 },
            black_king: Coord { row: 7, col: 4 },
            check: None,
            score: 0,
            non_pawn_material: 0,
            hash: 0,
        };

        let mut parts = fen.split(' ');

        let placement = parts.next().unwrap_or("");
        let mut row: i8 = 7;
        let mut col: i8 = 0;

        for c in placement.chars() {
            match c {
                '/' => {
                    row -= 1;
                    col = 0;
                }
                '1'..='8' => {
                    col += c as i8 - '0' as i8;
                }
                _ => {
                    let color = if c.is_uppercase() { White } else { Black };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Pawn,
                        'r' => Rook,
                        'n' => Knight,
                        'b' => Bishop,
                        'q' => Queen,
                        'k' => King,
                        _ => unreachable!("invalid FEN char: {c}"),
                    };
                    let coord = Coord {
                        row: row as u8,
                        col: col as u8,
                    };
                    if piece == King {
                        match color {
                            White => board.white_king = coord,
                            Black => board.black_king = coord,
                        }
                    }
                    board[(row, col)] = Occupied(piece, color);
                    board.score += get_piece_value_at(&piece, &color, &coord);
                    board.non_pawn_material += non_pawn_raw(&piece);
                    col += 1;
                }
            }
        }

        let active_color = match parts.next().unwrap_or("w") {
            "b" => Black,
            _ => White,
        };

        for c in parts.next().unwrap_or("-").chars() {
            match c {
                'K' => board.white_castle.short = true,
                'Q' => board.white_castle.long = true,
                'k' => board.black_castle.short = true,
                'q' => board.black_castle.long = true,
                _ => {}
            }
        }

        let ep = parts.next().unwrap_or("-");
        if ep != "-" {
            let mut chars = ep.chars();
            if let (Some(file), Some(rank)) = (chars.next(), chars.next()) {
                board.en_passant = Some(Coord {
                    row: rank as u8 - b'1',
                    col: file as u8 - b'a',
                });
            }
        }

        let halfmove_clock = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);

        let fullmove = parts.next().unwrap_or("1").parse::<u32>().unwrap_or(1);

        board.sync_hash(active_color);

        FenInfo {
            board,
            active_color,
            halfmove_clock,
            fullmove,
        }
    }

    pub fn to_fen(&self, active_color: Color, halfmove_clock: u32, fullmove: u32) -> String {
        let mut placement = String::new();
        for row in (0..8i8).rev() {
            let mut empty_run = 0u8;
            for col in 0..8 {
                match self[(row as usize, col as usize)] {
                    Occupied(piece, color) => {
                        if empty_run > 0 {
                            placement.push_str(&empty_run.to_string());
                            empty_run = 0;
                        }
                        let c = match piece {
                            Pawn => 'p',
                            Rook => 'r',
                            Knight => 'n',
                            Bishop => 'b',
                            Queen => 'q',
                            King => 'k',
                        };
                        placement.push(if color == White {
                            c.to_ascii_uppercase()
                        } else {
                            c
                        });
                    }
                    Cell::Free => empty_run += 1,
                }
            }
            if empty_run > 0 {
                placement.push_str(&empty_run.to_string());
            }
            if row > 0 {
                placement.push('/');
            }
        }

        let active = if active_color == White { "w" } else { "b" };

        let mut castling = String::new();
        if self.white_castle.short {
            castling.push('K');
        }
        if self.white_castle.long {
            castling.push('Q');
        }
        if self.black_castle.short {
            castling.push('k');
        }
        if self.black_castle.long {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }

        let en_passant = match self.en_passant {
            Some(coord) => format!("{}{}", (b'a' + coord.col) as char, (b'1' + coord.row) as char),
            None => "-".to_string(),
        };

        format!("{placement} {active} {castling} {en_passant} {halfmove_clock} {fullmove}")
    }
}
