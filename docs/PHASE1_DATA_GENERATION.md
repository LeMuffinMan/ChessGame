# Phase 1 — Génération de données (FEN + eval Stockfish)

Cette note détaille la Phase 1 du projet [NNUE](../NNUE.md) : produire un
dataset de positions d'échecs (FEN) labellisées par l'eval Stockfish, pour
entraîner en Phase 2 un petit réseau de neurones qui remplacera à terme
`evaluate()` (`src/engine/evaluator.rs`).

Convention actée : **`eval_cp` est toujours du point de vue des Blancs**
(positif = bon pour les Blancs), pour rester cohérent avec `evaluate()`
côté Rust, qui retourne déjà un score absolu.

Code de référence pour ce type de pipeline (pour aller plus loin) : le
dépôt officiel `official-stockfish/nnue-pytorch` sur GitHub, qui documente
la génération de données et l'entraînement NNUE tels que faits par
Stockfish lui-même.

## Vue d'ensemble du pipeline

```
Base PGN (Lichess Elite)
   │  python-chess : chess.pgn.read_game() en streaming
   ▼
Parties individuelles
   │  échantillonnage : 1-3 positions "quiet" par partie
   ▼
Positions candidates (FEN)
   │  dédoublonnage (FEN normalisé)
   ▼
Positions uniques
   │  Stockfish (UCI, nodes fixes) — parallélisé sur N workers
   ▼
(fen, eval_cp, game_index)
   │  écriture CSV en streaming
   ▼
dataset.csv
   │  split par PARTIE (pas par position !)
   ▼
train.csv / val.csv / test.csv
```

## Étape 1 — Récupérer une base de parties

On utilise la **Lichess Elite Database** plutôt que la base Lichess
complète : celle-ci fait des centaines de Go et contient énormément de
parties bullet/blitz de joueurs faibles, une distribution de positions peu
utile à apprendre. La Elite Database est un sous-ensemble mensuel curaté
(joueurs >2000 Elo, cadences longues), quelques centaines de Mo par mois
téléchargé — largement suffisant pour ce projet.

```bash
# à la racine du repo (le fichier .pgn est déjà ignoré par git, cf. .gitignore)
curl -LO https://example-mirror/lichess_elite_2024-01.zip
unzip lichess_elite_2024-01.zip
```

*(remplace l'URL par le mirroir Lichess Elite Database de ton choix — le
nom exact du fichier dépend du mois téléchargé)*

**Question importante** : un seul mois suffit largement pour une v1 (des
centaines de milliers de parties). Pas besoin de tout télécharger d'un
coup — on peut relancer le script sur un second mois si le dataset généré
est trop petit.

## Étape 2 — Lire les parties PGN en streaming

Un fichier PGN de plusieurs centaines de Mo ne doit **jamais** être chargé
intégralement en mémoire. `python-chess` fournit `chess.pgn.read_game(f)`
qui lit une partie à la fois depuis un objet fichier et retourne `None` à
la fin du fichier :

```python
import chess.pgn

with open("lichess_elite_2024-01.pgn", encoding="utf-8", errors="ignore") as f:
    while True:
        game = chess.pgn.read_game(f)
        if game is None:
            break
        # traiter `game` ici, puis game est libéré avant la prochaine itération
```

`game.mainline_moves()` donne l'itérateur des coups joués (on ignore les
variantes annotées). On rejoue ces coups sur un `chess.Board()` pour
reconstruire chaque position de la partie.

## Étape 3 — Échantillonner des positions "quiet" par partie

**Théorie** : on ne veut *pas* prendre toutes les positions d'une partie.
Deux raisons :
- Les positions consécutives d'une même partie sont extrêmement corrélées
  (quasi le même contenu informationnel) — les inclure toutes gonfle le
  dataset sans ajouter de diversité réelle.
- L'ouverture est déjà massivement surreprésentée dans n'importe quelle
  base de parties (les 10 premiers coups de la Sicilienne apparaissent des
  millions de fois) ; sans filtrage, le réseau apprendrait surtout à bien
  évaluer des ouvertures déjà connues plutôt que des positions variées de
  milieu/fin de partie.

