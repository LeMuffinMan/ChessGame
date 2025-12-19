use crate::Color;
use crate::Color::*;
use crate::Coord;
use crate::board::cell::Cell;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::DefaultHasher;
use crate::board::board_struct::DrawOption::*;
use crate::board::board_struct::DrawRule::*;
use crate::board::validate_move::piece_case::*;

#[derive(Clone, PartialEq)]
pub enum DrawOption {
    //faire une option
    Request,
    Available(DrawRule),
}

#[derive(Clone, PartialEq)]
pub enum DrawRule {
    TripleRepetition,
    FiftyMoves,
}

#[derive(Clone, PartialEq)]
pub struct LateDraw {
    pub board_hashs: HashMap<u64, usize>,
    pub draw_option: Option<DrawOption>,
    pub draw_moves_count: u32,
    pub draw_hash: Option<u64>,
}


#[derive(Clone, PartialEq)]
pub enum End {
    Checkmate,
    TimeOut,
    Pat,
    Draw,
    Resign,
}

#[derive(Clone, PartialEq)]
pub struct Board {
    pub active_player: Color,
    pub grid: [[Cell; 8]; 8],
    pub white_castle: (bool, bool), //(long, short)
    pub black_castle: (bool, bool),
    pub threaten_cells: Vec<Coord>,
    pub legals_moves: Vec<(Coord, Coord)>,
    pub en_passant: Option<Coord>,
    pub check: Option<Coord>,
    pub pawn_to_promote: Option<Coord>,
    pub promote: Option<Piece>,
    pub opponent: Color,
    pub end: Option<End>,
    pub last_move: Option<(Coord, Coord)>,
    pub turn: u32,
    pub draw: LateDraw,
}

impl Board {
    //Considering to move from GameState, into board :
    //  - Active player / opponent color
    //  - end state
    //  - turn
    pub fn new() -> Board {
        let mut board = Board {
            grid: [[Cell::Free; 8]; 8],
            en_passant: None,
            white_castle: (true, true),
            black_castle: (true, true),
            threaten_cells: Vec::new(),
            legals_moves: Vec::new(),
            check: None,
            pawn_to_promote: None,
            promote: None,
            active_player: Color::White,
            opponent: Color::Black,
            end: None,
            last_move: None,
            turn: 1,
            draw: LateDraw {
                board_hashs: HashMap::new(),
                draw_option: None,
                draw_moves_count: 0,
                draw_hash: None,
            },
        };

        board.fill_side(White);
        board.fill_side(Black);
        board.update_legals_moves();
        board.update_threatens_cells();

        board
    }

    //this function takes two cells as the move input from the player
    //it test the move legality, and if it exposes king to a threat
    //If it passes these tests it update the board to end turn
    pub fn try_move(&mut self, from: &Coord, to: &Coord) -> Result<(), String> {
        if let Err(e) = self.is_legal_move(&from, &to) {
            return Err(format!("Illegal move: {from:?} -> {to:?} : {e}"));
        }
        if let Err(e) = self.is_king_exposed(&from, &to) {
            return Err(format!("King is exposed: illegal move : {e}"));
        }
        Ok(())
    }
    //Makes a copy of the board, and update it with the legal move to verify is the active player king
    //is in check position or does not solve a previous check position
    pub fn is_king_exposed(&mut self, from: &Coord, to: &Coord) -> Result<(), String> {
        let mut new_board = self.clone();
        new_board.update_board(from, to); //active player ou opponent ?
        new_board.update_threatens_cells();
        if let Some(coord) = new_board.get_king(&self.active_player) {
            if new_board.threaten_cells.contains(&coord) {
                return Err("King is threaten".to_string());
            }
            return Ok(());
        } else {
            return Err(format!("Error : {:?} king not found", self.active_player));
        }
    }

    pub fn is_active_player_piece(&mut self, coord: &Coord) -> bool {
        let cell = self.get(coord);
        cell.is_color(&self.active_player)
    }

    pub fn switch_players_color(&mut self) {
        self.active_player = match self.active_player {
            White => Black,
            Black => White,
        };
        self.opponent = match self.opponent {
            White => Black,
            Black => White,
        };
    }


