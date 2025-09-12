#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
use Piece::*;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Color {
    Black,
    White,
}
use Color::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    pub col: u8, //declarer de base des u8 ?
    pub row: u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Occupied(Piece, Color),
    Free,
}

impl Cell {
    pub fn get_piece(&self) -> Option<&Piece> {
        match self {
            Cell::Occupied(piece, _) => Some(piece),
            Cell::Free => None,
        }
    }
    pub fn get_color(&self) -> Option<&Color> {
        match self {
            Cell::Occupied(_, color) => Some(color),
            Cell::Free => None,
        }
    }

    pub fn is_color(&self, color: &Color) -> bool {
        match self {
            Cell::Occupied(_, cell_color) => cell_color == color,
            Cell::Free => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Free => true,
            Cell::Occupied(_, _) => false,
        }
    }
    pub fn is_opponent_color(&self, color: &Color) -> bool {
        match self {
            Cell::Free => false,
            Cell::Occupied(_, cell_color) => cell_color != color,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Cell::Occupied(piece, color) => {
                let piece_char = match piece {
                    Pawn => "p",
                    Rook => "r",
                    Knight => "n",
                    Bishop => "b",
                    Queen => "q",
                    King => "k",
                };
                match color {
                    Black => piece_char.to_uppercase(),
                    White => piece_char.to_string(),
                }
            }
            Cell::Free => String::from(" "),
        };
        write!(f, "{display_str}")
    }
}
