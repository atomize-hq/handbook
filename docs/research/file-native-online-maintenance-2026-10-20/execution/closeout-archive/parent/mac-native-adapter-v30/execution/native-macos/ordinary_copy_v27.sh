#!/bin/bash
set -euo pipefail
PYTHON="$1"; HELPER="$2"; STORE="$3"; OUT="$4"; CAPTURED="$5"; SAVE="$6"
tree="$("$PYTHON" -B "$HELPER/ordinary_route_v27.py" "$STORE" "$OUT" "$CAPTURED" "$SAVE")"
/bin/cp -R "$tree"/. "$OUT"/
"$PYTHON" -B "$HELPER/ordinary_manifest_v27.py" "$OUT"
