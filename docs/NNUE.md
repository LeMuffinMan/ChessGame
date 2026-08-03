# NNUE — Eval par réseau de neurones (distillation Stockfish)

Roadmap du projet consistant à remplacer/compléter la fonction d'évaluation
heuristique du moteur (`src/engine/evaluator.rs`) par un petit réseau de
neurones entraîné par distillation depuis Stockfish (position → eval
Stockfish comme label). Objectif secondaire : apprendre le ML de bout en
bout (génération de données, entraînement supervisé, export, inférence).

Chaque phase est validée avant de démarrer la suivante — aucune phase
future n'est implémentée par anticipation.

## Contraintes actées

- **Déploiement statique inchangé** : le projet est buildé avec `trunk` et
  déployé sur GitHub Pages (`.github/workflows/deploy.yml`). Le réseau doit
  donc être **inférable en `wasm32` sans dépendance runtime lourde** — pas
  de PyTorch/ONNX-runtime dans le navigateur. Seul l'entraînement (offline,
  Python) a le droit d'être gourmand.
- **Entraînement** : Python + PyTorch, sur RTX 3050 Ti (4 Go — suffisant
  pour cette échelle de réseau). Cloud computing en option si besoin de
  scaler, dans la limite d'un budget raisonnable.
- **Génération de données Stockfish** : CPU-bound, faisable en local
  (i5/i7 disponibles), en tâche de fond si besoin.
- **Inférence** : Rust pur, **écrite à la main** (pas de `tract`/`burn`/ONNX
  côté Rust) — garantit la compatibilité `wasm32` sans risque de portage et
  reste dans l'esprit "apprendre le ML", pas "apprendre un framework".

## Ce qui existe déjà et sera réutilisé

- **FEN import/export** : `Board::board_from_fen` / `Board::to_fen`
  (`src/board/fen.rs`) — permet de charger/sauver des positions en FEN
  côté Rust. Pas bloquant côté Python non plus (`python-chess` a
  `board.fen()`).
- **Import/export PGN en Rust** (`src/gui/features/pgn/`) : feature de
  l'app (coller/charger une partie PGN pour la rejouer dans l'UI,
  symétrique à l'export existant), **sans lien avec la Phase 1**. Ce
  décodeur ne lit qu'une seule partie à la fois et n'est pas fait pour
  traiter une base PGN entière. Le dataset NNUE (Phase 1) reste **100%
  Python** (`python-chess`) : extraction depuis une base PGN, labelling
  (subprocess Stockfish UCI), dédoublonnage, split train/val/test — cohérent
  avec la règle du projet **ML → Python, reste → Rust**.
- **Point d'appel unique d'`evaluate()`** : `src/engine/minimax.rs:736`,
  dans `quiescence_minimax`, comme stand-pat score. La recherche délègue à
  la quiescence dès profondeur 0 → l'eval NN sera appelée à **chaque nœud
  feuille** : la vitesse d'inférence est un critère de conception dès le
  départ, pas un détail à optimiser après coup.
- **Zobrist hash** (`src/engine/zobrist.rs`) — réutilisable pour
  dédupliquer des positions si besoin côté Rust.
- **UCI binary** (`src/bin/uci.rs`), déjà utilisé avec cutechess-cli pour du
  test Elo (`justfile`: `test-uci`, `elo-uci`) — réutilisable tel quel pour
  mesurer la force du bot NN (Phase 5).
- **Feature flag `native`** (`Cargo.toml`) — pattern à suivre si un outil de
  génération de données doit vivre côté Rust natif (non exposé en wasm).

## Phases

### Phase 1 — Génération de données
Dataset de positions (FEN) + eval Stockfish (centipawns, avec gestion
spéciale des scores de mat).

- Partir d'une base PGN existante (Lichess elite database) plutôt que du
  self-play aléatoire, pour une distribution de positions réaliste — c'est
  l'approche standard pour ce type de distillation (utilisée par Stockfish
  NNUE lui-même et par des projets comme Maia Chess).
- Script Python (`python-chess`) : parse la base PGN, rejoue les coups,
  échantillonne des positions par partie, exporte en FEN.
- Script Python (Stockfish en subprocess UCI) : labellise chaque FEN,
  dédoublonne, split train/val/test.
- Le décodeur PGN Rust de l'app (`src/gui/features/pgn/decode_pgn.rs`)
  n'intervient pas ici : il est pensé pour importer une partie à la fois
  dans l'UI, pas pour traiter une base de plusieurs milliers de parties.

