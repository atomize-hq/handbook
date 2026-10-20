# Incremental native protocol fixture plan and the 64 MiB feasibility limit

Status: **proposed source-level plan; 64 MiB is not proved for the unchanged
default payload profile**. No engine, corpus, unit or shared harness write ran.
Only this owned native planning document was added. A portable case writer needs
explicit ownership handoff; the matrix-owned harness is unchanged.

Superseding coordinator direction: preserve the 128 MiB new-focus proposal;
**no payload-profile change or reclamation assumption is selected**. The compact
profile discussed below is an unselected alternative only. Target-artifact
forecast reassignment is being investigated separately in `target192-forecast.md`.

## A source-level obstruction, before allocating clones

`baseline_model.rs::fixture` creates seven 4,096-byte, two 16,384-byte and one
65,536-byte document per ten IDs. N64 therefore has exact payload sum:

`6 * (7*4096 + 2*16384 + 65536) + 4*4096 = 778,240 bytes`.

`store.rs::compute` calls `projection::generation` for **every B publication**;
`projection.rs::generation` copies every projection file. Empty operations and
new physical activations cannot substitute for genuine accepted semantic history.
At H64 plus three cycles adding >=16 accepted publications each, >=112 B
generations contain **87,162,880 bytes (83.125 MiB) of document copies alone**.
Originals, navigation, manifests, accepted records, A, owners, quota data and
corrupt/restore copies are additional. No deletion or physical allocation discount
is used in this calculation.

The authenticated donor inventory contains no N64 seed. Its largest B publication
control count is six (for example completed recovery restored-B-128). Even granting
six already-counted publications as generous credit while retaining the same
64-document default payload cohort, 106 further B copies total 82,493,440 bytes
(78.671875 MiB), still exceeding 64 MiB. This is an obstruction for the unchanged
payload profile, not a universal minimum for every possible N64 fixture design.

A fresh full restore of default B H112 additionally needs >=83.125 MiB for its
document copies alone. A 32 MiB restore-coexistence claim cannot cover that image.
The same final-pin physical restore cases already required in the donor packing
capsule can be reused where they prove the identical obligation; that does not
make a separately required deep-B restore small or authorize dropping it.

Consequently **384 MiB final F08 must not be called proved by simply replacing the
earlier 128 MiB estimate with 64 MiB**. With default payloads, the actual necessary
new steady-data bound is already >83.125 MiB before the remaining schedules.
There is no source evidence here for a precise sufficient whole increment.

## A concrete case/reuse plan, independent of the payload decision

Use two primary same-final-pin cohorts, A and B, initialized at N64. Build real
accepted history to H64, then three cycles with >=16 accepted publications each.
Use unique intent-bound requests, count durable successes rather than attempts,
include conflicts and lost replies, and report actual final H. Budget up to H128
for supplemental owner/capture/recovery requests; do not count physical switches
as semantic publications. Cycle 1 uses one publisher; cycle 2 uses four; cycle 3
returns to one or the remaining required configuration. Four independent readers
run during offered load. The >15-second reader survives at least two physical
successors; the capture/semantic gap includes a newly accepted then compacted key.

Keep a permanent explicit case ledger. After each critical crash recover the same
owned cohort monotonically from selected authority, then run the complete small
independent oracle. Never restore a selected control backward to reset a test.
Before-selection, selected-before-journal, loose-delete and actual-old-container
delete prefixes extend corresponding required packing cases; they are not a
second copy of all 52 cases. Failed/aborted candidates remain charged until their
real protocol reclamation is observed; no planned credit finances the next case.

