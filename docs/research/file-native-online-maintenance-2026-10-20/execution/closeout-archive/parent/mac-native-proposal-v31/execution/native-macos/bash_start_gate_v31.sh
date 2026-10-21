#!/bin/bash
# SOURCE-ONLY proposal: bind these exact bytes and /bin/bash before use.
# Before release only Bash builtins run; no SELECTED read or helper launch.
set -euo pipefail
IFS= read -r gate
[[ "$gate" == 'NATIVE-ROLE-GO-v31' ]]
unset gate
# Caller passes the reviewed script and fixed arguments; no command string.
exec /bin/bash "$@"
