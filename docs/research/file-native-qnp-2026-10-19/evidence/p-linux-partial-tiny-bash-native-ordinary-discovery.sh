set -euo pipefail
mkdir -p -- "$RESULT"
selector=$(cat -- "$STORE/SELECTED.json")
publication=$(printf '%s' "$selector" | /usr/bin/python3 -c 'import sys,json; print(json.load(sys.stdin)["publication"])')
control="$STORE/publications/$publication.json"
printf '%s\n' "$selector" > "$RESULT/selected-read-once.json"
printf '%s\n' "$control" > "$RESULT/expected-publication-path.txt"
if [ ! -f "$control" ]; then
 printf 'FAIL missing ordinary selected publication control: %s\n' "$control"
 printf 'No engine, custom pack decoder, generation scan, or alternative discovery was used.\n'
 exit 21
fi
attempt=$(/usr/bin/python3 -c 'import sys,json; print(json.load(open(sys.argv[1]))["attempt"])' "$control")
tree="$STORE/generations/$attempt"
printf '%s\n' "$tree" > "$RESULT/generation-path.txt"
find "$tree" -type f -print | sort > "$RESULT/listing.txt"
grep -R -n -F -- 'd000' "$tree" > "$RESULT/search.txt"
mkdir -- "$RESULT/exact-copy"
cp -a -- "$tree/." "$RESULT/exact-copy/"
/usr/bin/python3 -c 'import sys,pathlib,hashlib,json; a=pathlib.Path(sys.argv[1]); b=pathlib.Path(sys.argv[2]); h=lambda p:{str(x.relative_to(p)):hashlib.sha256(x.read_bytes()).hexdigest() for x in p.rglob("*") if x.is_file()}; assert h(a)==h(b); print(json.dumps({"files":len(h(a)),"exact_copy":True}))' "$tree" "$RESULT/exact-copy"
