# Algorithms

Implementation notes for the search, evaluation, and ordering techniques used in the engine.

---

## Search

### Alpha-Beta Pruning — [CPW](https://www.chessprogramming.org/Alpha-Beta)

The foundation. Minimax explores the full game tree; alpha-beta prunes subtrees that cannot improve the current best result. A node is cut off as soon as `alpha >= beta`.

The quality of move ordering is the single biggest factor in how many nodes get cut: a perfect first move means every subsequent sibling is immediately pruned. This is why so much effort goes into ordering — see the Move ordering section below.

### Principal Variation Search (PVS) — [CPW](https://www.chessprogramming.org/Principal_Variation_Search)

After searching the first move (the expected best) with the full `[alpha, beta]` window, all subsequent siblings are searched with a null window `[alpha, alpha+1]`. If a sibling beats alpha — an uncommon outcome for a well-ordered move list — it is re-searched with the full window. On average this saves searches, since most late moves confirm they're worse.

### Iterative Deepening — [CPW](https://www.chessprogramming.org/Iterative_Deepening)

Rather than searching to depth N directly, the engine runs depth 1, then 2, then 3, … until the time budget expires. The last completed depth is returned.

Two benefits: it provides a usable result at every interruption point (needed for time management), and the TT move from depth d-1 seeds the ordering at depth d — the "warm-up" effect means iterative deepening is faster than searching depth N cold.

### Aspiration Windows — [CPW](https://www.chessprogramming.org/Aspiration_Windows)

Instead of launching each depth with `[−∞, +∞]`, the search opens with a narrow window around the score returned by the previous depth. If the score falls outside the window (a fail-low or fail-high), the search is re-run with a wider or full window. Works well when the score is stable across depths; saves significant work in practice.

### Null Move Pruning — [CPW](https://www.chessprogramming.org/Null_Move_Pruning)

If a position is already so good that passing (not making a move) still exceeds beta, the actual best move will too. The engine makes a null move (just flips the side to move), searches at depth `d−3`, and prunes if the result is still >= beta. Disabled in zugzwang-prone positions (detected by absence of non-pawn material), in check, and on consecutive plies.

### Late Move Reductions (LMR) — [CPW](https://www.chessprogramming.org/Late_Move_Reductions)

Late moves in a well-ordered list are unlikely to be best. After searching the first few moves at full depth, the engine reduces depth by 1 for quiet, non-critical late moves. If the reduced search still beats alpha, a full-depth re-search confirms. LMR is the biggest source of additional depth gain after alpha-beta.

### Futility Pruning — [CPW](https://www.chessprogramming.org/Futility_Pruning)

At depth 1, if the static evaluation plus a material margin is still below alpha, quiet moves are unlikely to raise it. Those moves are skipped. This prunes a significant portion of the leaf layer without affecting result quality.

### Check Extensions — [CPW](https://www.chessprogramming.org/Check_Extensions)

When a move delivers check, depth is extended by 1. Check positions are tactically sharp — cutting them off early would miss forced-mate sequences and other critical continuations.

### Quiescence Search — [CPW](https://www.chessprogramming.org/Quiescence_Search)

At depth 0 the board is not evaluated statically. Instead, a "calm" position is searched by continuing to explore captures only. This avoids the horizon effect — evaluating a board mid-capture sequence as if it were stable.

Bounded to 4 extra plies to cap the cost.

### Delta Pruning (in quiescence) — [CPW](https://www.chessprogramming.org/Delta_Pruning)

Inside quiescence, a capture is skipped if the material gain plus a delta margin cannot raise alpha. This limits the explosion of capture trees in positions with many cheap exchanges.

---

## Hashing & Memory

### Zobrist Hashing — [CPW](https://www.chessprogramming.org/Zobrist_Hashing)

