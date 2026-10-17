# Gate 0 — bounded execution sheet

Execution amendment at 16:01 UTC, within the original caps: first-run stores at
`C:/file-native-experiment-20261017` are preserved and superseded after causal
review found a deep attachment navigation defect. Corrected runs use
`C:/file-native-experiment-20261017-v2`; both directories count toward the same
64 GiB cap. Build target is `C:/file-native-experiment-20261017-build` (also counted).
Use Windows FileStandardInfo.AllocationSize for actual allocated file bytes;
the first-run GetCompressedFileSizeW observations are not cluster allocation.
The original 16:48 workload launch cutoff / 17:00 end cap is unchanged. A saved
packet now includes the local relationship/attachment closure of its first eight
ordered assembly inputs, with relative navigation checked. At 10k+ this includes
the 5,000-constituent overview dependency closure; count those copied bytes.

At 16:03 UTC a real concurrent Windows original-install refusal interrupted the
second run. Preserve that failed trial and all v2 stores; corrected runs use
`C:/file-native-experiment-20261017-v3`. All three roots plus build files share the
original cap. Shared-directory creation now tolerates a concurrent existing
directory; Windows original installation uses no-replace MoveFileExW, verifies an
identical-content winner, and bounds transient refusal retries to five attempts
5 ms apart. This does not retry application publication or arbitrary agent work.
The unexecuted Unix path may replace an identically addressed original in a race;
no inode-immutability or native Linux qualification is inferred from Windows.
The serial 64-document publication also captures one 256 KiB attachment in that
same batch (exact base, no extra publication), in addition to the initial size mix.

At 16:20 UTC the fixed 15-second lock timeout produced actual `busy` refusals
in the 10k admission wave. The initial Python driver asserted only conflict;
preserve that stopped driver record. Resumption now reads positive selected
history, skips already accepted request IDs and explicitly resubmits missing pure
fixture commands. No mutation is replayed to discover an outcome. Finish paired
10k project admission cases, then prioritize paired 100k project admission cases
before other remaining configurations, within the unchanged caps. Unreached
consumer/policy allocations remain unmet. `cap-guard.ps1` monitors this exact
driver family and stops remaining native work at 16:48 UTC or the free-space
reserve; it changes no stored authority or retained data.

Recorded before fixture generation, 2026-10-17 15:40 UTC (11:40 EDT).
Controlling brief: `../dolt-gitbutler-file-engine-2026-10-16/experiment.md`,
updated October 17. Both A and B remain open. This is standalone experimental
code, with no production dependencies or owner migrations.

## Bounds and platforms

- Source: this directory; generated stores/builds: `C:/file-native-experiment-20261017`.
  Refuse an existing run directory. Preserve every generated selected snapshot,
  original, packet, abandoned candidate and raw observation for this run.
- Hard elapsed execution cap: 80 minutes from 15:40 UTC, including implementation,
  runs and review. Stop workload launch by 16:48 UTC to reserve reporting time.
- Hard generated-data allocation: 64 GiB, with 160 GiB free-space reserve. C: NTFS
  inventory before generation: 259,793,305,600 free bytes. Reserve per-case upper
  bounds before launch and check actual size/free space between cases; no rolling
  retained-generation deletion. Maximum 8 outstanding child processes, 120 seconds
  per small trial, 600 seconds per growth publication, 15 seconds lock wait,
  no automatic retry of arbitrary requests. Build allowance 2 GiB inside the cap.
- Native local: Windows x86_64/NTFS, Rust 1.89.0 MSVC, Cargo 1.89.0, Python 3.11.9.
  Existing SSH host `spenser-linux-codex` was probed, but configured key load fails
  with invalid format followed by authentication refusal. No accessible native
  Linux/ext4 runner established. WSL distributions exist; they cannot substitute.
  Native Linux is an unmet gate, not an inferred pass. macOS/APFS/Finder follow-on.
- Desktop Explorer process exists. Computer-use capability will be tested after
  fixtures exist; API paths are not evidence of shell usability. Linux ordinary
  browser cannot be exercised without a native host/session.
- GitNexus index: c7f54e63, one commit behind b6b3aed9. Query is degraded (schema
  mismatch); status/analyze fail in analyzer identity hashing. No existing source
  symbols will be edited or reused. Unknown new symbols are not impact coverage.
  Run impact before repairs and detect_changes before committing; report limits.
- Gate 3 only after valid Gate 1 evidence and within remaining caps. Optional
  DoltLite 0.51.0 Windows integration effort cap 8 minutes, at most one external
  adapter. Do not start it if required correctness evidence remains unmet.

## Authority and acyclic identities

Requests, attempts, generations and publications have separate identities.
Attempt directories use PID, nanosecond timestamp and monotonic sequence, then
exclusive create; interrupted same-process retries cannot reuse staging paths.
One `SELECTED.json` record is the application authority. It carries publication
ID and SHA-256 of an immutable publication record. That record carries the exact
predecessor selector, candidate state digest/path, B closure manifest digest/path
or A refresh target, original request fingerprint and original result. Digest
dependencies are selector → publication → candidate manifest/state → payloads;
neither manifest nor state embeds its own hash or publication hash.
Only selected predecessor links establish positive history; directory ordering,
seals, abandoned candidates and surviving payloads do not establish membership.
History corruption is an integrity error, including when searching a missing
request. No retention/GC or rollback-freshness certification is claimed.

