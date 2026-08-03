use crate::ChessApp;
use crate::board::cell::Color;
use crate::board::cell::Coord;
use crate::board::cell::Piece;
use crate::board::cell::Piece::*;
use crate::board::moves::move_structs::CastleSide;
use crate::board::moves::move_structs::CastleSide::*;
use crate::board::moves::move_structs::MoveType;
use crate::game::Game;
use crate::gui::chessapp::AppMode;
use crate::gui::features::replay::ReplayInfos;

struct SanMove {
    piece: Piece,
    disambig_col: Option<u8>,
    disambig_row: Option<u8>,
    dest: Coord,
    promotion: Option<Piece>,
    castle: Option<CastleSide>,
}

/// Strips tag pairs (`[Event "..."]`) and end-of-line `;` comments, joining
/// the remaining movetext into a single string.
fn strip_tags_and_line_comments(pgn: &str) -> String {
    pgn.lines()
        .filter(|line| !line.trim_start().starts_with('['))
        .map(|line| match line.find(';') {
            Some(idx) => &line[..idx],
            None => line,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Removes every balanced `open`/`close` span (nesting-aware), e.g. `{...}`
/// comments or `(...)` variations.
fn strip_delimited(text: &str, open: char, close: char) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth = 0u32;
    for c in text.chars() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth = depth.saturating_sub(1);
        } else if depth == 0 {
            out.push(c);
        }
    }
    out
}

/// Splits cleaned movetext into SAN move tokens, separating out the trailing
/// result token (`1-0`/`0-1`/`1/2-1/2`/`*`) if present.
fn tokenize_movetext(pgn: &str) -> (Vec<String>, Option<String>) {
    let no_tags = strip_tags_and_line_comments(pgn);
    let no_comments = strip_delimited(&no_tags, '{', '}');
    let no_variations = strip_delimited(&no_comments, '(', ')');

    let mut tokens = Vec::new();
    let mut result_token = None;

    for raw in no_variations.split_whitespace() {
        if raw.starts_with('$') {
            continue; // NAG
        }
        if matches!(raw, "1-0" | "0-1" | "1/2-1/2" | "*") {
            result_token = Some(raw.to_string());
            continue;
        }
        let after_number = raw.trim_start_matches(|c: char| c.is_ascii_digit());
        let after_number = if after_number.len() != raw.len() {
            after_number.trim_start_matches('.')
        } else {
            raw
        };
        if after_number.is_empty() {
            continue;
        }
        tokens.push(after_number.to_string());
    }

    (tokens, result_token)
}

fn parse_san_token(token: &str) -> Option<SanMove> {
    let core = token.trim_end_matches(['+', '#', '!', '?']);
    if core.is_empty() {
        return None;
    }
    if core == "O-O" || core == "0-0" {
        return Some(SanMove {
            piece: King,
            disambig_col: None,
            disambig_row: None,
            dest: Coord { row: 0, col: 0 },
            promotion: None,
            castle: Some(Right),
        });
    }
    if core == "O-O-O" || core == "0-0-0" {
        return Some(SanMove {
            piece: King,
            disambig_col: None,
            disambig_row: None,
            dest: Coord { row: 0, col: 0 },
            promotion: None,
            castle: Some(Left),
        });
    }

    let (body, promotion) = match core.find('=') {
        Some(idx) => {
            let promo = match core[idx + 1..].chars().next()? {
                'Q' => Queen,
                'R' => Rook,
                'B' => Bishop,
                'N' => Knight,
                _ => return None,
            };
            (&core[..idx], Some(promo))
        }
        None => (core, None),
    };

    let mut chars = body.chars();
    let first = chars.next()?;
    let (piece, rest) = match first {
        'N' => (Knight, &body[1..]),
        'B' => (Bishop, &body[1..]),
        'R' => (Rook, &body[1..]),
        'Q' => (Queen, &body[1..]),
        'K' => (King, &body[1..]),
        _ => (Pawn, body),
    };

    let square_chars: Vec<char> = rest.chars().filter(|&c| c != 'x').collect();
    let n = square_chars.len();
    if n < 2 {
        return None;
    }
    let dest_file = square_chars[n - 2];
    let dest_rank = square_chars[n - 1];
    if !dest_file.is_ascii_lowercase() || !('a'..='h').contains(&dest_file) {
        return None;
    }
    if !('1'..='8').contains(&dest_rank) {
        return None;
    }
    let dest = Coord {
        col: dest_file as u8 - b'a',
        row: dest_rank as u8 - b'1',
    };

    let mut disambig_col = None;
    let mut disambig_row = None;
    for &c in &square_chars[..n - 2] {
        if ('a'..='h').contains(&c) {
            disambig_col = Some(c as u8 - b'a');
        } else if ('1'..='8').contains(&c) {
            disambig_row = Some(c as u8 - b'1');
        }
    }

    Some(SanMove {
        piece,
        disambig_col,
        disambig_row,
        dest,
        promotion,
        castle: None,
    })
}

