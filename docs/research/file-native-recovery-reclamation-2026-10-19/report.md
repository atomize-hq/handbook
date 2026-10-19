# File-native recovery and initial reclamation results

October 19, 2026. Standalone experimental engine research. The Windows
preservation/restore/reclamation trial succeeded. Platform coverage is qualified
below; this is not production qualification or a Handbook migration.

## Result

The original H101003 store remains at its original path and selector. It was
read for provenance, verification and capture, and was never renamed, repaired,
published into or collected. Collection used one independently restored clone.

The clone passed full operational restoration, fresh reopening/recovery, a valid
successor, collection without retirement, an interrupted collection and restart,
a prospectively bounded 100-publication retirement cohort, and a distinct-source
publication followed by restoration of the prior bytes. The distinct source was
actually removed while its original accepted result remained exact.

All 101,003 original accepted publications/results and membership/outcome
coverage survived. Final history has 101,006 publications, including the three
new operational/reclamation publications. The 101 intentionally retired content
snapshots are unavailable by contract; they are not counted as retained-content
passes. The final current graph equals the untouched original graph, and exact
retained source/index obligations passed independent verification.

| Clone state | Regular files | Logical file bytes |
| --- | ---: | ---: |
| After operational reopening/successor, before GC | 1,413,495 | 5,465,011,991 |
| After collection without retirement | 510,713 | 888,599,719 |
| After retirement pilot and distinct-source collection | 510,583 | 887,923,682 |

Thus the first stage reduced the observed logical footprint by about 4.58 GB
(83.7%). Windows cluster allocation was **not measured**. File lengths, unlink
counts and physical allocation are different quantities.

## Source, build and measurement identities

| Purpose | Commit |
| --- | --- |
| Pinned prior result and source baseline | `3396092cbf20f593e36b9640c6a76e5afd2f2a09` |
| Prior Rust/Cargo core, verified unchanged at that result | `5a65a2155f9eb297d2cbb1b7c900fa749629605c` |
| Initial corrected core and first complete native lanes | `40509a90a015de84bd12d26e152a52c92721baa0` |
| Operational transport used for H101003 capture/restore | `3d3f7b47264029dd1a8a3a1298cb1c740c0e7ae9` |
| Recreated-directory preservation correction | `ec544776255c6eafb06700c40c04b1e27fdf8898` |
| Core used for operational reopening and actual long GC | `fe1ee22a82d65dc88a14e38b7e77c921538c4bc4` |
| Final core, adding the external-empty-directory guard | `07a8574e581e66ba58d7caac87238aa5f93918be` |

The last core change restricts A repair pruning and corrects a packet comment.
The GC/accounting implementation is identical to the measured `fe1ee22a` core.
The final Windows capsule, probes and matched small costs use `07a8574e`.
Later report/harness commits do not change the Rust/Cargo core.

Windows long-trial executable SHA-256:
`4d8ecb0ff2c29b42bf83161803336fadfc6dd9cfbd2ae6594c05d9358adbac0b`.
Final Windows executable SHA-256:
`cbd9b0352c52998ae7e6c9a2cd160bc3cbc16ea3d1b40c339eb570450d765cca`.
Complete source/build hashes are in
[the measurement binding](evidence/final-build-binding.json) and
[the final binding](evidence/final-code-build-binding.json).

Original selector, verified before capture and after collection:

```json
{"publication":"publication-74796-1792371048916760700-13","digest":"9b80c0b54b15af6457ee50290f266684bed6c06223cbe105a2654da7bb368546"}
```

The source baseline was also confirmed against the pushed origin ref. The shared
Handbook checkout remains on `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`;
this work is isolated on `codex/file-native-recovery-20261019`.

## Correctness and compatibility

A refresh retains the original sealed plan representation, but executes removals
deepest first and installations shallowest first. It removes obsolete managed
files and empty managed parents, preserves divergent bytes and obstructing
subtrees, and completes a pending plan before advancing to a successor. A user
recreating an obstructing directory after an exit produces a second preserved
incarnation. Untracked external parent directories are retained during repair.
Valid ancestor/descendant transitions work in both directions; impossible final
file namespaces, including case-folded collisions, refuse before selection.

New ordinary preparations target projection profile 2, including on a legacy
store. Exact authored files and existing packet/provenance files remain, with
one supplementary `collection.yaml` for missing graph metadata: membership,
stable IDs, current titles/kinds/paths, and ordered directed typed relationships.
There are no new Markdown navigation/index files and no compulsory document
guide. This is not a one-for-one Markdown-to-YAML conversion. The metadata is
needed because the fixture graph stores these facts independently of source
bytes. A complete logical export is a different interface.