    ///check if the piece on from coords, can move to the "to" coords, and if there is an
    ///obstacle on way
    pub fn is_legal_move(
        &mut self,
        from: &Coord,
        to: &Coord,
    ) -> Result<(), String> {
        let cell = self.get(from);

        match cell {
            Cell::Free => Err("No piece on source square".into()),

            Cell::Occupied(piece, piece_color) => {
                if piece_color != self.active_player {
                    return Err("Piece color does not match active player".into());
                }

                if self.get(to).is_color(&self.active_player) {
                    return Err("Destination occupied by own piece".into());
                }

                let legal = match piece {
                    Pawn   => pawn_case(from, to, &self.active_player, self),
                    Rook   => rook_case(from, to, &self.active_player, self),
                    Knight => knight_case(from, to, &self.active_player, self),
                    Bishop => bishop_case(from, to, &self.active_player, self),
                    Queen  => queen_case(from, to, &self.active_player, self),
                    King   => king_case(from, to, &self.active_player, self),
                };

                if legal {
                    Ok(())
                } else {
                    Err("Illegal move for this piece".into())
                }
            }
        }
    }

    //to rename : easier access to set the tuple of castles bools
    pub fn switch_castle(&mut self, long: bool, short: bool) {
        let castle_tuple = if self.active_player == White {
            &mut self.white_castle
        } else {
            &mut self.black_castle
        };
        castle_tuple.0 = long;
        castle_tuple.1 = short;
    }


    //if a move has passed the is legal move and is king exposed test : we update the board
    //applying this move
    pub fn update_board(&mut self, from: &Coord, to: &Coord) {
        self.en_passant = None;
        self.check = None;
        match self.get(from).get_piece() {
            Some(piece) => match piece {
                Pawn => self.update_en_passant(from, to),
                King => self.update_king_castle(from, to),
                Knight | Rook | Queen | Bishop => {}
            },
            None => {
                println!("Error : update board : from cell empty")
            }
        }
        //replace puts Cell::Free in the board cell "from" and returns what "from" contained
        //we assign the "to" cell with this returned value
        self.grid[to.row as usize][to.col as usize] = std::mem::replace(
            &mut self.grid[from.row as usize][from.col as usize],
            Cell::Free,
        );
    }

    pub fn update_en_passant(&mut self, from: &Coord, to: &Coord) {
        //if the moving piece is a pawn and it just moved in diag : if the cell is empty, it is a
        //en passant, so we need to clean the cell "behind" this pawn following the side
        if self.grid[to.row as usize][to.col as usize].is_empty() && from.col != to.col {
            self.grid[to.row as usize][to.col as usize] =
                self.grid[from.row as usize][from.col as usize];
            self.grid[from.row as usize][to.col as usize] = Cell::Free;
            return;
        }
        //if the pawn moved 2 row, it oppens a en_passant move for opponent : we store the coord of
        //the exposed pasn
        let dif = from.row as i8 - to.row as i8;
        if dif.abs() == 2 {
            self.en_passant = Some(*to);
        }
    }

