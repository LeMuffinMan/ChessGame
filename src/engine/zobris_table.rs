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

//generate() will return a struct containing 768 random values
// at call of generate(), we compose the hash of the first board : en empty board, where for each square we XOR "in" all pieces on their starting position
// This allows us to calculate new hashs very efficiently : instead of recalculating a new hash for each new board, we XOR in and XOR out the differences between them
// Since each turn is a piece moving and optionaly a piece captured, we can incrementaly deduce the hash of the new board following these steps :
//  XOR the moving piece from its origin : since it was hashed for our previous board, it's a double XOR, so the new hash match a board where this cell is empty,
//  XOR the the piece at dest in case of capture (same logic as above)
//  XOR the moving piece at dest
//  Castles, en_passant and player on trait must be hashed too
//Doing so, we have the exact same resulted hash for the new board, but with way fewer operations than generating a hash from scratch
impl ZobristTable {
    fn generate() -> Self {
        //That's our seed
        let mut state: u64 = 0x9E3779B97F4A7C15;

        //next is a closure, capturing state as &mut, we use it as a manual iterator
        // using state as seed, we will have a new seeded u64 at each call of next();
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

// our table MUST be init once, and accessible anytime : a const with Onelock to read it as quick as any variable (Mutex would imply a read lock)
static ZOBRIST: OnceLock<ZobristTable> = OnceLock::new();

//&'static : the table will live for the all program duration
pub fn zobrist() -> &'static ZobristTable {
    ZOBRIST.get_or_init(|| ZobristTable::generate())
}

//to generate a pseudo-random sequence, we often use state = state * A + B were A and B are defined constants (chosen to guarantee long period and bit distribution)
//Theses constants guarantee a good distribution of bits on the all u64 space, thus, there is still a risk of collision, but very low (≈ 1/2^64 for each duo of boards)
// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
fn lcg_rand(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

// on utilise toujours cette fonction a defaut de bien gerer en_passant castle
pub fn hash_from_scratch(board: &Board, active: Color) -> u64 {
    let zt = zobrist();
    let mut h: u64 = 0;

    for row in 0..8 {
        for col in 0..8 {
            if let Cell::Occupied(piece, color) = board[(row, col)] {
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
