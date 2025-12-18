#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

DFHOOKS_VERSION="${DFHOOKS_VERSION:-v1}"
DFHOOKS_BASE_URL="${DFHOOKS_BASE_URL:-https://github.com/DFHack/dfhooks/releases/download/${DFHOOKS_VERSION}}"

DF_TRANSLATIONS_REPO="${DF_TRANSLATIONS_REPO:-https://github.com/refracta/df-translations}"
DF_TRANSLATIONS_BRANCH="${DF_TRANSLATIONS_BRANCH:-dfint-rust-cjk-ko}"

DLL_PATH=""
SO_PATH=""
OUT_ZIP="$ROOT_DIR/dist/dfint-rust-cjk-ko.zip"
WORK_DIR="$ROOT_DIR/dist/work"

usage() {
  cat <<'EOF'
Usage:
  package.sh --dll <dfhooks_dfint_cjk_ko.dll> --so <libdfhooks_dfint_cjk_ko.so> [--out <zip>] [--work <dir>]

Env:
  DFHOOKS_VERSION=v1
  DFHOOKS_BASE_URL=https://github.com/DFHack/dfhooks/releases/download/v1
  DF_TRANSLATIONS_REPO=https://github.com/refracta/df-translations
  DF_TRANSLATIONS_BRANCH=dfint-rust-cjk-ko
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "missing command: $1"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dll)
      DLL_PATH="${2:-}"
      shift
      ;;
    --so)
      SO_PATH="${2:-}"
      shift
      ;;
    --out)
      OUT_ZIP="${2:-}"
      shift
      ;;
    --work)
      WORK_DIR="${2:-}"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "unknown parameter: $1"
      ;;
  esac
  shift
done

[[ -n "$DLL_PATH" ]] || die "missing --dll"
[[ -n "$SO_PATH" ]] || die "missing --so"
[[ -f "$DLL_PATH" ]] || die "not a file: $DLL_PATH"
[[ -f "$SO_PATH" ]] || die "not a file: $SO_PATH"

if [[ "$OUT_ZIP" != /* ]]; then
  OUT_ZIP="$ROOT_DIR/$OUT_ZIP"
fi
if [[ "$WORK_DIR" != /* ]]; then
  WORK_DIR="$ROOT_DIR/$WORK_DIR"
fi

need_cmd curl
need_cmd unzip
need_cmd zip
need_cmd zipinfo

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/stage"

STAGE_DIR="$WORK_DIR/stage"

cp -f "$DLL_PATH" "$STAGE_DIR/dfhooks_dfint_cjk_ko.dll"
cp -f "$SO_PATH" "$STAGE_DIR/libdfhooks_dfint_cjk_ko.so"

echo "Downloading dfhooks chainloader (${DFHOOKS_VERSION})..."
curl -fsSL -o "$STAGE_DIR/dfhooks.dll" "${DFHOOKS_BASE_URL}/dfhooks.dll"
curl -fsSL -o "$STAGE_DIR/libdfhooks.so" "${DFHOOKS_BASE_URL}/libdfhooks.so"

echo "Downloading dfint-data (${DF_TRANSLATIONS_REPO}@${DF_TRANSLATIONS_BRANCH})..."
DF_TRANSLATIONS_ZIP_URL="${DF_TRANSLATIONS_REPO}/archive/refs/heads/${DF_TRANSLATIONS_BRANCH}.zip"
curl -fsSL -o "$WORK_DIR/df-translations.zip" "$DF_TRANSLATIONS_ZIP_URL"
unzip -q "$WORK_DIR/df-translations.zip" -d "$WORK_DIR"

SRC_DIR="$(find "$WORK_DIR" -maxdepth 1 -type d -name 'df-translations-*' -print -quit)"
[[ -n "$SRC_DIR" ]] || die "failed to find extracted df-translations directory"

mv "$SRC_DIR" "$STAGE_DIR/dfint-data"
rm -rf "$STAGE_DIR/dfint-data/.git" || true

mkdir -p "$(dirname "$OUT_ZIP")"
rm -f "$OUT_ZIP"

(cd "$STAGE_DIR" && zip -q -r -9 "$OUT_ZIP" \
  dfhooks.dll \
  dfhooks_dfint_cjk_ko.dll \
  libdfhooks.so \
  libdfhooks_dfint_cjk_ko.so \
  dfint-data)

echo "Wrote: $OUT_ZIP"

echo "Verifying zip contents..."
ZIP_ENTRIES="$(zipinfo -1 "$OUT_ZIP")"
grep -q '^dfhooks\.dll$' <<<"$ZIP_ENTRIES" || die "missing dfhooks.dll in zip"
grep -q '^dfhooks_dfint_cjk_ko\.dll$' <<<"$ZIP_ENTRIES" || die "missing dfhooks_dfint_cjk_ko.dll in zip"
grep -q '^libdfhooks\.so$' <<<"$ZIP_ENTRIES" || die "missing libdfhooks.so in zip"
grep -q '^libdfhooks_dfint_cjk_ko\.so$' <<<"$ZIP_ENTRIES" || die "missing libdfhooks_dfint_cjk_ko.so in zip"
grep -q '^dfint-data/' <<<"$ZIP_ENTRIES" || die "missing dfint-data/ in zip"

echo "OK"