Règles appliquées (mêmes conventions que les pipelines NNUE existants,
Chess Programming Wiki) :
- Sauter les `MIN_PLY = 10` premiers demi-coups (théorie d'ouverture).
- Sauter les `MAX_PLY_FROM_END = 10` derniers demi-coups (positions déjà
  décidées / mats imminents, peu informatives pour une eval "générale").
- Exclure les positions où le joueur au trait est en échec (une position
  en échec n'est pas une position "stable" — l'eval y est structurellement
  différente, ce n'est pas un bon exemple statique à apprendre).
- Tirer 1 à 3 positions **aléatoires** parmi les candidates restantes,
  plutôt que toutes — garde la diversité sans surreprésenter les parties
  longues.

```python
def sample_positions(game, n_samples, rng):
    board = game.board()
    positions = []
    for move in game.mainline_moves():
        board.push(move)
        positions.append(board.copy(stack=False))  # copy(stack=False) = plus léger

    candidates = [
        b for i, b in enumerate(positions)
        if MIN_PLY <= i < len(positions) - MAX_PLY_FROM_END and not b.is_check()
    ]
    if not candidates:
        return []
    k = min(n_samples, len(candidates))
    return [normalized_fen(b) for b in rng.sample(candidates, k)]
```

**Question importante (raffinement possible, pas fait en v1)** : on
pourrait aussi exclure les positions "tactiques" (une capture évidente en
cours) puisque l'eval statique d'une position en pleine séquence
d'échanges est moins significative. Stockfish le fait dans son propre
pipeline d'entraînement via une petite recherche de vérification. On s'en
passe pour la v1 pour garder le script simple — à raffiner si les
résultats de Phase 2 sont décevants.

## Étape 4 — Dédupliquer par FEN normalisé

**Théorie** : le FEN complet contient les compteurs de demi-coups et de
coups complets (`... 0 12`), qui changent à chaque coup même si la
position stratégique est identique (transposition). Dédupliquer sur le FEN
brut raterait donc les vraies transpositions. On normalise en ne gardant
que les 4 premiers champs (position des pièces, trait, droits de roque,
en passant) :

```python
def normalized_fen(board: chess.Board) -> str:
    return " ".join(board.fen().split(" ")[:4])
```

Le dédoublonnage se fait via un `set()` de FEN déjà vus, au fil du
streaming (pas besoin de tout charger pour comparer).

## Étape 5 — Labelliser avec Stockfish (UCI)

**Théorie** : `python-chess` pilote Stockfish via le protocole UCI avec
`chess.engine.SimpleEngine.popen_uci(path)`. On demande une analyse avec
une limite en **nombre de nœuds** plutôt qu'en temps :

```python
engine = chess.engine.SimpleEngine.popen_uci("./stockfish")
info = engine.analyse(board, chess.engine.Limit(nodes=200_000))
eval_cp = info["score"].white().score(mate_score=3000)
```

Pourquoi `nodes` et pas `movetime` : une limite de temps dépend de la
charge de la machine au moment de l'exécution (donc pas reproductible
d'une run à l'autre, ni comparable entre les deux laptops) ; une limite de
nœuds donne un budget de calcul déterministe.

`info["score"]` est un `PovScore` (relatif au joueur au trait par
défaut). `.white()` le convertit en score absolu du point de vue des
Blancs — c'est la convention qu'on a actée. `.score(mate_score=3000)` gère
automatiquement la conversion des scores de mat (`Mate(n)` → un centipawn
capé à ±3000, décroissant légèrement avec la distance au mat) — pas besoin
de coder cette logique à la main.

**Question importante** : Stockfish 12+ utilise déjà son propre NNUE en
interne par défaut (l'eval "classique" historique a été retirée des
versions récentes). Les labels qu'on génère sont donc littéralement une
distillation du NNUE (assez gros) de Stockfish vers notre futur réseau
(bien plus petit) — c'est exactement l'objectif, mais c'est utile de le
savoir explicitement.

## Étape 6 — Paralléliser sur plusieurs cœurs

**Théorie** : chaque appel Stockfish est CPU-bound et domine le temps
total. Un `chess.engine.SimpleEngine` n'est pas partageable entre process
— il faut donc **un process Stockfish par worker**, ouvert une seule fois
via l'`initializer` du `multiprocessing.Pool` (pas un nouveau process
Stockfish par position, ce qui serait très lent) :

```python
_engine = None

def _init_worker(stockfish_path):
    global _engine
    _engine = chess.engine.SimpleEngine.popen_uci(stockfish_path)

def _eval_fen(args):
    fen, nodes = args
    board = chess.Board(fen)
    info = _engine.analyse(board, chess.engine.Limit(nodes=nodes))
    return fen, info["score"].white().score(mate_score=3000)

with mp.Pool(processes=8, initializer=_init_worker, initargs=(stockfish_path,)) as pool:
    for fen, eval_cp in pool.imap_unordered(_eval_fen, tasks, chunksize=16):
        ...
```

`imap_unordered` renvoie les résultats dès qu'ils sont prêts (pas dans
l'ordre d'entrée) — plus rapide que `imap`/`map` quand certaines positions
prennent plus de temps à analyser que d'autres.

## Étape 7 — Écrire le dataset

Format choisi : **CSV simple** (`fen, eval_cp, game_index`), écrit en
streaming ligne par ligne — pas de format binaire custom à ce stade. C'est
lisible à la main, facile à charger avec `pandas`, et l'encodage en
features (les 768 floats pour le réseau) se fera à la volée dans le
`Dataset` PyTorch en Phase 2, pas au moment du stockage — ça permet de
changer l'encodage plus tard sans regénérer tout le dataset.

`game_index` est gardé dans le CSV : indispensable pour l'Étape 8
(split sans fuite de données).

## Étape 8 — Split train/val/test sans fuite de données

