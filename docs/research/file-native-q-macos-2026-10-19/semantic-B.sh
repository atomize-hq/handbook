set -euo pipefail
tree=/private/tmp/file-native-q-macos-20261019/shell-B/generations/attempt-11183-1792449439131709000-0
find "$tree" -type f
grep -n -E 'id:|title:|source:|kind:|target:|external:' "$tree/collection.yaml"
awk '/  - id: "d000001"/{f=1;next} f && /  - id:/{exit} f{print}' "$tree/collection.yaml" > /private/tmp/file-native-q-macos-20261019/edges-B.txt