Historical profile-1 objects remain byte-for-byte unchanged. The immutable
projection identifies its reader profile; already-sealed legacy preparations
replay under their original profile. New targets use profile 2. Checkpoints carry
their selected snapshot unchanged. New standalone packets use their existing
source/provenance information without an extra guide. Windows tests also reopen
legacy packet preparations, preserving their original retained manifests.
The independent legacy verifier now checks actual root destinations; the new
oracle checks the complete backing-file hashes and B manifest binding. Legacy
stored navigation bytes, including historical defects, are not rewritten.

The new oracle checks graph discovery/membership, source locations, relationship
direction/type/order/multiplicity, exact extraction, backing-file integrity and
selected-once B reading. It distinguishes immutable correctness from an A tree
that is lagging or interrupted. It does not mistake an A completion marker for
an external-edit integrity guarantee. Ordinary PowerShell and Bash listing,
search, copy and SHA-256 extraction passed for A/B N16/N128 without engine calls.

Checkpoint and collection now reconcile selected allocation transfers before
candidate evidence can be removed. Transfer still requires the original accepted
attempt/intent and permanent allocation-owner binding. The original
accounting-transfer/checkpoint/retire/two-collections sequence passes in A and B,
with the accepted result unchanged. This repairs the recoverable accounting
inconsistency; it does not establish that the old observation lost accepted data.

The final Windows capsule passed 24 cases / 282 process events. Additional final
checks passed for same-ID/different-intent handling, replay dependencies across
GC, packet ownership, strict missing/corrupt evidence refusal, independent
whitespace-corruption refusal, and external empty-directory preservation.
Small restored A cases now contain a genuinely interrupted path transition with
different target/prior watermarks; their pending/completed obligations survive
byte-for-byte before recovery. Restored fences and intent bindings were also
explicitly exercised. These event counts are not independent statistical trials.

## Full operational preservation gate

The quiescent cooperating contract excludes active mutators during capture.
Durable eligible and half-prepared work is permitted. Every regular file is
captured, including fences, pins, reservations, candidates, packets, A obligations,
B generations, sources and indexes. OS lock files and the root-bound disposable
permission are excluded. No portable deletion authority is imported.

The transport has a complete per-file inventory/hash, an inventory hash, and a
transport hash. A partial capture has no completed manifest. Restore requires a
new destination, checks every framed record and digest, and explicitly reports
that operational reopening is still required. Missing/corrupt required evidence
cannot establish an operational restore-success claim.

Small A/B N16/N128 tests cover pins, terminal fences, packet ownership, eligible
selection after restore, half-preparation refusal/fencing/cleanup, pending and
completed A state, B generation integrity and a subsequent valid publication.
The old original-renaming GC continuation now refuses; its proof-only caller no
longer launches collection.

| H101003 preservation stage | Observation |
| --- | --- |
| Original census | 1,413,491 files / 5,464,941,402 logical bytes |
| Operational capture | 1,413,489 files / same logical bytes; two zero-byte OS locks excluded |
| Compressed transport | 2,466,253,310 bytes |
| Per-file inventory | 248,280,348 bytes |
| Capture | 546.825 s controller; 75,075,584-byte sampled peak RSS |
| Restore to new root | 328.626 s; 23,355,392-byte sampled peak RSS |
| Independent restored verification | 518.212 s controller; all original history and retained obligations passed |
| Fresh operational reopening | recovery, audit, lost-reply resolution, valid successor: success |

The original independent check had two superseded development attempts stopped
within its original 900-second deadline. Removing redundant reads and using four
bounded result-read workers retained every hash/body/index check. The completed
attempt took 278.532 s with a warmed prefix; total original verification elapsed
about 729 s. These are not isolated performance ratios.

Capture uses at most 16 workers / 64 pending small files; large files stream.
It visits each captured regular file once rather than following repeated logical
historical roots. This is full operational preservation, not a relabeled success
for the old censored `pack_history.py` proof. The old proof censor remains.

The full transport and inventory remain at
`C:/file-native-recovery-20261019/history-operational-backup`.
Their manifest and exact hashes are committed in
[operational-backup-manifest.json](evidence/operational-backup-manifest.json).

## Actual collection

The only collection root was
`C:/file-native-recovery-20261019/history-restored/disposable/store`.
Its literal resolved path was checked against the newly owned destination and
the restoration gate. Disposable permission was established anew there.

| Pass | Result | Time | Peak engine memory |
| --- | --- | ---: | ---: |
| Controlled interruption | Exit 86 after first actual reclaimed file | 29.748 s | 347,901,952 B |
| Restart, no content retirement | Completed; 902,782 further files / 4,576,395,870 B removed | 243.345 s | 347,893,760 B |
| 100-publication pilot + distinct snapshot | Completed; 270 files / 819,541 B removed | 13.565 s | 186,388,480 B |