**Théorie (point ML important)** : le split doit se faire au niveau des
**parties**, pas des positions individuelles. Si deux positions de la même
partie finissent l'une dans `train` et l'autre dans `val`, la validation
est biaisée — le modèle a déjà vu des positions très proches (même style
de jeu, structures de pions similaires) pendant l'entraînement. C'est une
fuite de données (*data leakage*) classique, facile à rater.

```python
game_ids = df["game_index"].unique().tolist()
rng.shuffle(game_ids)
n_train = int(len(game_ids) * 0.9)
train_ids = set(game_ids[:n_train])
# ... puis on filtre le dataframe par appartenance de game_index à train_ids
```

## Questions importantes / décisions à ajuster en pratique

1. **Volume total** : viser ~300k positions pour une v1 (largement
   suffisant pour un petit réseau). Le script peut être relancé sur
   d'autres mois de la Elite Database si besoin de scaler.
2. **Budget Stockfish par position** (`--nodes`) : 200 000 nœuds est un
   point de départ raisonnable (~profondeur 14-16 selon la position). À
   ajuster après avoir mesuré le débit réel (`pos/s` affiché par le
   script) sur ta machine — c'est le principal levier vitesse ↔ qualité
   du label.
3. **Positions tactiques** (capture immédiate en cours) : non filtrées en
   v1, raffinement possible plus tard (cf. Étape 3).
4. **Nombre de positions par partie** (`--positions-per-game`) : 3 par
   défaut — plus haut augmente le volume sans télécharger plus de PGN,
   mais réduit la diversité relative (plus de positions corrélées entre
   elles par partie).

## Synthèse des étapes

| Étape | Rôle | Fichier / fonction |
|---|---|---|
| 1 | Récupérer une base PGN de qualité | Lichess Elite Database (téléchargement manuel) |
| 2 | Lire les parties en streaming | `chess.pgn.read_game` dans `iter_sampled_fens` |
| 3 | Échantillonner des positions "quiet" | `sample_positions` |
| 4 | Dédupliquer | `normalized_fen` + `set()` dans `iter_sampled_fens` |
| 5 | Labelliser avec Stockfish (nodes fixes, mate capé) | `_eval_fen` |
| 6 | Paralléliser (1 process Stockfish/worker) | `_init_worker` + `mp.Pool` |
| 7 | Écrire le CSV en streaming | boucle `main()` dans `generate_dataset.py` |
| 8 | Split train/val/test par partie (sans fuite) | `split_dataset.py` |

## Code complet

### `ml/requirements.txt`

```
chess>=1.10
pandas>=2.0
```

### `ml/generate_dataset.py`

```python
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
```

### `ml/split_dataset.py`

```python
"""Split le dataset (Phase 1) en train/val/test SANS fuite de données.

Le split se fait au niveau des PARTIES (game_index), pas des positions :
plusieurs positions d'une même partie sont corrélées (même style de jeu,
même structure de pions...), donc les répartir entre train et val/test
romprait l'indépendance attendue entre les splits.

Usage:
    python split_dataset.py --in data/dataset.csv --out-dir data/splits/
"""

from __future__ import annotations

import argparse
import random
from pathlib import Path

import pandas as pd


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--in", dest="input", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--train", type=float, default=0.9)
    parser.add_argument("--val", type=float, default=0.05)
    parser.add_argument("--test", type=float, default=0.05)
    parser.add_argument("--seed", type=int, default=0)
    args = parser.parse_args()

    assert abs(args.train + args.val + args.test - 1.0) < 1e-6, "train+val+test doit faire 1.0"

    df = pd.read_csv(args.input)

    game_ids = df["game_index"].unique().tolist()
    rng = random.Random(args.seed)
    rng.shuffle(game_ids)

    n = len(game_ids)
    n_train = int(n * args.train)
    n_val = int(n * args.val)

    train_ids = set(game_ids[:n_train])
    val_ids = set(game_ids[n_train : n_train + n_val])
    test_ids = set(game_ids[n_train + n_val :])

    args.out_dir.mkdir(parents=True, exist_ok=True)
    splits = {"train": train_ids, "val": val_ids, "test": test_ids}
    for name, ids in splits.items():
        subset = df[df["game_index"].isin(ids)]
        subset.to_csv(args.out_dir / f"{name}.csv", index=False)
        print(f"{name}: {len(ids)} parties, {len(subset)} positions")

    print("\nDistribution des eval_cp (train):")
    print(df[df["game_index"].isin(train_ids)]["eval_cp"].describe())


if __name__ == "__main__":
    main()
```

### Utilisation

```bash
pip install -r ml/requirements.txt

# 1. Génération (depuis la racine du repo, ./stockfish déjà présent comme pour test-uci)
python ml/generate_dataset.py \
    --pgn lichess_elite_2024-01.pgn \
    --out ml/data/dataset.csv \
    --positions-per-game 3 \
    --nodes 200000

# 2. Split
python ml/split_dataset.py --in ml/data/dataset.csv --out-dir ml/data/splits/
```
