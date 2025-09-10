use crate::Board;
use crate::Color;
use crate::Coord;
use crate::cell::Piece::*;
use crate::gui::render::{centered_square, draw_border, draw_board, draw_pieces, draw_dragged_piece};

use eframe::{App, egui};
use egui::Pos2;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub active_player: Color,
    pub checkmate: bool,
    pub pat: bool,
    pub last_move: Option<(Coord, Coord)>,
    pub last_move_pgn: String,
    pub turn: u32,
}

pub struct ChessApp {
    //history undo / redo
    pub current: GameState,
    pub undo: Vec<GameState>,
    pub redo: Vec<GameState>,
    next_replay_time: Option<Instant>,
    //gui options
    pub flip: bool,
    pub autoflip: bool,
    replay_speed: u64,
    show_coordinates: bool,
    //gui cell colors
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            current: GameState {
                board: Board::init_board(),
                active_player: Color::White,
                checkmate: false,
                pat: false,
                last_move: None,
                last_move_pgn: String::new(),
                turn: 1,
            },
            undo: Vec::new(),
            redo: Vec::new(),
            next_replay_time: None,
            replay_speed: 1000,
            flip: true,
            autoflip: false,
            show_coordinates: false,
            from_cell: None,
            drag_from: None,
            drag_pos: None,
            piece_legals_moves: Vec::new(),
        }
    }
}


//This App trait runs the eframe : fn update is the main loop, run for each frame
impl App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .default_width(150.0)
            .show(ctx, |ui| {
                self.side_panel_ui(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel_ui(ui);
            });
    }
}

impl ChessApp {
    fn replay_step(&mut self, ctx: &egui::Context) {
        if let Some(next_time) = self.next_replay_time {
            if Instant::now() >= next_time {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(self.current.clone());
                    self.current = next;
                    self.next_replay_time = Some(Instant::now() + Duration::from_millis(self.replay_speed));
                    ctx.request_repaint(); 
                } else {
                    self.next_replay_time = None;
                }
            } else {
                ctx.request_repaint();            }
        }
    }



    pub fn from_move_to_pgn(&mut self, move_to_encode: (Coord, Coord)) {
        let (from, to) = move_to_encode;
        let piece_char = match self.current.board.get(&from).get_piece() {
            Some(Pawn)  => None,
            Some(Rook)  => Some('R'),
            Some(Knight)=> Some('N'),
            Some(Bishop)=> Some('B'),
            Some(Queen) => Some('Q'),
            Some(King)  => Some('K'),
            None        => Some('?'),
        };

        let is_capture = !self.current.board.get(&to).is_empty();

        let dest_col = (b'a' + to.col as u8) as char; 
        let dest_row = char::from_digit((to.row + 1) as u32, 10).unwrap(); 

        self.current.last_move_pgn = String::new();
        if let Some(pc) = piece_char {
            self.current.last_move_pgn.push(pc);
        } else if is_capture {
            let src_col = (b'a' + from.col as u8) as char;
            self.current.last_move_pgn.push(src_col);
        }

        if is_capture {
            self.current.last_move_pgn.push('x');
        }

        self.current.last_move_pgn.push(dest_col);
        self.current.last_move_pgn.push(dest_row);
        //ajouter le check
        //ajouter le mat / pat
        //ajouter la promotion
        //ajouter ambiguite
        //ajouter les roques
    } 


    fn side_panel_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("ChessGame");
        ui.separator();
        ui.label(format!("Turn #{}", self.current.turn));
        if self.current.checkmate {
            let color = if self.current.active_player == Color::White { Color::Black } else { Color::White };
            ui.label(format!("Checkmate ! {:?} win", color));
        } else if self.current.pat {
            ui.label(format!("Pat !"));
        } else {
            ui.label(format!("{:?} to move", self.current.active_player));
        }
        ui.separator();
        if ui.button("New game").clicked() {
            *self = ChessApp::default();
        }
        ui.separator();
        if ui.button("Flip board").clicked() {
            self.flip = !self.flip;
        }
        if ui.toggle_value(&mut self.autoflip, "Autoflip").changed() {
        }
        ui.separator();
        if ui.checkbox(&mut self.show_coordinates, "Coordinates").changed() {

        }
        ui.separator();
        ui.horizontal(|ui| {
            let can_undo = !self.undo.is_empty();
            let can_redo = !self.redo.is_empty();
            if ui.add_enabled(can_undo, egui::Button::new("Undo")).clicked() {
                if let Some(prev) = self.undo.pop() {
                    self.redo.push(self.current.clone());
                    self.current = prev;
                    self.piece_legals_moves.clear();
                }
            }
            if ui.add_enabled(can_redo, egui::Button::new("Redo")).clicked() {
                if let Some(next) = self.redo.pop() {
                    self.undo.push(self.current.clone());
                    self.current = next;
                }
            }
            if ui.add_enabled(!self.undo.is_empty(), egui::Button::new("Replay")).clicked() {
                self.redo.clear();
                self.redo.push(self.current.clone());
                while let Some(prev) = self.undo.pop() {
                    self.redo.push(prev.clone());
                    if self.undo.is_empty() {
                        self.current = prev;
                    }
                }
                self.next_replay_time = Some(Instant::now() + Duration::from_millis(self.replay_speed));
            }
            self.replay_step(ui.ctx());

        });
        if let Some(_) = self.next_replay_time {
            ui.add(
                egui::Slider::new(&mut self.replay_speed, 100..=2000)
                    .text("Speed (ms)")
                    .logarithmic(true)
            );
        }
        ui.separator();
        ui.label("last move:");
        ui.monospace(&self.current.last_move_pgn);
    }

    fn central_panel_ui(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click_and_drag());
        let rect = response.rect;

        let board_rect = centered_square(rect);              
        let inner = if self.show_coordinates {
            draw_border(&painter, board_rect);                   
            board_rect.shrink(16.0)
        } else { board_rect };

        let sq = inner.width() / 8.0;

        if self.show_coordinates { self.show_coordinates(&painter, inner, sq); }
        draw_board(&painter, inner, sq, &self.piece_legals_moves, &self.current.last_move, self.from_cell, self.flip);  
        draw_pieces(&painter, inner, sq, &self.current.board, self.flip, self.drag_from);   
        draw_dragged_piece(&painter, inner, self.drag_from, self.drag_pos, &self.current.board);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }
}


