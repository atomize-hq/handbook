# Independent causal review

Reviewed the initial isolated Rust implementation and `protocol.md` against the
controlling October 17 `../dolt-gitbutler-file-engine-2026-10-16/experiment.md`.
This is the focused independent review under `code-review-and-quality`; findings
below describe the reviewed source before remediation, not a completed execution
or production approval. No source was changed by this reviewer. Execution tests
and measurements were still being prepared at review time.

## Critical

- **Incomplete original writes can poison retries.** `model.rs::put_original`
  writes directly to the deterministic final `originals/{digest}/{name}` using
  `io::write_new`. A real short write or I/O failure after creation can leave a
  partial final file. A later same-intent attempt sees the file and fails its
  digest check indefinitely. Existing `file-write` injection occurs after
  `write_all`, so it does not reproduce a partial write. Prepare originals in a
  uniquely owned temporary path, sync and verify, then install the immutable final
  name without overwriting another attempt's final; verify racing installed
  content. Test a partial-write failure and subsequent successful same-process
  retry. Selected corruption must still be reported rather than silently repaired.

## Required

- **Qualify returned postselection errors.** In the initial `select_locked`,
  errors from `selector-visible`, containing-name sync, `selection-boundary`,
  receipt creation/checkpoints and `reply` propagate to `main` as the same plain
  `status:error` used before selection. Trigger: inject a reported error at
  `receipt-before`; authority and original result are already selected, but the
  response has no selected publication or qualified outcome. Preserve phase,
  publication and original result: visible but not boundary-complete may be
  uncertain; a succeeded boundary followed by delivery failure is published.
  Resolve beneath a later successor without replay. Process exit has no reply;
  its oracle must inspect retained selected history and exact original result.

- **Require an import base on every admission route.** `publish` initially
  checks `import` plus null base only under policy `slot`. The speculative and
  direct `prepare` routes accept it, capturing whichever base happens to be
  current when preparation runs. Move the rule to common preparation/admission
  validation. Test both policies and direct preparation, plus later editor-file
  changes after a valid exact-byte capture.

- **Reject duplicate JSON keys before the intermediate Value loses them.**
  `main` initially parses stdin into `serde_json::Value`, then constructs typed
  requests from that Value. Duplicate request/operation keys have already become
  last-value-wins, contrary to the protocol's duplicate-field refusal claim and
  the structured-format boundary. Reject duplicate keys recursively before Value
  construction, or decode an appropriate typed envelope directly. Test nested
  duplicate fields as well as the outer envelope.

- **Keep accounting and scope claims literal.** A `refresh` scans/hashes all
  visible documents, enumerates the namespace and rewrites complete navigation
  even for one changed document. It copies only changed document payloads, but
  total read/hash/navigation work is not proportional only to changes. Report
  those costs separately, including failed preparations and rereads at selection.
  Execution counts, fixture allocation, retained bytes and growth coverage must
  come from actual runs; protocol allocations are not observations.

## Explicit qualification gaps

- Native Linux/ext4 and its ordinary desktop file-browser exercise were
  unavailable at review time. WSL evidence cannot fill that requirement.
- Windows `sync_dir` records unsupported operations rather than a successful
  directory flush. This can support the stated process-interruption experiment,
  not nested-name persistence after OS crash/power loss. Linux name synchronization
  also needs attention after an interrupted `mkdir`: the existing-directory
  early return skips re-establishing its parent-name synchronization.
- No actual GC, pin-registration/GC race or backend retention qualification exists
  in this file-only implementation. Retaining everything demonstrates retained
  readers only. Gate 3 must remain conditional on valid earlier evidence.
- The ordinary Explorer exercise, Windows no-delete-sharing handle behavior,
  external kills, actual fault checkpoint counts, sparse growth observations and
  independent-store progress need executed evidence. Scripted path opens cannot
  stand in for desktop usability.

## Optional

