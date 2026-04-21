use crate::Board;
use crate::board::cell::Cell::*;
use crate::board::cell::Color;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::move_gen::Move;
use crate::board::move_gen::MoveList;
use crate::board::move_gen::generate_moves;
use crate::engine::evaluator::Evaluator;
use crate::engine::evaluator::PositionalEvaluator;
use crate::gui::bot_difficulty::BotDifficulty::*;
use crate::gui::chessapp::SearchStats;
use crate::gui::player_type::PlayerType;
use crate::gui::player_type::PlayerType::*;
use js_sys::Math;

const MATE_SCORE: i32 = 1_000_000;
const MEDIUM_DEPTH: u8 = 3;
const HARD_DEPTH: u8 = 4;

pub fn minimax<E: Evaluator>(
    board: &mut Board,
    depth: u8,
    active_player: Color,
    eval: &E,
    mut alpha: i32,
    beta: i32,
    stats: &mut SearchStats,
) -> i32 {
    stats.nodes += 1;

    if depth == 0 {
        return eval.evaluate(board, active_player);
    }

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return if is_king_exposed(board, &active_player) {
            -MATE_SCORE + depth as i32
        } else {
            0
        };
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    for i in 0..moves.len() {
        // 🔍 Sélection du meilleur coup restant (lazy)
        let mut best_idx = i;
        let mut best_score = i32::MIN;

        for j in i..moves.len() {
            let score = move_order_score(
                &moves[j],
                board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize].get_piece(),
                board,
                stats.killer_moves[depth as usize]
                    .get(0)
                    .and_then(|x| x.as_ref()),
                stats.killer_moves[depth as usize]
                    .get(1)
                    .and_then(|x| x.as_ref()),
            );

            if score > best_score {
                best_score = score;
                best_idx = j;
            }
        }

        // 🔁 Mettre le meilleur coup en position i
        moves.swap(i, best_idx);
        let m = moves[i];

        // ▶️ Jouer le coup
        board.apply_move(&m, active_player);

        let score = -minimax(board, depth - 1, opponent, eval, -beta, -alpha, stats);

        board.undo_move(m, active_player);

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            stats.cutoffs += 1;

            if m.capture == Free {
                let depth_index = depth as usize;

                if stats.killer_moves[depth_index][0] != Some(m) {
                    stats.killer_moves[depth_index][1] = stats.killer_moves[depth_index][0];
                    stats.killer_moves[depth_index][0] = Some(m);
                }
            }

            return alpha;
        }
    }

    alpha
}

pub fn find_best_move<E: Evaluator>(
    board: &mut Board,
    active_player: Color,
    eval: &E,
    depth: u8,
    stats: &mut SearchStats,
) -> Option<Move> {
    stats.nodes += 1;

    let mut move_list = MoveList::new();
    generate_moves(board, &active_player, &mut move_list);
    let moves = &mut move_list.moves[..move_list.count];

    if moves.is_empty() {
        return None;
    }

    let opponent = match active_player {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let mut best_move = None;
    let mut alpha = i32::MIN;

    for i in 0..moves.len() {
        let mut best_idx = i;
        let mut best_score = i32::MIN;

        for j in i..moves.len() {
            let score = move_order_score(
                &moves[j],
                board.grid[moves[j].origin.row as usize][moves[j].origin.col as usize].get_piece(),
                board,
                stats.killer_moves[depth as usize]
                    .get(0)
                    .and_then(|x| x.as_ref()),
                stats.killer_moves[depth as usize]
                    .get(1)
                    .and_then(|x| x.as_ref()),
            );

            if score > best_score {
                best_score = score;
                best_idx = j;
            }
        }

        moves.swap(i, best_idx);
        let m = moves[i];

        board.apply_move(&m, active_player);

        let score = -minimax(
            board,
            depth - 1,
            opponent,
            eval,
            -MATE_SCORE,
            MATE_SCORE,
            stats,
        );

        board.undo_move(m, active_player);

        if score > alpha {
            alpha = score;
            best_move = Some(m);

            stats.cutoffs += 1;

            if m.capture == Free {
                let depth_index = depth as usize;

                if stats.killer_moves[depth_index][0] != Some(m) {
                    stats.killer_moves[depth_index][1] = stats.killer_moves[depth_index][0];
                    stats.killer_moves[depth_index][0] = Some(m);
                }
            }
        }
    }

    best_move
}

pub fn move_order_score(
    mv: &Move,
    attacker: Option<&Piece>,
    board: &Board,
    killer1: Option<&Move>,
    killer2: Option<&Move>,
) -> i32 {
    if Some(mv) == killer1 {
        return 1_000_000;
    }

    if Some(mv) == killer2 {
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
    let promotion_bonus = if mv.is_promotion(board) { 800 } else { 0 };
    let check_bonus = if mv.check.is_some() { 50 } else { 0 };
    let mvv_lva = capture_score * 10 - attacker_penalty;

    -(mvv_lva + promotion_bonus + check_bonus)
}

pub fn get_bot_move(
    difficulty: &PlayerType,
    board: &mut Board,
    active_player: Color,
    stats: &mut SearchStats,
) -> Option<Move> {
    match difficulty {
        Bot(Hard) => find_best_move(
            board,
            active_player,
            &PositionalEvaluator,
            HARD_DEPTH,
            stats,
        ),
        Bot(Medium) => find_best_move(
            board,
            active_player,
            &PositionalEvaluator,
            MEDIUM_DEPTH,
            stats,
        ),
        Bot(Easy) => {
            let mut move_list = MoveList::new();
            generate_moves(board, &active_player, &mut move_list);
            let moves = &mut move_list.moves[..move_list.count];
            let index = (Math::random() * moves.len() as f64).floor() as usize;
            Some(moves[index])
        }
        _ => None,
    }
}
