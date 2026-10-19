#!/usr/bin/env bash
# Engine-free discovery and extraction on an already captured A/B path.
set -euo pipefail
tree=$1
dest=$2
mkdir "$dest"
find "$tree" -type f -print | LC_ALL=C sort > "$dest/listing.txt"
rg -n 'id:|title:|kind:|source:|target:|external:|relationships_' "$tree/navigation" "$tree/navigation.yaml" > "$dest/discovery.txt"
source=$(sed -n 's/^    source: "\([^"]*\)"$/\1/p' "$tree"/navigation/groups/*.yaml | awk 'NR==1 {print}')
test -n "$source"
printf '%s\n' "$source" > "$dest/extracted-path.txt"
cp "$tree/$source" "$dest/extracted-source"
cp -R "$tree" "$dest/tree"
sha256sum "$dest/extracted-source" > "$dest/extracted-source.sha256"
