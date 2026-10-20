set -eu
store="$1"
out="$2"
mode="$3"
cp "$store/SELECTED.json" "$out/selected-once.json"
publication=$(sed -n 's/.*"publication": *"\([^"]*\)".*/\1/p' "$out/selected-once.json")
if [ "$mode" = B ]; then
 attempt=$(sed -n 's/^  "attempt": *"\([^"]*\)".*/\1/p' "$store/publications/$publication.json")
 tree="$store/generations/$attempt"
else
 tree="$store/view"
fi
printf '%s\n' "$tree" > "$out/captured-tree.txt"
cd "$tree"
find navigation -type f -name '*.yaml' | sort > "$out/listing.txt"
find navigation/groups -type f -name '*.yaml' | sort > "$out/groups.txt"
first=$(sed -n '1p' "$out/groups.txt")
grep -n 'source:' "$first" > "$out/search.txt"
source=$(sed -n '/^    source: /{s/^    source: "//;s/"$//;p;}' "$first" | sed -n '1p')
printf '%s\n' "$source" > "$out/chosen-source.txt"
cp "$source" "$out/copied-source"
shasum -a 256 "$out/copied-source" > "$out/copy-sha256.txt"