    pub fn update_king_castle(&mut self, from: &Coord, to: &Coord) {
        let dif_col = to.col as i8 - from.col as i8;
        let row = match self.active_player {
            White => 0,
            Black => 7,
        };
        //the dif_col allows us to know if its a long or a short castle
        //thus, wich rook to update
        if dif_col == -2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][0] = Cell::Free;
                self.grid[row][col + 1] = Cell::Occupied(Piece::Rook, self.active_player);
            }
        } else if dif_col == 2 {
            let col = to.col as usize;
            if col > 0 {
                self.grid[row][7] = Cell::Free;
                self.grid[row][col - 1] = Cell::Occupied(Rook, self.active_player);
            }
        }
    }

   pub fn update_castles(&mut self, to: &Coord) {
        if let Some(piece) = self.get(to).get_piece() {
            match piece {
                Rook => {
                    match to.col {
                        7 => self.switch_castle(false, true),
                        0 => self.switch_castle(true, false),
                        _ => {}
                    };
                }
                King => {
                    self.switch_castle(false, false);
                }
                _ => {}
            }
        };
    }


    //for the 3 repetition draw, we need to check only if the a same situation happenned 2 times
    //and if the player can legaly reproduce a 3rd repetition
    //the situation to compare include castle state, en passant state, grid and active player
    //Instead of comparing each of these variables to verify the repetition, we can make a hash of
    //these variables together. Comparing hash will give us the info "is it the exact same as
    //somehting we found", without actually comparing the content. Only it's signature.
    pub fn add_hash(&mut self) {
        let mut hasher = DefaultHasher::new();
        let state = (
            (self.white_castle, self.black_castle),
            self.en_passant,
            self.active_player,
            self.grid,
        );
        state.hash(&mut hasher);
        let hash_value = hasher.finish();

        let count = self.draw.board_hashs.entry(hash_value).or_insert(0);
        *count += 1;

        if *count == 3 {
            self.draw.draw_option = Some(Available(TripleRepetition));
            self.draw.draw_hash = Some(hash_value);
        }

        //if the hash we saved was not used : the player can't claim this hash and the 2 previous
        if let Some(h) = self.draw.draw_hash
            && let Some(count) = self.draw.board_hashs.get_mut(&h)
        {
            *count = 0;
        }
    }

    //after 50 moves without pawn moves or capture, it triggers a draw
    pub fn fifty_moves_draw_check(&mut self, from: &Coord, to: &Coord) {
        //if a pawn moved, wthe counter reset
        if let Some(p) = self.get(from).get_piece()
            && p == &Pawn
        {
            self.draw.draw_moves_count = 0;
            return;
        }
        //if a capture occured, we reset the counter
        //this is why we need to call this fct before updating the baord
        if !self.get(to).is_empty() {
            self.draw.draw_moves_count = 0;
            return;
        }
        //incremente in any other case
        self.draw.draw_moves_count += 1;
        //triggers the draw if count reached 50
        if self.draw.draw_moves_count >= 50 {
            self.draw.draw_option = Some(Available(FiftyMoves));
        } else {
            self.draw.draw_option = None;
        }
    }

    //returns a tuple of all the occupied cells on the board
    //the piece occupiying it, it's color, and the "cell color" (for imposible mate situation)
    fn get_pieces_on_board(&mut self) -> Vec<(Piece, Color, Color)> {
        let mut vec = Vec::new();
        for x in 0..8 {
            for y in 0..7 {
                if let Some(piece) = self.grid[x][y].get_piece()
                    && let Some(color) = self.grid[x][y].get_color()
                {
                    let cell_color = if (x + y) % 2 == 0 { White } else { Black };
                    vec.push((*piece, *color, cell_color));
                }
            }
        }
        vec
    }

    //Some hard coded situation where there is no mat legaly possible : draw
    pub fn impossible_mate_check(&mut self) -> bool {
        let pieces = self.get_pieces_on_board();
        // println!("pieces on board : {:?}", pieces);
        match pieces.len() {
            2 => true,
            3 => {
                for (piece, _, _) in pieces {
                    if piece != King && piece != Bishop && piece != Knight {
                        return false;
                    }
                }
                true
            }
            4 => {
                let mut white_bishop_cell_color = None;
                let mut black_bishop_cell_color = None;
                for (piece, color, cell_color) in pieces {
                    if piece != King && piece != Bishop {
                        return false;
                    }
                    if piece == Bishop {
                        if color == White {
                            white_bishop_cell_color = Some(cell_color);
                        } else {
                            black_bishop_cell_color = Some(cell_color);
                        }
                    }
                }
                if white_bishop_cell_color.is_some() != black_bishop_cell_color.is_some() {
                    let Some(cell_1) = white_bishop_cell_color else {
                        return false;
                    };
                    let Some(cell_2) = white_bishop_cell_color else {
                        return false;
                    };
                    if cell_1 == cell_2 {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }


    //Setting up basic chess board
    pub fn fill_side(&mut self, color: Color) {
        let color_idx = match color {
            White => 0,
            Black => 7,
        };
        for x in 0..8 {
            // fill the base line
            self.grid[color_idx][x] = match x {
                0 | 7 => Cell::Occupied(Rook, color),
                1 | 6 => Cell::Occupied(Knight, color),
                2 | 5 => Cell::Occupied(Bishop, color),
                3 => Cell::Occupied(Queen, color),
                4 => Cell::Occupied(King, color),
                _ => unreachable!(),
            };
            // fill the pawns
            match color_idx {
                0 => self.grid[color_idx + 1][x] = Cell::Occupied(Pawn, color),
                7 => self.grid[color_idx - 1][x] = Cell::Occupied(Pawn, color),
                _ => unreachable!(),
            };
            if color_idx == 0 {
                self.grid[color_idx + 1][x] = Cell::Occupied(Pawn, color);
            } else {
                self.grid[color_idx - 1][x] = Cell::Occupied(Pawn, color);
            }
        }
    }

    //Since we need player input to know in which piece promote a pawn, i need to
    //store the coord of the pawn to promote and stop the try move process
    //the GUI will hook on the coord position stored and force player to input a desired promotion
    //Then this hook process the end of try move we skipped earlier
    pub fn promote_pawn(&mut self) {
        let promote_row = if self.active_player == White { 7 } else { 0 };
        for y in 0..8 {
            if self.grid[promote_row][y].is_color(&self.active_player)
                && let Some(piece) = self.grid[promote_row][y].get_piece()
                && *piece == Pawn
            {
                let coord = Coord {
                    row: promote_row as u8,
                    col: y as u8,
                };
                self.pawn_to_promote = Some(coord);
            }
        }
    }
}
