
pub struct Move {
    origin: Coord,
    dest: Coord,
    capture: Cell,
    en_passant: Option<Coord>,
    white_castle: (bool, bool),
    black_castle: (bool, bool),
    move_type: MoveType,
}

pub enum MoveType {
    Regular,
    EnPassant,
    Castle(CastleSide),
    // Promotion(Piece), dans mon implementation actuelle je n'ai pas encore recupere l'input pour la promotion
}

pub enum CastleSide {
    Left,
    Right,
}
