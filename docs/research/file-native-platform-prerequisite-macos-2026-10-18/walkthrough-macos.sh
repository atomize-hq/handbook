#!/usr/bin/env bash
set -euo pipefail
store=$1
destination=$2
ls -la "$store"
cat "$store/README.md"
# Standard-library JSON parsing only; this does not import or invoke the engine.
binding=()
while IFS= read -r binding_line; do binding+=("$binding_line"); done < <(python3 - "$store" <<'PY'
import json,pathlib,sys,hashlib
r=pathlib.Path(sys.argv[1]);s=json.loads((r/'SELECTED.json').read_bytes());b=(r/'publications'/(s['publication']+'.json')).read_bytes();assert hashlib.sha256(b).hexdigest()==s['digest'];p=json.loads(b);packet=r/p['candidate']['closure'];print(packet);print(p['candidate']['manifest_digest']);print(r/'view' if p['candidate']['mode']=='A' else packet)
PY
)
packet=${binding[0]}
expected=${binding[1]}
readable=${binding[2]}
cat "$readable/NAVIGATION.md"
find "$readable/docs" -type f
find "$readable/docs" -maxdepth 1 -name '*d000000*' -exec head -n 6 '{}' \;
find "$readable/docs" -name '*.md' -exec head -n 4 '{}' \;
find "$readable/docs" -name '*.txt' -exec head -n 4 '{}' \;
test ! -e "$destination"
cp -r "$packet" "$destination"
python3 - "$destination" "$expected" <<'PY'
import json,pathlib,hashlib,sys
r=pathlib.Path(sys.argv[1]);data=(r/'MANIFEST.json').read_bytes();assert hashlib.sha256(data).hexdigest()==sys.argv[2];m=json.loads(data);assert all(hashlib.sha256((r/n).read_bytes()).hexdigest()==h for n,h in m['files'].items());print('EXTRACTION VERIFIED',len(m['files']),'files',r)
PY
cat "$destination/packet/provenance.json"