Intent fingerprints cover captured request input (base and ordered operations,
including exact import bytes), independently of attempt/generation identities.
Under the per-store OS lock, check selected lineage for dedup before comparing
base publication. Same request/intent resolves the original outcome; different
intent refuses. Selected results remain resolvable beneath later edits/deletes.
No receipt is authoritative. Absence is `absent-as-of`, never terminal noncommit
while a writer may publish. Cancellation uses the same publication lock and a
durably installed fence checked at selection; fence takes precedence over new
attempts, but an already-selected outcome resolves first.

## Publication boundary and recovery

Prepare immutable originals and complete candidate metadata; B additionally
copies/validates every required readable byte into a unique generation and seals
its exact namespace. Sync files and nested names bottom-up before selection.
Under the native per-store process lock: resolve predecessor synchronization,
dedup/fence/base checks, install/sync immutable publication, stage/sync selector,
replace selector, sync containing name, then acknowledge original result.
Linux implementation uses rename and fsync each affected directory; it remains
unexecuted until a native runner is available. Windows uses MoveFileExW with
WRITE_THROUGH, file sync, and separately records unsupported directory flushing.
Windows results qualify process interruption with functioning NTFS, not complete
nested-name persistence after OS crash. An exposed post-replace/pre-boundary
selector is uncertain; a successor re-establishes the predecessor boundary
before acknowledging. Process death after boundary must recover original result.

A publishes immutable state before refresh. Its view may be stale/mixed; status
records target and completed paths. Refresh uses whole-file replacement, can be
resumed from selected exact state, and never replays mutation. Resume under the
store lock avoids an old renderer overwriting a successor. B selects only a fully
installed/verified unique generation. Reader selects a retained path once; it
must never follow a current alias or mutable originals during navigation.

## Encoding, closure and validation

One candidate representation is a bounded JSON state manifest with ordered stable
IDs, exact blob hashes, portable meaningful paths, full Unicode titles, typed
edges, uniqueness labels and budget values. Machine manifests use serde JSON;
authored YAML/JSON/Markdown payload bytes stay exact (including CRLF). This is an
experimental contract, not adoption of proposed repository-wide format defaults.
No YAML parser or JSON reserialization of authored source is needed. Requests are
typed closed JSON; duplicate fields are refused by typed decoding.

Portable paths use a safe ASCII title slug plus stable ID and extension, with
bounded depth/length. Reserved/colliding/normalization-sensitive full titles are
retained separately. Generated navigation contains exact relative links for
local relationships, attachments and packets; external misses are explicit.
B closure includes all documents, navigation, packet bytes, ordered provenance,
recipe/renderer version, omissions and source hashes. A retains the same packet
and its input bytes in immutable candidate storage, separately from mutable view.
Destination verification rereads/hashes installed bytes and checks exact names;
it is not merely a manifest construction. Closure copies use a 64 KiB userspace
read/write loop into newly created regular files, never std::fs::copy/reflink.

Validate whole state: stable-ID/path/label uniqueness, required local edge target
existence, edge kind and aggregate budget. Fixture commands are pure; retries
require explicit new base and full semantic validation. No general merge/OCC or
external-effect replay. Two consumer stores have independent locks. Cross-store
packet roots are an explicit vector plus retained copies, never a global commit.

## Allocated workload (each consumer and each policy, not a Cartesian product)

Corpus seed 20261017. Each 10 documents: seven 4096-byte, two 16384-byte and one
65536-byte sources, alternating LF/CRLF, plus special exact JSON and bounded
larger attachments. Project and research use separate instances/vocabulary.
At 10k+, include a 5,000-constituent relationship set and high-degree overview.

| Tier | Initial + subsequent B publications | Allocation of subsequent publications |
| --- | --- | --- |
| 1k | 1 + 19 = 20 | 3 serial batches (1,8,64); contention waves 2,4,8 processes (14 accepted admission slots or explicitly rebased pure speculative commands); 2 slow-versus-small requests |
| 10k | 1 + 9 = 10 | 3 serial batches (1,8,64); 2-process then 4-process contention (6 accepted publications) |
| 100k | 1 + 2 = 3 | one 1-document and one 64-document publication; sparse observations individually |

A runs the identical mutation/publication sequence with incremental refresh.
Policies: speculative final preparation vs one admission slot around final
preparation/validation/materialization/selection. Thinking/capture is outside.
Conflicting complete speculative candidates remain inventoried, then a new
explicitly rebased fixture command may be submitted. Locks do not imply FIFO.
About 30 deterministic repetitions per small-race family are separate from the
growth allocation. Gate 1 must pass locally before growth; absent platforms/UI,
unreached faults/workloads stay listed. Gate 3 is conditional, not mandatory
adapter development despite failed earlier gates.

## Measurement

Raw JSONL per invocation: stage durations, bytes/helper counts, sync attempts,
lock wait/hold, request outcome, pending/discarded candidate sizes, process peak
memory where available. Separate external wall time from instrumented stages;
overlapping sync/file-stage times are explicitly inclusive. Fresh-process lookup
is not cold device. p50/p95 only for >=20 samples (nearest-rank); two 100k changes
are not a distribution. Inventory logical and allocated storage, files/directories,
originals/generations/state/publications/staging/copies. Record actual compiler,
helper implementation, NTFS compression and dedup status if accessible.
Fault injection: named process exit/reported error/external kill barriers around
write/sync/install, file/directory creation, copy/verification/seal, expected-base
and dedup, selector write/replace/name sync, receipt/reply, A refresh. Actual counts
and checkpoint-specific oracle are evidence; process-only tests never simulate
power removal. Do not call an incomplete gate full completion in a commit/report.
