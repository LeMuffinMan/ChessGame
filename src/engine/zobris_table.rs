use crate::Board;
use crate::Color;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use std::sync::OnceLock;

pub struct ZobristTable {
    pub pieces: [[u64; 64]; 12],
    pub side_to_move: u64,
    pub castling: [u64; 4],
    pub en_passant: [u64; 8],
}

impl ZobristTable {
    fn generate() -> Self {
        let mut state: u64 = 0xDEADBEEF_CAFEBABE;
        let mut next = || lcg_rand(&mut state);

        let mut pieces = [[0; 64]; 12];
        for row in &mut pieces {
            for sq in row.iter_mut() {
                *sq = next();
            }
        }
        let mut castling = [0; 4];
        for c in &mut castling {
            *c = next();
        }
        let mut en_passant = [0; 8];
        for ep in &mut en_passant {
            *ep = next();
        }
        Self {
            pieces,
            side_to_move: next(),
            castling,
            en_passant,
        }
    }
}

static ZOBRIST: OnceLock<ZobristTable> = OnceLock::new();

pub fn zobrist() -> &'static ZobristTable {
    ZOBRIST.get_or_init(|| ZobristTable::generate())
}

fn lcg_rand(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

pub fn hash_from_scratch(board: &Board, active: Color) -> u64 {
    let zt = zobrist();
    let mut h: u64 = 0;

    for row in 0..8 {
        for col in 0..8 {
            if let Cell::Occupied(piece, color) = board.grid[row][col] {
                let sq = row * 8 + col;
                h ^= zt.pieces[piece_index(piece, color)][sq];
            }
        }
    }

    if active == Color::Black {
        h ^= zt.side_to_move;
    }

    if board.white_castle.long {
        h ^= zt.castling[0];
    }
    if board.white_castle.short {
        h ^= zt.castling[1];
    }
    if board.black_castle.long {
        h ^= zt.castling[2];
    }
    if board.black_castle.short {
        h ^= zt.castling[3];
    }

    if let Some(ep) = board.en_passant {
        h ^= zt.en_passant[ep.col as usize];
    }

    h
}

pub fn piece_index(piece: Piece, color: Color) -> usize {
    let base = match piece {
        Piece::Pawn => 0,
        Piece::Knight => 1,
        Piece::Bishop => 2,
        Piece::Rook => 3,
        Piece::Queen => 4,
        Piece::King => 5,
    };
    let offset = if color == Color::Black { 6 } else { 0 };
    base + offset
}