- Keep the intentional bounded representation simple. Publication identity is
  distinct from state identity; predecessor links, unique candidate paths,
  under-lock dedup/fences, exact original results, B destination hashes/names and
  A render-only resumption are sound design directions in the reviewed source.
  Do not generalize this experiment into an automatic merge engine.
- Record current source hashes or a remediation note with rerun evidence, so this
  initial review cannot be mistaken for an outstanding-defect list or a final
  clean review of different bytes.

Verdict at initial review: **request changes** for the concrete causal issues
above. Both browsing options remain open; mixed ordinary reads alone are not an
A defect, and this review makes no backend or production selection.

## Remediation recheck

Rechecked the same causal issues after the experimental source changes; no new
native child workloads were launched by this reviewer. The rechecked source
hashes were `io.rs` 3bc42be5f1af45e2fef7b2497238664974df4660ff4345b2f4dd985bfb1b788f,
`model.rs` ba940416b1bba8b5585bda42127f37f256d29d1951eb7e198d5c67564fbac7f9,
and `main.rs` e074b9c553a73cfbd9cfff3cf2943be30cbacbf7f86e0933f44b8acf0f0257a3a.
Later revisions need their own build/evidence association.

- The deterministic partial-original poisoning mechanism is fixed: partial
  bytes now remain in a unique payload stage, and only fully written, synced,
  hash-checked bytes reach the final path. Recorded partial-write cases for A/B
  show successful later retry and audit. The final install still uses replacement
  after an existence check; two identical verified producers can race and replace
  the same final name. This preserves identical payload bytes under the measured
  model, but is not a proof of exclusive installation or unchanged inode/name
  identity. A no-delete-sharing handle can cause a bounded refusal; do not claim
  a true no-replace installer.
- Common `prepare` now rejects imports without a captured base. The supplemental
  script explicitly tests slot, speculative and direct-prepare routes; its
  execution was pending at this recheck.
- `strict_json` recursively rejects duplicate map keys before constructing a
  Value. The recorded outer duplicate is refused; a nested-operation duplicate
  test is also prepared in the supplement. Exact authored JSON bytes continue
  to bypass reserialization.
- Selection/delivery errors are phase-qualified as `selection-uncertain` or
  `published/reply-pending`, retaining proposed publication identity and the
  original result after the boundary. A postpublication refresh failure remains
  published/view-pending. Recorded Windows no-delete-sharing refusal is
  conservative uncertainty followed by successful explicit retry.
- A now uses a positively selected historical state identified by a complete
  view status to skip unchanged payload reads. It still enumerates the namespace,
  constructs state/sets and rewrites full navigation. This optimization assumes
  the managed projection was not independently edited/deleted after its complete
  marker; neither ordinary view integrity audits nor healing arbitrary external
  view damage are established by that marker. Incomplete refresh falls back to
  byte comparison and rendering rather than mutation replay.
- A's retained packet promises eight explicit ordered inputs and their output,
  not the complete graph or all attachment targets. B's manifest includes all
  collection documents and packet input/provenance. Stable-ID relationships in
  regenerated navigation follow renamed paths; exact original headers are not
  rewritten just to match new titles.

Read-only evidence inspection found **120 race observations** and **158 fault
rows**. No recorded preselection row had selected the attempted publication; no
recorded selection-boundary/receipt/reply/A-refresh row had lost it. The initial
fault test nevertheless accepted either observed selection branch at every
checkpoint. Strengthen its assertions to make these phase-specific expectations
part of the executable oracle, and preserve exact original outcomes under later
successors. Generic helper fault names fire at their first occurrence: these
counts do not establish every later file/directory/sync operation was faulted.

One additional concrete closure issue was reported during this recheck: the
new deeply placed attachment retained the authored `../NAVIGATION.md` locator,
which resolves to a nonexistent path from its nested directory. Remove that
inappropriate attachment locator or produce a depth-correct locator, and validate
required local links in addition to manifest names/hashes. Renaming a source
across directory depth must not leave an advertised required locator broken.

