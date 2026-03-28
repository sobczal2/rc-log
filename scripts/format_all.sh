#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

printf "\n[1/2] Formatting frontend with Prettier...\n"
(
  cd "$ROOT_DIR/frontend"
  npm run format
)

printf "\n[2/2] Formatting backend with cargo fmt (rustfmt config)...\n"
(
  cd "$ROOT_DIR/backend"
  cargo fmt --all --manifest-path Cargo.toml
)

printf "\nFormatting complete.\n"