Each board state maps to a 64-bit hash. Rather than recomputing it at every node, the hash is updated incrementally in `apply_move` and reversed in `undo_move` via XOR: moving a piece XORs out its source square and XORs in its destination. Castling, captures, en passant, and side-to-move each have their own random keys.

The table is initialized once via `thread_local! + OnceLock` and accessed globally with zero overhead.

### Transposition Table — [CPW](https://www.chessprogramming.org/Transposition_Table)

A fixed `Vec<TtEntry>` of 1M slots, indexed by `hash & (TT_SIZE - 1)`. Each entry stores:

- **key** — full Zobrist hash for collision detection
- **depth** — how deep the entry was searched
- **score** — the value at that depth
- **flag** — `Exact`, `LowerBound`, or `UpperBound` (depending on whether alpha/beta cut)
- **best_move** — used to seed move ordering at the next depth
- **generation** — incremented between games; stale entries are ignored without clearing the table

TT probe happens before move generation. TT store happens after the search, overwriting only if the incoming entry searched deeper or is from the current generation.

---

## Move Ordering

Good ordering turns alpha-beta into a near-linear search. The engine scores each move before sorting:

### TT Move — [CPW](https://www.chessprogramming.org/Transposition_Table)

The best move stored from a previous search at the same position gets the highest priority. This is the single most impactful ordering technique: the TT move causes an immediate beta cutoff most of the time.

### MVV-LVA — [CPW](https://www.chessprogramming.org/MVV-LVA)

Captures are scored by Most Valuable Victim / Least Valuable Attacker: capturing a queen with a pawn is scored higher than capturing a pawn with a queen. Implemented as a simple lookup table.

### Killer Move Heuristic — [CPW](https://www.chessprogramming.org/Killer_Move)

Quiet moves that caused a beta cutoff at the same depth (but in a sibling node) are stored as "killers" (2 per depth). They are tried early in move ordering, before other quiet moves.

### History Heuristic — [CPW](https://www.chessprogramming.org/History_Heuristic)

Every quiet move that causes a beta cutoff increments a `history[from][to]` counter. This score accumulates across the whole search and acts as a soft signal for which quiet moves tend to be strong, independent of position.

---

## Evaluation

### Piece-Square Tables — [CPW](https://www.chessprogramming.org/Piece-Square_Tables)

Each piece has two PST tables: one for the opening and one for the endgame. The active table is interpolated by a game phase score computed from remaining material. Knights prefer central squares; kings prefer the corner in the opening and the center in the endgame; pawns reward advancement.

The PST score is maintained incrementally in `apply_move` / `undo_move` alongside the material score — no recomputation at any depth.

### King Safety — [CPW](https://www.chessprogramming.org/King_Safety)

The area around the king (the "king zone") is checked for pawn shield strength and the number of enemy attackers. Missing pawn shields and multiple attackers are penalized. Only applied in the middlegame (high material), since open-king safety matters less in endgames.

### Pin Detection — [CPW](https://www.chessprogramming.org/Pin)

Absolute pins are detected by ray casting from the king outward. A piece is pinned if removing it exposes the king to a sliding attacker. Pinned pieces receive a mobility penalty in evaluation.

### Mop-up Evaluation — [CPW](https://www.chessprogramming.org/Mop-up_Evaluation)

In winning endgames (large material advantage), the engine adds a bonus for pushing the enemy king toward the corner and closing the distance between kings. Without this, the engine may stall in winning positions and fail to convert.

### King Corner Pressure — [CPW](https://www.chessprogramming.org/King_Centralization)

Tracks how far the losing king is from the board center and adds a bonus for cornering it. Complements mop-up by rewarding king centralization for the winning side.

### Rook & Bishop Cut Bonus — [CPW](https://www.chessprogramming.org/Rook_Endgames)

In rook and bishop endgames, bonuses reward cutting the enemy king's escape routes — rooks on open files near the king, bishops controlling key diagonals. Helps the engine convert drawn-looking positions correctly.