The interrupted prefix removed one 16,481-byte object. The whole first stage
therefore removed 902,783 object files / 4,576,412,351 bytes. The before/after
table also includes the new 79-byte disposable marker. No content was retired
before that completed pass.

GC retains all permanent selected publication/result evidence and current
membership/outcome/change roots, with typed protections for readers, pins,
reservations, packets and A obligations. Checked reuse is only for identical
`(graph tree ID, projection tree ID)` pairs; each unique traversal keeps the
existing hash/version/parent-bound checks. The first pass reduced 101,005 root
visits to 129 unique checked pairs; the second used 89 pairs from 100,906 visits.
No unoptimized long-GC timing exists, so this is not a causal speedup ratio.

The first completed pass recorded 8,322 page reads, 109,339 instrumented opens,
902,782 namespace removals, 215,514,692 bytes read and no engine bytes written.
The second recorded 8,003 page reads, 109,022 opens and 270 removals.
Windows recorded unsupported directory-sync operations for these deletions.
These counters are instrumented API work, not a complete syscall trace.

The cohort was fixed at the first 100 ledger publications before retirement;
the prospective pilot limit was 100, not 1,000. Two subsequent source publications
introduced distinct bytes and restored the prior bytes. The distinct publication
was retired, its unreferenced source disappeared, and its original accepted result
remained exact. New publication/retirement bookkeeping is included in net counts.

### Residual roles

Cells are **files / logical bytes**. The final column is a net state comparison,
including two new publications and retirement markers; the pass counters above
are gross removals. Per-role gross deletions for the second pass were not traced.

| Role | Before first GC | After no-retirement GC | Final |
| --- | ---: | ---: | ---: |
| Objects | 908,406 / 4,638,738,285 | 5,623 / 62,325,934 | 5,584 / 62,012,279 |
| Attempts | 202,008 / 403,012,724 | same | 201,810 / 402,632,280 |
| Publications | 101,004 / 135,593,838 | same | 101,006 / 135,596,518 |
| Results | 101,004 / 25,919,709 | same | 101,006 / 25,920,251 |
| Attempt hints | 101,004 / 14,862,069 | same | 101,006 / 14,862,355 |
| Originals | 17 / 155,648 | same | same |
| Derived files | 20 / 5,610 | same | same |
| Current readable view | 19 / 154,329 | same | same |
| Fences | 3 / 60 | same | same |
| Retirement markers | 0 / 0 | 0 / 0 | 101 / 14,554 |
| Root files | 10 / 246,569,719 | 11 / 246,569,798 | same |

The test ledger alone is 246,560,316 bytes, separate from engine retention
requirements. There are still 303,018 publication/result/hint files and 201,810
content-lifetime attempt files. These measured residuals inform a later human
decision; live packing/compression and retention-horizon changes were not made.

## Post-GC preservation proof

The independent final walker compared **all** current membership/outcome entries
against the complete selected predecessor chain and original result hashes. It
also checked every ledger result, all retained graph/index/source obligations,
retirement identities and current control roots. It verified the original
101,003-publication suffix digest after the three new publications. Final
verification took 30.561 s controller / 502,173,696-byte sampled peak RSS;
its cache state was warm after collection.

Fresh-process probes compared with the untouched original for initial, early,
middle, recent and lost-reply requests. All original results were exact. Positive
point reads and source bytes passed for retained snapshots; `history-000001`
correctly returned `retired:snapshot`. All three original fences remained exact
and refused new publications. There were 14 successful probes and four expected
refusals. The distinct retired source's absence and accepted result were checked
separately in the trial.

[The final verification](evidence/post-gc-operational-verification-result.json),
[original/control preservation check](evidence/original-preservation-and-final-state.json)
and raw bundles retain the complete commands/responses. Original selector,
STORE, history-source metadata and ledger hashes still match capture.

## Small cost controls and platform coverage

Matched release builds used N16/N128, three edits per cell, with A refresh timed
separately. At N128, final A selection was 16.8â€“23.6 ms and explicit refresh about
16 ms; whole publication was about 78 ms. Final B whole publications were
453â€“500 ms, with 27.2â€“56.3 ms selection. Baseline B publications in this small run
were 781â€“844 ms. This combines fewer view files with complete B construction;
caches were uncontrolled and samples are sparse. There are no SLA, p95,
device-cold or cross-OS causal claims.

Profile 2 performs an O(N) metadata scan into one small map. At N128 its ordinary
publication metadata reads were about 0.58â€“0.88 MB. The existing
`documents_visited` metric counts point-header calls and does not count this
range scan; it must not be used to claim that navigation visited two documents.
See [the captured small costs](evidence/matched-small-final-summary.json).

