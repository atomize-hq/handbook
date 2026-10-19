set -euo pipefail
tree=/private/tmp/file-native-q-macos-20261019/shell-A/view
find "$tree" -type f
grep -n -E 'id:|title:|source:|kind:|target:|external:' "$tree/collection.yaml"
awk '/  - id: "d000001"/{f=1;next} f && /  - id:/{exit} f{print}' "$tree/collection.yaml" > /private/tmp/file-native-q-macos-20261019/edges-A.txt
