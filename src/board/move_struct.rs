use crate::Coord;
use crate::board::cell::Cell;
use crate::board::board_struct::CastleRights;

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub origin: Coord,
    pub dest: Coord,
    pub capture: Cell,
    pub en_passant: Option<Coord>,
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
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