| Source qualification | Windows | Native Linux/ext4 | WSL2/ext4 | macOS/APFS |
| --- | --- | --- | --- | --- |
| Initial corrections: build/tests, protocol/faults, paired intent, replay, backup/lifecycle | success | success | success | unperformed |
| `ec544776`: recreated-directory recovery capsule + strict/packet checks | success | success | success | unperformed |
| `fe1ee22a`: legacy replay/new-target transition | success | success, A/B | success, A/B; strict extras also passed | unperformed |
| Latest complete capsule at `fe1ee22a` | success | unperformed: full cap did not fit inherited deadline | unperformed: same reason | unperformed |
| Final `07a8574e` guard, strengthened pending-backup/oracle checks, paired intent/replay | success | unperformed: host lane closed | unperformed: host lane closed | unperformed |
| Long preservation and reclamation | success | unperformed by allocation | unperformed by allocation | unperformed |

Linux and WSL build/source hashes were checked against the exact core files;
the final Linux `fe1ee22a` binary is
`13e368f6f46e3781f32bf73616477de9fa3828ec762e4e08001f8b0e769aa6db`,
WSL is `a4b728862b86df5940abf9379f995706d155a5e46a9f9aeafec6ccdfb9818d36`.
Native work kept the original 1,800-second host deadlines; rejected late-case
admission was not reset. Evidence collection afterward is closeout, not another
functional lane. This session had no configured Mac shell route and created no
additional top-level platform task. These missing lanes remain actual gaps.

## Bounds, review and remaining limitations

The execution budget was six hours with 30 minutes reserved for termination and
closeout. Ordinary children were bounded, large stages had finite admission,
verifiers were monitored against 2 GiB RSS, and only one new full-history clone
was created. Capture and restore each stayed under their original 1,800-second
caps; the long trial controller finished in 482.860 s.

The preflight admitted a 33.34 GB logical upper forecast under the 32 GiB
(34,359,738,368-byte) ceiling. Closeout recorded 24.59 GB of combined local/WSL
logical files, using fixed preserved inventories plus current owned/guest
censuses. Adding gross removed bytes and a 512 MiB evidence/build reserve gives
a conservative 29.70 GB peak reconstruction, and 4.31 million files including a
100,000-file reserve, below the five-million-file cap. Native Linux new work was
about 592 MB, below its separate 6 GiB allowance. Recorded Windows free space
was 216.90 GB at closeout, above 140 GiB; WSL had 516.23 GB above 40 GiB and
native Linux 49.81 GB above 20 GiB. Directory/FS metadata and physical writes are
not logical file lengths.

Conservative cumulative owner allowances were 6,788,903,037,184 bytes before
the new operations and 6,789,104,628,864 after them. Active reservations are empty.
These repeated allowances include shared/intermediate allocation and are not
physical terabytes of storage.

The focused author causal review is in [review.md](review.md). Qualifying findings
were fixed: namespace ordering/incarnations, external-directory pruning,
legacy/new-target routing, transfer recovery, bounded restore reads and independent
oracle integrity checks. GitNexus impact/detect-changes attempts were made against
the actual worktree, but its stale index lacks this engine and the worktree is
unregistered; schema degradation was also observed. Risk is UNKNOWN from the
graph, with manual caller tracing and focused tests substituted. No clean graph
verdict or comprehensive independent audit is claimed.

The first small backup harness attempt reached no exit hook because refresh was
already current. Its raw failed attempt remains. Superseded verifier attempts,
native admission refusals, and the old proof-export censor also remain; none was
silently relabeled. Raw initial census fields that mislabeled the Windows
compressed-size API as allocated bytes are explicitly superseded by corrected
records. Actual Windows cluster allocation and second-pass per-role gross removal
traces remain unmeasured.

Process exits do not establish reboot/power-loss/SIGKILL durability. Windows
directory synchronization remains unsupported. No broad layout competition,
A100k/B100k render completion, live-history packing/compression, new backend,
production integration, API freeze or successor experiment was performed.

## Evidence and review posture

[evidence/index.json](evidence/index.json) hashes the bounded committed evidence.
`windows-raw.tar.gz`, `linux-raw.tar.gz` and `wsl-raw.tar.gz` contain exact
process inputs/outputs, source bindings, failures and stage receipts. The full
operational transport stays in the owned runtime with authenticated manifest
hashes; it is intentionally not committed as a multi-gigabyte Git object.

The final clone's current readable entry point is
`C:/file-native-recovery-20261019/history-restored/disposable/store/view/collection.yaml`.
The legacy root README is captured historical boilerplate; current discovery is
the profile-2 tree. Historical profile-1 projections retain their original bytes.
The original store, operational archive, restored clone and task worktree remain
available for human/meta review. No next experiment was dispatched.