| Schedule | Cohort/extra-copy policy |
| --- | --- |
| Three real rolling cycles; 1/4 publishers, four readers; held-reader/capture gap/two successors | Extend primary A/B once. Preserve each cycle's full byte/outcome/graph/source/navigation oracle, all accepted inputs/results and raw timing/refusal rows. |
| Reader error/cancel/death; maintainer death; two contenders; stale physical CAS | Reuse primary cohorts at named stable boundaries. Use distinct process/owner identities, retain closure and selected-state receipts. No whole-store clone is required solely to kill/recover a process. |
| Same-content path reuse, stale journal, actual pack deletion and restart | Extend the existing required old-container/deletion cases at their new versioned hooks. Verify distinct physical birth paths and inherited work; no duplicate full case family. |
| Preparations, sealed owners, packet ownership, retained snapshots and retired-content refusal | Extend existing operational A/B and focused owner cases, using their already-counted roots. New persistent content only is charged incrementally. Do not retire or revoke a valid blocker to force progress. |
| Paged touched-route/leaf/raw-record/negative-boundary/untouched corruption | One extra quarantined clone of a frozen, quiescent primary A checkpoint with enough real accepted keys for multiple pages/segments. Before each independent corruption, prove the same complete baseline inventory; afterward restore the exact changed bytes and prove the complete oracle again. Save seed hash, exact before/after bytes or bounded page patches, refusal output and audit result per case. Unknown residual changes forbid reuse and require fresh admission. |
| Partial cursor/holes, deferred eligibility, segment pressure and cutoff crossings | Reuse primary cohort or the already-counted partial-build/restore case. The shared final harness must identify the exact new immutable cursor/owner schema; no invented reset or forged acceptance. |
| Raw-size/quota/oversize/backpressure/recovery | At most one new large raw source, one P0 container and one PZ container. Raw source <=8,388,608 bytes; P0 <=8,388,608; PZ <=8,454,144, plus index/control metadata. Total large bodies <=25,231,360 bytes. The already-required >8 MiB exclusion fixture stays counted once in donor geometry. A legal accepted/owned large-record constructor must come from the shared harness; do not pad an already accepted immutable record and call it authentic history. |
| Retained-generation cap/backpressure | Use a small authentic low-H pressure cohort for up to the declared 64 retained physical generations; it need not copy the H112 B semantic history 64 times. Count every physical generation/work root. No >=1,000 native stress repeat: that sampling requirement remains on the primary host, with native focused lifetime/backpressure checks retained. |
| Legacy conversion, old prepared selection/refusal and old-image restore | Extend preconversion primary cohorts and the existing donor legacy/packing restore cases. Historical executable is separately SHA/source/role bound. Never weaken candidate source-pin equality. |
| Final physical backup/restore and accepted successor | Reuse applicable same-final-pin packing/recovery cases only when they cover the same physical version/cursor/owner obligation. If a separate main-cohort restore is required, charge its complete source-image bytes prospectively; no assumption that it fits 32 MiB. |

Maximum extra root/copy topology for the proposed focused implementation:
**two primary roots, one pressure root, one corruption clone, at most one A restore
and one B restore**. Legacy preconversion is a state of an existing primary or
donor root, not an additional deep cohort. At most two archive transports are
retained, one per mode. All restored roots/transports remain charged. No per-case
independent full clone is created when a quiescent exact reset has been proved;
otherwise the plan stops for additional admission rather than omitting a case.

These topology bounds are exact proposed construction rules. Byte and file/directory
bounds for each root must still be derived from the exact final shared writer;
root counts alone do not prove a byte cap. Existing proposed whole-native
50,000-file bound remains a forecast, not a measured count of this future plan.

## Explicit alternative that could remove the B payload obstruction

A **compact correctness-only payload profile** is a possible deliberate fixture
selection before any evaluation: initialize the ordinary N64 store, then use one
authentic, captured-base `import` publication with 64 operations to replace filler
with exact short authored sources. Keep IDs, title/path/kind/edge semantics,
Unicode/line-ending/JSON-number examples and ordinary browsing instructions.
Initial source text <=256 bytes/document; subsequent edits stay <=512 bytes.
Capture and independently verify every new exact byte, count that publication in
H, and retain the original default payload/genesis bytes. No engine source or
storage format changes are involved. The full donor/default-size capsules and
the raw 8 MiB quota/codec cells remain unchanged and separately required.

At H112 this profile's B document-copy bound is at most
`778,240 + 111*(64*512) = 4,415,488 bytes`, instead of 87,162,880. At H128 it is
4,939,776 bytes. That removes the document-copy obstruction **without removing
N/H/cycles/readers/owners/crash/corruption/restore schedules**. It is an explicit
workload-profile change, not an unannounced equivalence/performance claim.

It still does **not prove 64 MiB for the whole implementation**: final Work2,
rolling/paged metadata, COW replay files, legal quota construction, exact resets,
and backup transport/restore coexistence must be source-bound first. Per-root
64/32/16 MiB envelopes may be proposed after that selection; arbitrary quota labels
or an assumption of compression/reclamation cannot stand in for the proof.
No compact-profile writer was implemented or assigned ownership by this worker.

Current conclusion: the requested 64 MiB target is not supported for unchanged
default payloads. Either deliberately select and prove the compact functional
profile before freezing the final harness, or retain a larger admission bound
based on the actual final construction. No final F08 growth is released here.
