pub mod board;
pub mod cell;
pub mod is_king_exposed;
pub mod moves;
pub mod pin_detection;
pub mod threat;
pub mod try_move;
pub mod update_board;
pub mod utils;
pub use board::Board;
pub mod fen;

#[cfg(test)]
mod tests;
