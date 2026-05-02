# Architecture

## Module overview

```
src/
├── board/                   — board representation and move logic
│   ├── cell.rs              — Cell, Piece, Color, Coord types
│   ├── mod.rs               — Board struct, Index<Coord> trait impl
│   ├── fen.rs               — FEN parser (board_from_fen)
│   ├── update_board.rs      — incremental score and material tracking
│   ├── threat.rs            — threat map generation
│   ├── is_king_exposed.rs   — legality check via ray casting
│   ├── pin_detection.rs     — absolute pin detection
│   ├── try_move.rs          — move validation entry point
│   ├── utils.rs
│   └── moves/
│       ├── move_structs.rs  — Move, MoveType, MoveList
│       ├── move_gen.rs      — generate_moves (all pieces)
│       ├── apply_move.rs    — apply_move with incremental Zobrist + score
│       ├── undo_move.rs     — restore previous board
│       └── pieces_moves/    — per-piece move generators
│           ├── pawn_moves.rs
│           ├── knight_moves.rs
│           ├── king_moves.rs
│           └── sliding_pieces_moves.rs
│
├── engine/                  — search, evaluation, memory
│   ├── minimax.rs           — alpha-beta, PVS, LMR, null move, quiescence
│   ├── evaluator.rs         — evaluate(), king safety, mop-up, PST blending
│   ├── move_ordering.rs     — MVV-LVA, killer, history, TT move scoring
│   ├── search_context.rs    — SearchContext, SearchParams, HistoryTable, KillerTable
│   ├── search_stats.rs      — SearchStats (nodes, cutoffs, TT hits…)
│   ├── ttentry.rs           — TtEntry, TtFlag (Exact / LowerBound / UpperBound)
│   ├── zobris_table.rs      — ZobristTable (thread_local + OnceLock)
│   ├── pst_maps.rs          — piece-square tables (opening + endgame)
│   ├── bench.rs             — bench infrastructure, WASM exports
│   ├── bot.rs               — bot move dispatch (difficulty → depth + time budget)
│   └── tests.rs             — engine unit tests
│
├── game/
│   └── mod.rs               — Game struct: try_move, history Vec<Move>, draw conditions
│
├── gui/                     — egui UI, zero engine logic
│   ├── chessapp.rs          — ChessApp, eframe::App impl
│   ├── layout.rs            — desktop / mobile layout switch
│   ├── render.rs            — board rendering, highlights, hints
│   ├── inputs.rs            — click handling, drag
│   ├── styles.rs            — colors, fonts
│   ├── panels/              — left, right, top, central, bot panels
│   ├── components/          — reusable widgets (timer, engine infos, buttons…)
│   ├── features/            — replay, PGN export, settings, timer logic
│   └── hooks/               — promotion window, modal hooks
│
├── bin/
│   └── bench.rs             — native benchmark binary (stdout JSON)
│
├── lib.rs                   — WASM entry point (wasm_bindgen exports)
└── main.rs                  — native entry point
```

---

## Key design decisions

### Board

`Board` is a stack-only struct: `[[Cell; 8]; 8]`, castling rights, en passant `Option<Coord>`, Zobrist hash `u64`, incremental score `i32`, king positions. No heap allocation: copy is cheap.

`Cell` is `enum Cell { Occupied(Piece, Color), Free }`. Exhaustive pattern matching throughout; null states are impossible by construction.

`Board` implements `Index<Coord>` and `IndexMut<Coord>`, eliminating `as usize` casts in hot paths.

### apply / undo

`apply_move` and `undo_move` maintain the board state incrementally: the Zobrist hash is XOR-updated per piece/square, and the material + PST score is updated as a delta. No full recomputation at any depth.

This makes the search loop allocation-free: no cloning, no heap, just stack frames.

### Engine / GUI separation

`engine/` and `game/` have zero dependency on `egui`. The GUI (`ChessApp`) calls `ChessGame::try_move` and `bot::find_move`; it contains no chess rules. This boundary makes it possible to run the engine headless (`src/bin/bench.rs`) and will allow a UCI binary (`src/bin/uci.rs`) without touching the GUI.

### Transposition Table

A `Vec<TtEntry>` of fixed size 1M, indexed by `hash & (TT_SIZE - 1)`. Each entry stores the Zobrist key for collision detection and a generation counter to invalidate stale entries between games without clearing the table.

### Zobrist hashing

A `ZobristTable` is initialized once via `thread_local! + OnceLock` and accessed globally. The hash is updated incrementally in `apply_move` (XOR per piece/square moved, captured, castled) and reversed in `undo_move`.

---

## Data flow: one bot move

```
ChessApp::update()
  └── bot::find_move(board, color, difficulty)
        └── minimax::timed_out_iterative_deepening(...)
              ├── aspiration_search(depth, prev_score)
              │     └── find_best_move(alpha, beta)
              │           └── minimax(depth-1, ...)  [recursive]
              │                 ├── TT probe
              │                 ├── null move pruning
              │                 ├── generate_moves + sort (MVV-LVA, killers, history, TT)
              │                 ├── LMR / PVS per child
              │                 ├── check extension
              │                 ├── quiescence_minimax at depth=0
              │                 └── TT store
              └── best_move → ChessApp → GUI highlight
```
