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
INSTALL_BAT_SRC="$ROOT_DIR/scripts/install.bat"

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
[[ -f "$INSTALL_BAT_SRC" ]] || die "missing install.bat: $INSTALL_BAT_SRC"

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
need_cmd awk

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR/stage"

STAGE_DIR="$WORK_DIR/stage"
DATA_DIR="$STAGE_DIR/data"

mkdir -p "$DATA_DIR"

cp -f "$DLL_PATH" "$DATA_DIR/dfhooks_dfint_cjk_ko.dll"
cp -f "$SO_PATH" "$DATA_DIR/libdfhooks_dfint_cjk_ko.so"

echo "Downloading dfhooks chainloader (${DFHOOKS_VERSION})..."
curl -fsSL -o "$DATA_DIR/dfhooks.dll" "${DFHOOKS_BASE_URL}/dfhooks.dll"
curl -fsSL -o "$DATA_DIR/libdfhooks.so" "${DFHOOKS_BASE_URL}/libdfhooks.so"

echo "Downloading dfint-data (${DF_TRANSLATIONS_REPO}@${DF_TRANSLATIONS_BRANCH})..."
DF_TRANSLATIONS_ZIP_URL="${DF_TRANSLATIONS_REPO}/archive/refs/heads/${DF_TRANSLATIONS_BRANCH}.zip"
curl -fsSL -o "$WORK_DIR/df-translations.zip" "$DF_TRANSLATIONS_ZIP_URL"
unzip -q "$WORK_DIR/df-translations.zip" -d "$WORK_DIR"

SRC_DIR="$(find "$WORK_DIR" -maxdepth 1 -type d -name 'df-translations-*' -print -quit)"
[[ -n "$SRC_DIR" ]] || die "failed to find extracted df-translations directory"

mv "$SRC_DIR" "$DATA_DIR/dfint-data"
rm -rf "$DATA_DIR/dfint-data/.git" || true

echo "Writing install.bat..."
awk '{ sub(/\r$/, ""); printf "%s\r\n", $0 }' "$INSTALL_BAT_SRC" > "$STAGE_DIR/install.bat"

mkdir -p "$(dirname "$OUT_ZIP")"
rm -f "$OUT_ZIP"

(cd "$STAGE_DIR" && zip -q -r -9 "$OUT_ZIP" \
  install.bat \
  data)

echo "Wrote: $OUT_ZIP"

echo "Verifying zip contents..."
ZIP_ENTRIES="$(zipinfo -1 "$OUT_ZIP")"
grep -q '^install\.bat$' <<<"$ZIP_ENTRIES" || die "missing install.bat in zip"
grep -q '^data/dfhooks\.dll$' <<<"$ZIP_ENTRIES" || die "missing data/dfhooks.dll in zip"
grep -q '^data/dfhooks_dfint_cjk_ko\.dll$' <<<"$ZIP_ENTRIES" || die "missing data/dfhooks_dfint_cjk_ko.dll in zip"
grep -q '^data/libdfhooks\.so$' <<<"$ZIP_ENTRIES" || die "missing data/libdfhooks.so in zip"
grep -q '^data/libdfhooks_dfint_cjk_ko\.so$' <<<"$ZIP_ENTRIES" || die "missing data/libdfhooks_dfint_cjk_ko.so in zip"
grep -q '^data/dfint-data/' <<<"$ZIP_ENTRIES" || die "missing data/dfint-data/ in zip"

echo "OK"
