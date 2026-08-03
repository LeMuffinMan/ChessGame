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
