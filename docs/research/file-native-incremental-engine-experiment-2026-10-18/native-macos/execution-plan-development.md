# Native Mac execution boundary

The whole experiment root is `/private/tmp/file-native-incremental-macos-20261018`.
Its original `allocation.json` freezes a two-hour overall ceiling, a 105-minute
execution cutoff and at least fifteen minutes for reporting. Setup, development,
the coordinator's instrumentation hold, replacement builds and the final run all
consume that same budget. New source/build/runtime/evidence files share one
6 GiB cap and a 20 GiB volume free-space floor. No prior prerequisite root is cleaned.

The initial exact common pin `fb73e75bb9893a4d897ecd28eb5615f6cc2a45d2`
is development-only after the coordinator requested an instrumentation amendment.
Its archive, 47 checked Git blobs, release binaries, successful workspace build,
two passed Rust tests, 117 uncensored native events, controlled hold traceback and
verified lossless export remain intact. No development 1k smoke was launched.

Final source will be archived from the next explicitly supplied exact common pin
to a different source root. It will receive a separate Cargo target, runtime,
evidence lane and wrapper source snapshot. No old event, test or binary will be
relabeled as final. Engine source edits on Mac are prohibited; a core failure
must be reported before any change.

The pinned `native.py` entry point runs through a transparent `runpy` adapter.
It tightens requested call caps to 180 seconds for ordinary cases and 600 seconds
for native 1k smoke, and binds each subdriver to the outer deadline and source.
The original capsule's checks/assertions and source archive stay unchanged.
The outer controller watches the complete owned root, not just each subdriver.
It reserves a 512 MiB margin within 6 GiB for bounded writes between checks.
Cooperative stop allows the pinned controller to preserve stdout/stderr before
refusal; a final watchdog may kill only its newly owned process group.

The final proof set is the requested workspace build/tests and native capsule:
four A/B-by-consumer named cases, 21 publication boundaries, the causal-review
supplement, one 1k A/B staged smoke, two separately compiled library consumers
and three Bash 3.2 ordinary copies. No larger tier, history, repeated matrix,
production migration, successor chat, PR or Pro consultation is allocated.

Closeout checks lossless stdout length/hash and JSON types for every recorded
event, stderr byte bindings, executable/session/source associations, portable
archive names and bytes, actual reached smoke outcomes, owned-root allocation,
and the exact staged result namespace. A completion marker alone is insufficient
because the B control intentionally permits a failed attempt to be recorded.
The result-only commit is parented to the final common pin and published only to
`origin/codex/file-native-incremental-macos-20261018` for coordinator retrieval.
