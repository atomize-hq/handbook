# FNOP1 streaming harness — CONTROL ONLY

39-control module (old pin): execution/harness/stream_snapshot.py (34,662 bytes).
SHA256: ac4d34b899036a4fb23515ddf1ea364c91094fdacd9671b1a439b7f308f10aa8
Control source SHA256: c71ce0f5ea4afd530fd40dd21abc722578300f428ae9e9f6f89e95426359e79c

All 39 final synthetic controls passed. No engine, real store, long restore,
semantic qualification or accepted successor was run.

FNOP1 framing is unchanged. Independent pipe restore checks every path/size/hash,
raw/compressed SHA, gzip CRC/trailer/single-member EOF and exact FNOP1 EOF.
Full source before/after and independently rehashed destination inventories match.
Empty directories use a hash-bound sidecar; file blocks are64KiB, max file512MiB,
names4096 bytes and inventory2MiB, with bounds before append.

The final positive includes a multi-block ledger, empty directories and durable
physical-pins/x.lock. ALL SIX exact root exclusions were present and omitted:
writer.lock, preparation.lock, pack-readers.lock, pack-maintenance.lock,
physical-capture.lock, DISPOSABLE.json. All six also refused as restore frames.
Every other file, including durable .lock JSON, is included.

Controls cover truncation, CRC, hash/size/path errors, duplicates/case collisions,
trailing/concatenated gzip, Windows devices, same-size/same-mtime source changes,
preexistence, quotas, cutoff and failed verifier quarantine. Symlink/reparse,
disk and RSS use fault injection; native symlink creation was not qualified.

capture_restore(config, verifier=None) checks frozen module/pin/binary/oracle/
claim/budget/STORE/SELECTED bindings and admitted quiescent roots. Both helpers
close and byte verification finishes before a verifier callback. Parent owns
complete semantic/physical oracles and a destination-only accepted successor.
Real source cap651,179,227 leaves8MiB metadata+16MiB successor headroom.
No retained compressed capture is promised; tiny malformed control inputs remain.

Raw evidence: C:/file-native-online-matrix-20261020/receipts/stream-control-main/
Final controls-final.json SHA256:
cf21422784bc359c2c7714694e072ca46d81c902708fd41ca900fee621b97605
Final guard: exit0/no censor/errors, closure verified, peak RSS69,726,208B.
Five intervals exceeded40ms; maximum47ms. No strict cadence qualification.
Initial240s timeout and v2 evidence remain. Old runtime448KiB exceeded14,225B;
combined512KiB exceeded5,955B at worker census. Claim011 is not retroactive GO.
Final launcher checks actuals AFTER guard JSON/gzip close. Protocol1,508,133B/
164files; runtime542,242B/17files before this bounded Landing report.

GitNexus UNKNOWN/manual: new private module/controls only. No Rust/Cargo or other
21 Landing harness files edited. Parent alone stages/commits and reviews the
final pin before real long execution. No deletion/retry credit assumed.

P2 emitter-only revision: 34,729 bytes; current module SHA256:
f8d223618d869bf85a11ec648254830e643aa1abbb86374233ac9f31fe46cc20
Nine pure in-memory Unicode boundary cases passed: ASCII emission <=2001 bytes
including LF, with exit1 preserved. All40 function/class byte hashes and the
entire pre-main source are identical to the old39-control pin above.
Exact hashes and byte proof: runtime emitter-proof.json SHA256:
bbc984c969512adaaabdc3235357401a20a0c3366b5355a795bb6aa083c08763
The39 controls stay bound to ac4d; no full rerun is claimed. Focused guard:
exit0/no censor/errors, closure verified, sampled peak24,137,728B.
No protocol growth. Current runtime559,003B/23files; focused additions16,761B/
6files are within32KiB/7. The historical overrun and cadence limits remain.
