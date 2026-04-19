use crate::Board;
use crate::board::cell::Color;
use crate::board::cell::Piece;

pub fn is_king_exposed(board: &Board, active_player: &Color) -> bool {
    let king_pos = match active_player {
        Color::White => board.white_king,
        Color::Black => board.black_king,
    };

    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dr, dc) in directions {
        for dist in 1..8 {
            let r = king_pos.row as i32 + dr * dist;
            let c = king_pos.col as i32 + dc * dist;

            if !(0..8).contains(&r) || !(0..8).contains(&c) {
                break;
            }

            let cell = &board.grid[r as usize][c as usize];
            if let (Some(p_type), Some(p_color)) = (cell.get_piece(), cell.get_color()) {
                if *p_color == *active_player {
                    break;
                } else {
                    let is_diag = dr != 0 && dc != 0;
                    match p_type {
                        Piece::Queen => return true,
                        Piece::Rook if !is_diag => return true,
                        Piece::Bishop if is_diag => return true,
                        Piece::King if dist == 1 => return true,
                        Piece::Pawn if dist == 1 && is_diag => {
                            let attack_dir: i32 = if *active_player == Color::White {
                                1
                            } else {
                                -1
                            };
                            if dr == attack_dir {
                                return true;
                            }
                        }
                        _ => break,
                    }
                    break;
                }
            }
        }
    }

    let knight_moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for (dr, dc) in knight_moves {
        let r = king_pos.row as i32 + dr;
        let c = king_pos.col as i32 + dc;

        if (0..8).contains(&r) && (0..8).contains(&c) {
            let cell = &board.grid[r as usize][c as usize];
            if cell.get_piece() == Some(&Piece::Knight) && cell.get_color() != Some(active_player) {
                return true;
            }
        }
    }

    false
}
