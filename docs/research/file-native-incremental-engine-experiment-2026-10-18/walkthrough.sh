#!/usr/bin/env bash
set -eu
root=$1
destination=$2
script_dir=$(cd "$(dirname "$0")" && pwd)
test ! -e "$destination"
cat "$root/README.md"
binding=()
while IFS= read -r line; do binding[${#binding[@]}]="$line"; done < <(python3 "$script_dir/ordinary.py" bind "$root")
test "${#binding[@]}" -eq 2
retained=${binding[0]}
manifest=${binding[1]}
find "$retained" -type f | head -25
cat "$retained/NAVIGATION.md"
find "$retained/docs" -name '*.json' -exec grep -H '1.00' {} \; | head -5
if test -n "${WALK_READY:-}"; then
  printf '%s\n' "$retained" > "$WALK_READY"
  count=0
  while test ! -e "$WALK_RELEASE"; do
    count=$((count+1)); test "$count" -lt 6000; sleep 0.01
  done
fi
mkdir -p "$(dirname "$destination")"
mkdir "$destination"
cp -R "$retained/." "$destination/"
python3 "$script_dir/ordinary.py" verify "$destination" "$manifest"