**Livrable validable** : dataset + script reproductible + stats
(distribution des évals, taille du dataset, temps de génération).

**Ressource** : [Lichess Elite Database](https://database.nikonoel.fr/)
(source PGN, parties 2400+ contre 2200+, filtrée par mois) ; page
[Training datasets](https://github.com/official-stockfish/nnue-pytorch/wiki/Training-datasets)
du wiki nnue-pytorch pour la façon dont l'équipe Stockfish organise/filtre
ses propres données.

### Phase 2 — Entraînement (PyTorch)
- Encodage board → features : v1 simple, 768 = 6 pièces × 2 couleurs × 64
  cases, du point de vue du joueur au trait. (Encodage king-relative type
  HalfKP en stretch goal si la v1 montre ses limites.)
- Petit réseau : feature transformer + 1-2 couches denses.
- Loss via squash sigmoïde (win probability) plutôt que MSE brut sur
  centipawns.
- Entraînement sur RTX 3050 Ti.

**Livrable validable** : modèle entraîné + courbes de loss + comparaison
qualitative sur positions connues (NN vs Stockfish vs eval heuristique
actuelle).

**Ressource** : [nnue-pytorch](https://github.com/official-stockfish/nnue-pytorch)
(trainer PyTorch officiel de Stockfish) — archi, encodage HalfKP/HalfKA,
loss sigmoïde ; [Chess Programming Wiki — NNUE](https://www.chessprogramming.org/NNUE)
pour la théorie (feature transformer, accumulateur incrémental).

### Phase 3 — Export des poids
Script d'export des poids/biais PyTorch vers un format binaire simple
consommable par Rust (f32, quantization int16 différée si besoin de
vitesse).

**Livrable validable** : fichier de poids + doc du format binaire.

**Ressource** : [`serialize.py`](https://github.com/official-stockfish/nnue-pytorch/blob/master/serialize.py)
de nnue-pytorch — référence concrète pour la quantization
(`round(poids_float * 127)`, int16) et le layout binaire d'un export NNUE.

### Phase 4 — Inférence Rust (wasm32)
- Nouveau module (ex. `src/engine/nnue.rs`), poids chargés via
  `include_bytes!` au compile-time, forward pass écrit à la main (même
  signature que `evaluate()`).
- Bascule **runtime** entre eval heuristique et eval NN (ex. nouvelle
  variante de `BotDifficulty` dans `src/engine/bot.rs`), pour comparer
  facilement les deux bots dans l'UI plutôt qu'un flag de compilation.
- Vérification explicite de la compilation `wasm32-unknown-unknown` et
  d'un build `trunk` local.

**Livrable validable** : bot jouable (natif + wasm local) utilisant l'eval
NN.

**Ressource** : [bullet](https://github.com/jw1912/bullet) (trainer NNUE
Rust le plus utilisé côté moteurs hobby) — sa doc et ses exemples de code
d'inférence sont une bonne référence pour écrire le forward pass à la
main, même si l'entraînement lui-même reste en Python ici.

### Phase 5 — Validation / mesure de force
Matchs engine vs engine via l'infra UCI existante (`src/bin/uci.rs`,
`justfile` `test-uci`/`elo-uci`, cutechess-cli) : bot NN vs bot heuristique
actuel, et vs Stockfish.

**Livrable validable** : résultats de matchs (score, Elo estimé).

**Ressource** : [Ordo](https://github.com/michiguel/Ordo) — calcule un
classement Elo à partir d'un PGN de résultats de matchs, complémentaire à
cutechess-cli déjà utilisé dans `justfile`.

### Phase 6 — Itérations (stretch, plus tard)
- Quantization int16 si besoin de vitesse.
- Encodage king-relative (HalfKP).
- Second projet initial : modèle de coups entraîné sur PGN, une fois le
  pipeline ML rodé.
- Scale du dataset (cloud GPU/CPU, éventuellement génération par self-play)
  si le petit NNUE hobby donne satisfaction et que l'envie/le temps sont
  là — **non engagé**, à réévaluer après la Phase 5 seulement.

**Ressource** : [Marlinflow](https://github.com/dsekercioglu/marlinflow)
(alternative à bullet, mêmes usages de référence) pour la quantization et
l'encodage king-relative ; le
[Testing guide](https://github.com/LeelaChessZero/lc0/wiki/Testing-guide)
de lc0 et l'infra [fishtest](https://tests.stockfishchess.org/tests) de
Stockfish si le stretch goal self-play est un jour repris sérieusement.
