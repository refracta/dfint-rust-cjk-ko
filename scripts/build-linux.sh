#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

OUT_DIR="${OUT_DIR:-"$ROOT_DIR/dist"}"
OUT_FILE="${OUT_FILE:-"$OUT_DIR/libdfhooks_dfint_cjk_ko.so"}"

mkdir -p "$OUT_DIR"

(
  cd "$ROOT_DIR"
  cargo build --release
)

cp -f "$ROOT_DIR/target/release/libdfint_hook.so" "$OUT_FILE"
echo "Wrote: $OUT_FILE"
