use crate::Board;
use crate::Coord;
use crate::board::cell::Piece;

#[derive(Clone)]
pub struct PromoteInfo {
    pub from: Coord,
    pub to: Coord,
    pub prev_board: Board,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
}
