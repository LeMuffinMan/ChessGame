#!/usr/bin/env bash

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "usage: ${0##*/} <git-ref>" >&2
    exit 2
fi

cd "$(dirname "$0")/.."

ref=$1
sha=$(git rev-parse --short "$ref")
worktree="target/worktrees/$sha"
out="target/ref-bins/uci-ref"

git worktree prune
git worktree add --detach --force "$worktree" "$sha" >/dev/null
trap 'git worktree remove --force "$worktree"' EXIT

cargo build --release --bin uci --features=native \
    --manifest-path "$worktree/Cargo.toml" \
    --target-dir target/ref-builds

mkdir -p "$(dirname "$out")"
cp target/ref-builds/release/uci "$out"

echo "built $out from $ref ($sha)"
