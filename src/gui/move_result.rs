use crate::Coord;
use crate::Color;
use crate::Board;
use crate::validate_move;
use crate::mat_or_pat;

pub struct MoveOutcome {
    pub applied: bool,
    pub mate: bool,
    check: bool,
    pub messages: Vec<String>,
}

pub fn try_apply_move(
    board: &mut Board,
    color: &mut Color,
    turn: &mut u32,
    from: Coord,
    to: Coord,
) -> Option<MoveOutcome> {
    let mut msgs = vec![];
    if !board.is_legal_move(&from, &to, color) {
        msgs.push(format!("Illegal move : {from:?} -> {to:?}"));
        return Some(MoveOutcome { applied: false, mate: false, check: false, messages: msgs });
    }
    if validate_move::is_king_exposed(&from, &to, color, board) {
        msgs.push("King is exposed : illegal move".into());
        return Some(MoveOutcome { applied: false, mate: false, check: false, messages: msgs });
    }
    board.update_board(&from, &to, color);
    *turn += 1;
    *color = match *color { Color::White => Color::Black, Color::Black => Color::White };

    let mate = mat_or_pat(board, color);
    if mate {
        // msgs.push("Checkmate or stalemate".into());
        return Some(MoveOutcome { applied: true, mate: true, check: false, messages: msgs });
    }

    msgs.push(format!("{color:?} to move"));
    let mut in_check = false;
    if let Some(k) = board.get_king(color) {
        if board.threaten_cells.contains(&k) {
            board.check = true;
            msgs.push("Check !".into());
            in_check = true;
        }
    }
    Some(MoveOutcome { applied: true, mate: false, check: in_check, messages: msgs })
}

