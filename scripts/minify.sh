#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
INPUT_RS="$ROOT_DIR/src/main.rs"
OUTPUT_RS="${1:-$ROOT_DIR/submit.rs}"
TOOL_SRC="$ROOT_DIR/scripts/minify_submit.rs"
TOOL_BIN="${TMPDIR:-/tmp}/awtf_minify_submit_$$"

cleanup() {
    rm -f "$TOOL_BIN"
}
trap cleanup EXIT HUP INT TERM

rustc "$TOOL_SRC" -O -o "$TOOL_BIN"
"$TOOL_BIN" "$INPUT_RS" "$OUTPUT_RS"
