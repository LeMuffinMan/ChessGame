"""Phase 1 — génération d'un dataset (FEN, eval Stockfish) pour l'entraînement NNUE.

Convention : eval_cp est TOUJOURS du point de vue des Blancs (positif = bon
pour les Blancs), pour rester cohérent avec `evaluator::evaluate()` côté Rust.

Usage:
    python generate_dataset.py \
        --pgn lichess_elite_2024-01.pgn \
        --stockfish ./stockfish \
        --out data/dataset.csv \
        --positions-per-game 3 \
        --nodes 200000 \
        --workers 8
"""

from __future__ import annotations

import argparse
import csv
import multiprocessing as mp
import random
import time
from pathlib import Path

import chess
import chess.engine
import chess.pgn

MIN_PLY = 10  # nb de demi-coups à sauter en début de partie (théorie d'ouverture)
MAX_PLY_FROM_END = 10  # nb de demi-coups à sauter en fin de partie (positions déjà décidées)
MATE_SCORE_CP = 3000  # cap pour les scores de mat, avant le squash sigmoïde de la Phase 2

_engine: chess.engine.SimpleEngine | None = None


def _init_worker(stockfish_path: str) -> None:
    """Ouvre UN process Stockfish par worker (appelé une fois par process du Pool)."""
    global _engine
    _engine = chess.engine.SimpleEngine.popen_uci(stockfish_path)


def _eval_fen(args: tuple[str, int]) -> tuple[str, int] | None:
    fen, nodes = args
    board = chess.Board(fen)
    assert _engine is not None
    try:
        info = _engine.analyse(board, chess.engine.Limit(nodes=nodes))
    except chess.engine.EngineError:
        return None
    eval_cp = info["score"].white().score(mate_score=MATE_SCORE_CP)
    return fen, eval_cp


def normalized_fen(board: chess.Board) -> str:
    """FEN sans les compteurs de coups (halfmove/fullmove) pour dédupliquer
    les vraies transpositions, pas juste des FEN textuellement différents."""
    return " ".join(board.fen().split(" ")[:4])


def sample_positions(game: chess.pgn.Game, n_samples: int, rng: random.Random) -> list[str]:
    board = game.board()
    positions: list[chess.Board] = []
    for move in game.mainline_moves():
        board.push(move)
        positions.append(board.copy(stack=False))

    candidates = [
        b
        for i, b in enumerate(positions)
        if MIN_PLY <= i < len(positions) - MAX_PLY_FROM_END and not b.is_check()
    ]
    if not candidates:
        return []
    k = min(n_samples, len(candidates))
    return [normalized_fen(b) for b in rng.sample(candidates, k)]


def iter_sampled_fens(pgn_path: Path, positions_per_game: int, seed: int):
    """Stream les positions échantillonnées depuis un gros fichier PGN,
    une partie à la fois (jamais tout le fichier en mémoire)."""
    rng = random.Random(seed)
    seen: set[str] = set()
    with pgn_path.open(encoding="utf-8", errors="ignore") as f:
        game_index = 0
        while True:
            game = chess.pgn.read_game(f)
            if game is None:
                break
            game_index += 1
            for fen in sample_positions(game, positions_per_game, rng):
                if fen not in seen:
                    seen.add(fen)
                    yield game_index, fen


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pgn", type=Path, required=True)
    parser.add_argument("--stockfish", type=str, default="./stockfish")
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--positions-per-game", type=int, default=3)
    parser.add_argument("--nodes", type=int, default=200_000)
    parser.add_argument("--workers", type=int, default=mp.cpu_count())
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--max-positions", type=int, default=None)
    args = parser.parse_args()

    args.out.parent.mkdir(parents=True, exist_ok=True)

    fen_stream = iter_sampled_fens(args.pgn, args.positions_per_game, args.seed)
    game_by_fen: dict[str, int] = {}

    def tasks():
        count = 0
        for game_index, fen in fen_stream:
            if args.max_positions is not None and count >= args.max_positions:
                break
            game_by_fen[fen] = game_index
            count += 1
            yield fen, args.nodes

    start = time.time()
    n_written = 0
    with args.out.open("w", newline="", encoding="utf-8") as out_f:
        writer = csv.writer(out_f)
        writer.writerow(["fen", "eval_cp", "game_index"])

        with mp.Pool(
            processes=args.workers,
            initializer=_init_worker,
            initargs=(args.stockfish,),
        ) as pool:
            for result in pool.imap_unordered(_eval_fen, tasks(), chunksize=16):
                if result is None:
                    continue
                fen, eval_cp = result
                writer.writerow([fen, eval_cp, game_by_fen[fen]])
                n_written += 1
                if n_written % 1000 == 0:
                    elapsed = time.time() - start
                    print(f"{n_written} positions écrites ({n_written / elapsed:.1f} pos/s)")

    elapsed = time.time() - start
    print(f"\nTerminé : {n_written} positions écrites dans {args.out} en {elapsed:.1f}s "
          f"({n_written / elapsed:.1f} pos/s en moyenne)")


if __name__ == "__main__":
    main()
