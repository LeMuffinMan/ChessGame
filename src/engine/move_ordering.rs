use crate::board::cell::Cell::*;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::Move;
use crate::board::moves::move_structs::MoveType::Promotion;
use crate::board::cell::Piece;
use crate::engine::search_stats::HistoryTable;

pub fn move_order_score(
    mv: &Move,
    attacker: Option<&Piece>,
    killer1: Option<Move>,
    killer2: Option<Move>,
    history: &HistoryTable,
) -> i32 {
    if killer1 == Some(*mv) {
        return 1_000_000;
    }
    if killer2 == Some(*mv) {
        return 900_000;
    }

    let capture_score = match mv.capture {
        Occupied(piece, _) => match piece {
            Pawn => 10,
            Knight => 30,
            Bishop => 30,
            Rook => 50,
            Queen => 90,
            King => 10000,
        },
        Free => 0,
    };

    let attacker_penalty = match attacker {
        Some(Pawn) => 1,
        Some(Knight) => 3,
        Some(Bishop) => 3,
        Some(Rook) => 5,
        Some(Queen) => 9,
        Some(King) => 100,
        None => 0,
    };

    let promotion_bonus = match mv.move_type {
        Promotion(_) => 800,
        _ => 0,
    };

    let check_bonus = if mv.check.is_some() { 50 } else { 0 };
    let mvv_lva = capture_score * 10 - attacker_penalty;

    // killers (1M, 900K) > captures (200K+) > quiet+history (0–199K)
    if capture_score > 0 {
        return 200_000 + mvv_lva + promotion_bonus + check_bonus;
    }

    let history_bonus = {
        let from = mv.origin.row as usize * 8 + mv.origin.col as usize;
        let to = mv.dest.row as usize * 8 + mv.dest.col as usize;
        history.get(from, to) as i32
    };

    promotion_bonus + check_bonus + history_bonus
}
