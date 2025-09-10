use crate::Board;
use crate::Color;
use crate::Coord;
use eframe::{App, egui};
use egui::Pos2;
use crate::gui::render::centered_square;
use crate::gui::render::draw_border;
use crate::gui::render::draw_board;
use crate::gui::render::draw_pieces;
use crate::gui::render::draw_dragged_piece;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub active_player: Color,
    pub checkmate: bool,
    //a deplacer ?
    pub turn: u32,
    pub from_cell: Option<Coord>,
    pub drag_from: Option<Coord>,
    pub drag_pos: Option<Pos2>,
    pub piece_legals_moves: Vec<Coord>,
    pub last_move: Option<(Coord, Coord)>,
}

pub struct ChessApp {
    pub current: GameState,
    pub undo: Vec<GameState>,
    pub redo: Vec<GameState>,
    pub flip: bool,
    pub autoflip: bool,
    show_coordinates: bool,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            current: GameState {
                board: Board::init_board(),
                turn: 1,
                from_cell: None,
                active_player: Color::White,
                checkmate: false,
                drag_from: None,
                drag_pos: None,
                piece_legals_moves: Vec::new(),
                last_move: None,
            },
            undo: Vec::new(),
            redo: Vec::new(),
            flip: true,
            autoflip: false,
            show_coordinates: false,
        }
    }
}

//This App trait runs the egui : update is the main loop
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
    fn side_panel_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("ChessGame");
        
        ui.separator();

        ui.label(format!("Turn #{}", self.current.turn));
        ui.label(format!("{:?} to move", self.current.active_player));
        
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
                        self.current.piece_legals_moves.clear();
                    }
                }
                if ui.add_enabled(can_redo, egui::Button::new("Redo")).clicked() {
                    if let Some(next) = self.redo.pop() {
                        self.undo.push(self.current.clone());
                        self.current = next;
                    }
                }
        });
        //
        ui.separator();
        ui.label("last move:");
        if let Some((from, to)) = self.current.last_move {
            ui.monospace(format!("{:?} -> {:?}", from, to));
        } else {
            ui.monospace("—");
        }
    }

    fn central_panel_ui(&mut self, ui: &mut egui::Ui) {
        // 1) Layout & painter
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
        draw_board(&painter, inner, sq, &self.current.piece_legals_moves, &self.current.last_move, self.current.from_cell, self.flip);                     // damier
        draw_pieces(&painter, inner, sq, &self.current.board, self.flip, self.current.drag_from);   
        draw_dragged_piece(&painter, inner, self.current.drag_from, self.current.drag_pos, &self.current.board);

        self.left_click(inner, sq, &response);
        self.right_click(&response);
        self.drag_and_drop(inner, sq, &response);
    }
}



//TO DO
//interface promotion
//moves history
//   - pgn ?
//   - moves history
//   - pieces took
//Replay : rejoue depuis le debut chaque snapshot