The initial Critical/Required items are therefore remediated or explicitly
bounded as above. Final closure-link correction, strengthened fault-oracle
validation and supplemental execution were still pending at this recheck.
Native Linux, ordinary desktop browsers, crash/power persistence and GC/backend
qualification remain gaps; successful local process tests do not complete them.

## Final targeted recheck — corrected v3 evidence

The same review's remaining remediation items were checked against frozen source
and `C:/file-native-experiment-20261017-v3` observations. The measured executable
SHA-256 is `1b9a6caef1113d0c6bb62c52189feb2126bdb531c407b728cd6b3c36b0d9e584`,
confirmed by an independent file hash. Frozen source SHA-256 values are:

- `io.rs`: 026972d7d3af2dce198c5fe9a67abde237e1e8320233637acc1e320a2eff47c4
- `model.rs`: b1602ca5d86c65a66a66de2df897f00c7481d84f90bda25b3baab8c4d52ff514
- `main.rs`: 52f2142ee85f2749108ddd9003ae6343f23c7059080c3ddc37dd01355caa6f03

The Windows original installer now uses no-replace MoveFileExW with WRITE_THROUGH,
verifies an already installed same-content winner, and bounds transient retries.
Concurrent creation of an already-existing directory is accepted only when it
is a directory. This addresses the real v2 concurrent-install refusal; v2 is
preserved stopped evidence, not counted as a successful corrected run. The Unix
fallback still uses an existence check plus rename and is not an exclusive
installer qualification. Native Linux remains unexecuted.

The deep attachment's source locator is now depth-correct. Both closure
construction and B selection verify the fixture's Markdown-link and explicit
NAVIGATION-locator grammar, in addition to bytes and names. This is a bounded
fixture parser, not general Markdown/YAML/JSON reference interpretation. A's
saved packet now retains the transitive required local dependencies/attachments
of its eight ordered assembly inputs and generates navigation for that retained
subset. This supersedes the earlier first-eight-only closure description; the
5,000-constituent fixture can consequently increase A's retained packet cost.
Renaming across depth while preserving an incompatible embedded locator is
refused by closure validation rather than silently publishing broken navigation.
The tested successful rename/edge combination uses stable identity and valid
same-depth placement.

Read-only inspection confirmed:

- 120 main race observations across both consumers/options.
- 233 fault rows: 174 `must-not-select`, 51 `must-retain`, six
  `visible-uncertain`, and two partial-payload retry rows. Zero must-not-select
  rows selected; zero must-retain rows lost their publication. The driver now
  enforces those phase-specific assertions before recovery/retry. Named hooks
  additionally cover state/publication/selector/manifest/receipt write/sync and
  generation/view copy phases. Hook coverage remains bounded to the exercised
  invocation/first matching instance, not every possible operation or power cut.
- 480 semantic combination observations: separate initially valid candidates,
  whole-publication conflict, then explicit revalidation of delete/add-edge,
  uniqueness, budget and stable-ID rename/edge combinations.
- Two suspended userspace-copy observations: A's mutable copy is mixed; B's fixed
  retained-generation copy is coherent and passes byte/provenance verification.
- Nine supplemental validation checks, including all three import admission
  routes for each option and nested duplicate-field refusal. Missing A projection
  bytes leave immutable authoritative audit intact; this is not a claim of
  arbitrary projection-damage healing.

**Final causal verdict:** no unresolved blocking finding in the remediated,
bounded Windows fixture protocol/evidence reviewed here. The initial findings
above are historical and superseded by this section. v1's deep-locator evidence
and the failed supplemental Windows path-enumeration attempt remain preserved;
the corrected copy checks used fresh paths and completed.

This verdict supports publication/outcome/provenance behavior under the measured
native Windows process-fault model only. Growth was still running during the
recheck and its completion/performance claims are outside this final verdict.
Native Linux/ext4, actual ordinary Explorer/Linux-browser exercises, Windows
nested-name crash persistence, macOS, actual GC and backend integration remain
unmet. Gate 3 was not entered. Neither browsing option nor a production backend
is selected, and the standalone experimental report must remain partial wherever
those obligations or allocated growth tiers were not reached.
