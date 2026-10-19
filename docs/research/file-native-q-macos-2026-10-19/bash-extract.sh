#!/bin/bash
set -euo pipefail
r=/private/tmp/file-native-q-macos-20261019
for mode in A B; do
 root="$r/capsule/restored-$mode-16/disposable/store"
 if [ "$mode" = A ]; then tree="$root/view"; else tree=$(python3 - "$root" <<'BIND'
import pathlib,json,sys
r=pathlib.Path(sys.argv[1]);s=json.loads((r/'SELECTED.json').read_bytes());p=json.loads((r/f"publications/{s['publication']}.json").read_bytes());print(r/'generations'/p['attempt'])
BIND
); fi
 find "$tree" -type f
 grep -n -E 'id:|source:|title:|kind:|target:|external:' "$tree/collection.yaml"
 # Extract one exact authored payload using ordinary shell copy/hash tools.
 source=$(awk '/  - id: "d000001"/{found=1;next} found && /    source: /{sub(/^    source: "/, "");sub(/"$/, "");print;exit}' "$tree/collection.yaml")
 test -n "$source"
 mkdir -p "$r/bash-copy-$mode"
 cp "$tree/$source" "$r/bash-copy-$mode/exact"
 cmp "$tree/$source" "$r/bash-copy-$mode/exact"
 shasum -a 256 "$tree/$source" "$r/bash-copy-$mode/exact"
done
