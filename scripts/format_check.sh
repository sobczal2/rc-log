#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

printf "\n[1/2] Checking frontend formatting with Prettier...\n"
(
  cd "$ROOT_DIR/frontend"
  npm run format:check
)

printf "\n[2/2] Checking backend formatting with cargo fmt --check...\n"
(
  cd "$ROOT_DIR/backend"
  cargo fmt --all --manifest-path Cargo.toml -- --check
)

printf "\nFormatting check passed.\n"
