use crate::Board;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Color;
use crate::board::cell::Color::*;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::is_king_exposed::is_king_exposed;
use crate::board::moves::move_gen::generate_moves;
use crate::board::moves::move_structs::{Move, MoveList};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum End {
    Checkmate,
    TimeOut,
    Pat,
    Draw,
    Resign,
}

#[derive(Clone, PartialEq)]
pub enum DrawOption {
    Request,
    Available(DrawRule),
}

#[derive(Clone, PartialEq)]
pub enum DrawRule {
    TripleRepetition,
    FiftyMoves,
}

#[derive(Clone, PartialEq)]
pub struct DrawState {
    pub board_hashs: HashMap<u64, usize>,
    pub draw_option: Option<DrawOption>,
    pub draw_moves_count: u32,
}

impl DrawState {
    pub fn new() -> Self {
        Self {
            board_hashs: HashMap::new(),
            draw_option: None,
            draw_moves_count: 0,
        }
    }
}

//returned to chessapp
pub enum GameEvent {
    Ok,
    Check,
    Checkmate,
    Stalemate,
    Draw,
    PromotionPending(Coord),
}

pub struct Game {
    pub board: Board,
    pub active_player: Color,
    pub end: Option<End>,
    pub turn: u32,
    pub threaten_cells: Vec<Coord>,
    pub legals_moves: Vec<Move>,
    pub draw: DrawState,
    pub history: Vec<Move>,
    pub initial_board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::init_board();
        let initial_board = Board::init_board();
        let active_player = White;

        let threaten_cells = board.update_threatens_cells(&active_player);
        let mut move_list = MoveList::new();
        generate_moves(&mut board, &active_player, &mut move_list, false);
        let legals_moves = move_list.moves[..move_list.count].to_vec();