fn resolve_move(game: &Game, san: &SanMove) -> Option<(Coord, Coord)> {
    if let Some(side) = san.castle {
        let m = game
            .legals_moves
            .iter()
            .find(|m| matches!(m.move_type, MoveType::Castle(s) if s == side))?;
        return Some((m.origin, m.dest));
    }

    let mut candidates = game.legals_moves.iter().filter(|m| {
        m.dest == san.dest
            && game.board.get(&m.origin).get_piece() == Some(&san.piece)
            && san.disambig_col.is_none_or(|c| m.origin.col == c)
            && san.disambig_row.is_none_or(|r| m.origin.row == r)
            && match san.promotion {
                Some(p) => m.move_type == MoveType::Promotion(p),
                None => !matches!(m.move_type, MoveType::Promotion(_)),
            }
    });

    let m = candidates.next()?;
    if candidates.next().is_some() {
        return None; // ambiguous, PGN should have disambiguated further
    }
    Some((m.origin, m.dest))
}

impl ChessApp {
    /// Imports a single PGN game (tag pairs optional) into a fresh `Game`,
    /// replaying every move so the app ends up in the same state as if the
    /// moves had been played by hand. Purely text-based, used identically on
    /// wasm32 (pasted text) and native (pasted text or a file loaded via
    /// `import_pgn_from_path`). Nothing here touches the NNUE ML dataset
    /// pipeline, which stays Python-only.
    pub fn import_pgn(&mut self, pgn_text: &str) -> Result<(), String> {
        let (tokens, result_token) = tokenize_movetext(pgn_text);
        if tokens.is_empty() {
            return Err("No moves found in this PGN.".to_string());
        }

        let mut game = Game::new();
        let mut history_san = String::new();

        for (i, token) in tokens.iter().enumerate() {
            let san = parse_san_token(token)
                .ok_or_else(|| format!("Move {} unreadable: \"{token}\"", i + 1))?;
            let (origin, dest) = resolve_move(&game, &san)
                .ok_or_else(|| format!("Move {} illegal or ambiguous: \"{token}\"", i + 1))?;

            if game.active_player == Color::White {
                history_san.push_str(&game.turn.to_string());
                history_san.push_str(". ");
            }

            let event = match san.promotion {
                Some(p) => game.try_move_promotion(origin, dest, p),
                None => game.try_move(origin, dest),
            };
            if event.is_none() {
                return Err(format!("Move {} rejected by the engine: \"{token}\"", i + 1));
            }

            history_san.push_str(token.trim_end_matches(['!', '?']));
            history_san.push(' ');
        }

        if let Some(result) = result_token {
            history_san.push_str(&result);
        }

        self.game = game;
        self.history_san = history_san;
        self.replay_infos = ReplayInfos::new();
        self.replay_infos.index = self.game.history.len();
        self.last_move = self.game.history.last().map(|m| (m.origin, m.dest));
        self.promoteinfo = None;
        self.app_mode = AppMode::Replay;
        self.settings.pgn_import_error = None;

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ChessApp {
    pub fn import_pgn_from_path(&mut self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Unable to read \"{path}\": {e}"))?;
        self.import_pgn(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Board;
    use crate::gui::layout::UiType;

    fn c(col: u8, row: u8) -> Coord {
        Coord { col, row }
    }

    fn play(app: &mut ChessApp, moves: &[(Coord, Coord)]) {
        for &(from, to) in moves {
            app.try_move(from, to);
        }
    }

    /// Compares only piece placement/castling/en-passant (not move counters,
    /// which are a `Game` concern already covered by `history.len()`).
    fn board_fen(board: &Board) -> String {
        board.to_fen(Color::White, 0, 1)
    }

    #[test]
    fn round_trip_ruy_lopez_with_castle() {
        let mut app = ChessApp::new(UiType::Desktop);
        play(
            &mut app,
            &[
                (c(4, 1), c(4, 3)), // e4
                (c(4, 6), c(4, 4)), // e5
                (c(6, 0), c(5, 2)), // Nf3
                (c(1, 7), c(2, 5)), // Nc6
                (c(5, 0), c(1, 4)), // Bb5
                (c(0, 6), c(0, 5)), // a6
                (c(1, 4), c(0, 3)), // Ba4
                (c(6, 7), c(5, 5)), // Nf6
                (c(4, 0), c(6, 0)), // O-O
                (c(5, 7), c(4, 6)), // Be7
            ],
        );

        let pgn = app.history_san.clone();
        let expected_fen = board_fen(&app.game.board);
        let expected_len = app.game.history.len();

        let mut reimported = ChessApp::new(UiType::Desktop);
        reimported.import_pgn(&pgn).expect("import should succeed");

        assert_eq!(reimported.game.history.len(), expected_len);
        assert_eq!(board_fen(&reimported.game.board), expected_fen);
    }

    #[test]
    fn import_promotion_and_underpromotion_captures() {
        // a/h-pawn race: both sides capture into the back rank, one queening
        // and one underpromoting, exercising the promotion + capture SAN
        // parsing path end to end.
        let pgn = "1. a4 h5 2. a5 h4 3. a6 h3 4. axb7 hxg2 5. bxa8=Q gxh1=N";

        let mut app = ChessApp::new(UiType::Desktop);
        app.import_pgn(pgn).expect("import with promotions should succeed");

        assert_eq!(app.game.history.len(), 10);
        assert_eq!(
            app.game.board.get(&c(0, 7)).get_piece(),
            Some(&crate::board::cell::Piece::Queen)
        );
        assert_eq!(
            app.game.board.get(&c(7, 0)).get_piece(),
            Some(&crate::board::cell::Piece::Knight)
        );
    }

    #[test]
    fn tokenizer_strips_annotations() {
        let raw = "[Event \"Test\"]\n[Site \"?\"]\n\n1. e4 {best by test} e5 2. Nf3 $1 Nc6 (2... d6 3. d4) 3. Bb5 a6 1-0";
        let (tokens, result) = tokenize_movetext(raw);
        assert_eq!(tokens, vec!["e4", "e5", "Nf3", "Nc6", "Bb5", "a6"]);
        assert_eq!(result, Some("1-0".to_string()));
    }

    #[test]
    fn rejects_illegal_move() {
        // No bishop on f1's single available diagonal (e2-d3-c4-b5-a6) can
        // ever reach b4, so this must fail to resolve.
        let mut app = ChessApp::new(UiType::Desktop);
        let err = app.import_pgn("1. e4 e5 2. Bb4").unwrap_err();
        assert!(err.contains("Bb4"), "unexpected error message: {err}");
    }
}
