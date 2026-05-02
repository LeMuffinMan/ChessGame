use crate::board::moves::move_structs::Move;

//using a preallocated vec for TT
// generation is used for fifty_move and game_history, preserving TT entries for minimax compute
#[derive(Clone, Copy, Default)]
pub struct TtEntry {
    pub key: u64,
    pub score: i32,
    pub depth: u8,
    pub generation: u8,
    pub flag: TtFlag,
    pub best_move: Option<Move>,
}

//using alpha beta pruning, we often return not the exact score, but an information about the exploration of a branch
// Exact flag is used when we explored all moves, the score is reliable, we can use this position with this score next time we find it
// LowerBound is used when we had a score which were better than alpha, but we cut beta : we know the score is good, but may be it's even better
// UpperBound : no move was better than alpha, the real score might be at most equal to the score we registered, so score become our high bound
// so we can use these flags to :
// Exact : return score directly
// LowerBound : elevate alpha to our "at least" good score
// UpperBound : lower beta to our score (which is may be even lower)
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TtFlag {
    Exact,
    LowerBound,
    #[default]
    UpperBound,
}
