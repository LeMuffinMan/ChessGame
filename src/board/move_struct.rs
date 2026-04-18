use crate::Coord;
use crate::board::cell::Cell;

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub origin: Coord,
    pub dest: Coord,
    pub capture: Cell,
    pub en_passant: Option<Coord>,
    pub white_castle: (bool, bool),
    pub black_castle: (bool, bool),
    pub move_type: MoveType,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveType {
    Regular,
    EnPassant,
    Castle(CastleSide),
    // Promotion(Piece), dans mon implementation actuelle je n'ai pas encore recupere l'input pour la promotion
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastleSide {
    Left,
    Right,
}
