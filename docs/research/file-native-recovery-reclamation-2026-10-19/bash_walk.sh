#!/usr/bin/env bash
set -eu
base=$1
output=$2
mkdir "$output"
for mode in A B; do
  for count in 16 128; do
    root="$base/restored-$mode-$count/disposable/store"
    tree=$(python3 - "$root" <<'PY'
import hashlib,json,sys
from pathlib import Path
r=Path(sys.argv[1]);s=json.loads((r/'SELECTED.json').read_bytes());b=(r/f"publications/{s['publication']}.json").read_bytes()
assert hashlib.sha256(b).hexdigest()==s['digest'];p=json.loads(b);config=json.loads((r/'STORE.json').read_bytes())
print(r/('view' if config['mode']=='A' else 'generations/'+p['attempt']))
PY
)
    ls "$tree"
    grep -n 'relationships:' "$tree/collection.yaml" | head -3
    relative=$(awk '/  - id: "d000009"/ {getline;getline;getline;sub(/^    source: "/,"");sub(/"$/ ,"");print;exit}' "$tree/collection.yaml")
    test -n "$relative"
    cp "$tree/$relative" "$output/$mode-$count.txt"
    first=$(sha256sum "$tree/$relative" | cut -d' ' -f1)
    second=$(sha256sum "$output/$mode-$count.txt" | cut -d' ' -f1)
    test "$first" = "$second"
    printf '%s %s %s %s\n' "$mode" "$count" "$relative" "$first"
  done
done