        Self {
            board,
            active_player,
            end: None,
            turn: 1,
            threaten_cells,
            legals_moves,
            draw: DrawState::new(),
            history: Vec::new(),
            initial_board,
        }
    }

    pub fn opponent(&self) -> Color {
        match self.active_player {
            White => Black,
            Black => White,
        }
    }

    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        self.board.get(coord).is_color(&self.active_player)
    }

    pub fn board_at(&self, index: usize) -> Board {
        let mut board = self.initial_board.clone();
        let mut player = White;
        for m in &self.history[..index] {
            board.apply_move(m, player);
            player = match player {
                White => Black,
                Black => White,
            };
        }
        board
    }

    pub fn try_move(&mut self, from: Coord, to: Coord) -> Option<GameEvent> {
        let m = self.validate_and_build(from, to)?;

        self.board.apply_move(&m, self.active_player);
        if is_king_exposed(&self.board, &self.active_player) {
            self.board.undo_move(m, self.active_player);
            return None;
        }

        self.history.push(m);
        self.fifty_moves_draw_check(&m);

        if self.impossible_mate_check() {
            self.end = Some(End::Draw);
            return Some(GameEvent::Draw);
        }
        self.add_hash();

        if self.draw.draw_option == Some(DrawOption::Available(DrawRule::TripleRepetition)) {
            self.end = Some(End::Draw);
            return Some(GameEvent::Draw);
        }

        if self.active_player == Black {
            self.turn += 1;
        }

        if let Some(coord) = self.find_promotion() {
            return Some(GameEvent::PromotionPending(coord));
        }

        Some(self.after_move())
    }

    pub fn apply_promotion(&mut self, coord: Coord, piece: Piece) -> GameEvent {
        let color = self.active_player;
        self.board[(coord.row as usize, coord.col as usize)] = Cell::Occupied(piece, color);
        self.after_move()
    }

    pub fn undo(&mut self) -> bool {
        if self.history.is_empty() {
            return false;
        }

        let hash = self.board.hash;
        if let Some(count) = self.draw.board_hashs.get_mut(&hash) {
            if *count > 0 {
                *count -= 1;
            }
        }

        self.history.pop();
        let idx = self.history.len();
        self.board = self.board_at(idx);
        self.active_player = if idx % 2 == 0 { White } else { Black };
        self.turn = (idx / 2) as u32 + 1;
        self.end = None;

        self.threaten_cells = self.board.update_threatens_cells(&self.active_player);
        let mut move_list = MoveList::new();
        generate_moves(&mut self.board, &self.active_player, &mut move_list, false);
        self.legals_moves = move_list.moves[..move_list.count].to_vec();

        let king = match self.active_player {
            White => self.board.white_king,
            Black => self.board.black_king,
        };
        self.board.check = if self.threaten_cells.contains(&king) {
            Some(king)
        } else {
            None
        };

        true
    }

    fn validate_and_build(&mut self, from: Coord, to: Coord) -> Option<Move> {
        let mut move_list = MoveList::new();
        generate_moves(&mut self.board, &self.active_player, &mut move_list, false);
        let legal = move_list.moves[..move_list.count]
            .iter()
            .any(|m| m.origin == from && m.dest == to);
        if !legal {
            return None;
        }
        Some(self.board.build_move(from, to, self.active_player))
    }

    fn find_promotion(&self) -> Option<Coord> {
        let promote_row = if self.active_player == White { 7 } else { 0 };
        for y in 0..8 {
            if self.board[(promote_row as usize, y as usize)].is_color(&self.active_player)
                && self.board[(promote_row as usize, y as usize)].get_piece() == Some(&Pawn)
            {
                return Some(Coord {
                    row: promote_row as u8,
                    col: y as u8,
                });
            }
        }
        None
    }

    fn after_move(&mut self) -> GameEvent {
        self.active_player = match self.active_player {
            White => Black,
            Black => White,
        };

        self.threaten_cells = self.board.update_threatens_cells(&self.active_player);
        let mut move_list = MoveList::new();
        generate_moves(&mut self.board, &self.active_player, &mut move_list, false);
        self.legals_moves = move_list.moves[..move_list.count].to_vec();

        let king = match self.active_player {
            White => self.board.white_king,
            Black => self.board.black_king,
        };

        if self.legals_moves.is_empty() {
            if self.threaten_cells.contains(&king) {
                self.end = Some(End::Checkmate);
                self.board.check = Some(king);
                return GameEvent::Checkmate;
            } else {
                self.end = Some(End::Pat);
                return GameEvent::Stalemate;
            }
        }

        if self.threaten_cells.contains(&king) {
            self.board.check = Some(king);
            return GameEvent::Check;
        }

        self.board.check = None;
        GameEvent::Ok
    }

    fn fifty_moves_draw_check(&mut self, m: &Move) {
        if let Some(p) = self.board.get(&m.dest).get_piece()
            && p == &Pawn
        {
            self.draw.draw_moves_count = 0;
            return;
        }
        if m.capture != Cell::Free {
            self.draw.draw_moves_count = 0;
            return;
        }
        self.draw.draw_moves_count += 1;
        if self.draw.draw_moves_count >= 50 {
            self.draw.draw_option = Some(DrawOption::Available(DrawRule::FiftyMoves));
        } else {
            self.draw.draw_option = None;
        }
    }

    fn add_hash(&mut self) {
        let hash_value = self.board.hash;
        let count = self.draw.board_hashs.entry(hash_value).or_insert(0);
        *count += 1;
        if *count >= 3 {
            self.draw.draw_option = Some(DrawOption::Available(DrawRule::TripleRepetition));
        }
    }

    fn impossible_mate_check(&mut self) -> bool {
        let mut pieces = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.board[(x as usize, y as usize)].get_piece()
                    && let Some(color) = self.board[(x as usize, y as usize)].get_color()
                {
                    let cell_color = if (x + y) % 2 == 0 { White } else { Black };
                    pieces.push((*piece, *color, cell_color));
                }
            }
        }
        match pieces.len() {
            2 => true,
            3 => pieces
                .iter()
                .all(|(p, _, _)| *p == King || *p == Bishop || *p == Knight),
            4 => {
                let mut wb = None;
                let mut bb = None;
                for (piece, color, cell_color) in &pieces {
                    if *piece != King && *piece != Bishop {
                        return false;
                    }
                    if *piece == Bishop {
                        if *color == White {
                            wb = Some(*cell_color);
                        } else {
                            bb = Some(*cell_color);
                        }
                    }
                }
                if let (Some(c1), Some(c2)) = (wb, bb) {
                    c1 == c2
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
