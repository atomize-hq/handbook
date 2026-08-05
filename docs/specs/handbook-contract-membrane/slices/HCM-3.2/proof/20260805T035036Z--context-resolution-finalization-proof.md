# HCM-3.2 Context Resolution finalization proof

Status: implementation discovery findings `HCM32-JCS-IMPL-DISC-004`, `005`,
and `006` received one consolidated remediation. Different-fresh closure then
returned three directly unmasked P2s under those same IDs. Causal supplemental
1 closed `004` and `006` but retained an anchor-crash bypass under `005`.
Causal supplemental 2 repairs only that directly unmasked issue and passes the
complete proof wall. CLEAN in exact dispatch
`20260805T090000Z--HCM-3-2--finalization-implementation-supplemental-2`
completes implementation acceptance; any other verdict leaves it incomplete.

Parent: `20260805T014559Z--HCM-3-2--context-resolution-finalization`

Outcome: `hcm-3.2-context-resolution-finalization-whole-slice`

Packets: `HCM-3.2-FINAL-P2-product-remediation` and
`HCM-3.2-FINAL-P3-proof-and-review`

Base: `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`, tree
`d51327c442190edcf1bf56423db54fd2522c5e6f`

## Recovered and admitted product subject

The protected `9586` worktree was read only. Its twelve tracked product paths
were reconstructed in the assigned checkout with the obsolete direct-seed
authority fixture deleted. The recovered candidate was not trusted as final.
Fresh repairs preserve generic markerless refusal, admit only the exact
recorded quarantine candidate before publication mutation, close only an exact
committed T2 chain, retain an immutable open diagnostic plus an append-only
completion receipt, reconcile a post-T2 crash from that exact chain, and retain
a live cryptographically revalidated admission inside every operational
envelope, escalation candidate, and promotion request.

Production changes are limited to:

- `crates/engine/src/artifact_lineage_store.rs`;
- `crates/engine/src/artifact_mutation.rs`;
- `crates/engine/src/context_resolution.rs`.

Selected tests and the existing HCM-3.2 fixture tree are the only other
product paths. There is no public signature, dependency, crate, unsafe/native,
platform, transport, tooling, HCM-0.11, H2/H3, or HCM-3.3 change.

## Stable finding disposition

| Finding | Exact evidence | Disposition |
|---|---|---|
| `HCM32-JCS-IMPL-DISC-001` | Five hash-consistent signature, RP-ID hash, flags, counter, and challenge substitutions preserve the deterministic publication key and reach the real publisher cryptographic verifier. | closed by GREEN focused and complete kernel proof |
| `HCM32-JCS-IMPL-DISC-002` | Admissions retain `CurrentAuthorityWitness`; envelopes, escalation candidates, and promotion requests retain the admission. Cache hits, cloned envelopes, mutation, memory, validation, child/candidate/request construction, and transition admission re-run committed lineage, live registry, retained assertion/response, and cryptographic currentness. | closed by stale committed-chain and complete consumer proof |
| `HCM32-JCS-IMPL-DISC-003` | Retry availability reconstructs the authentic live transition and verifies the exact one-mapping registry delta, current credential/use-head, response/challenge, capsule predecessor, and generic committed chain. | closed by recovery classification and replacement proof |
| `HCM32-JCS-IMPL-DISC-004` | Selected authorized-without-intent, generic-pending, and installed-without-commit phases plus retry, invalid, predecessor, journal-conflict, installed-result, zero/one/multiple candidate, and fault-boundary behavior converge or refuse. Generic non-HCM promotion refuses a markerless installed output lacking its durable native result with the state tree byte-identical. Post-T2/pre-closeout and partial-closeout crashes reconstruct only the exact committed receipt. | causally remediated; supplemental 1 closed it with GREEN real zero-candidate, exact multiple-candidate classifier, 32-test kernel, and 55-test generic-lineage walls |
| `HCM32-JCS-IMPL-DISC-005` | Lock-bound pre-admission compares transition, deterministic idempotency key, candidate ref, candidate fingerprint, and the append-only original-record anchor before publication. Inventory validates anchor/open/committed identity before scratch cleanup or absent-open success. Anchor-only and committed-only states refuse without mutation; only an exact anchor-bound retry may reconstruct the open record. Completion independently verifies the retained ledger, committed intent, candidate, publisher transition, canonical output, result, and evidence chain; exact replay is byte-stable. | causally remediated twice; self-refingerprinting was RED before supplemental 1, and anchor-only absent-open bypass was retained by supplemental 1 before the supplemental-2 repair; exact supplemental-2 CLEAN condition applies |
| `HCM32-JCS-IMPL-DISC-006` | This proof distinguishes the original, post-discovery, causal-supplemental-1, and causal-supplemental-2 walls; records observed RED/GREEN results; and makes canonical completion conditional on the exact final supplemental verdict. It reports unavailable FTS honestly and neither publishes the quarantined checkpoint nor borrows historical proof as current capability. | supplemental 1 closed the truth claim; only exact supplemental-2 CLEAN completes the whole implementation subject |

## RED and GREEN evidence

The test-first RED command was:

`cargo test -p handbook-engine --test context_resolution_kernel publication_gate_cryptographically_refuses_forged_signature_rp_flags_counter_and_challenge -- --nocapture`

It failed before production recovery because
`GenericArtifactLineageStoreV1::record_hcm_publication_quarantine` did not
exist. After implementation, the same test passed 1/1 in 197.72 seconds.

Focused GREEN results:

- publisher cryptographic substitution: 1/1, 197.72 seconds;
- cached admission and cloned operational-envelope currentness: 1/1, 46.87
  seconds;
- original quarantine optional/idempotency/phase matrix: 1/1, 85.83 seconds;
- original generic markerless installed-subset recovery: 1/1, 12.07 seconds.

The fresh implementation discovery burst then reproduced three distinct RED
boundaries:

- exact completed retry failed because the sole record required
  `resolution=open` during replay;
- a public promotion whose valid candidate differed from the exact quarantine
  identity mutated canonical and journal state before closeout refused;
- a generic refused HCM result incorrectly rewrote quarantine as committed.

After one consolidated repair, the focused results were:

- exact retry plus byte-stable second replay: 1/1, 192.34 seconds;
- different valid recorded candidate refuses before canonical, journal, or
  quarantine mutation: 1/1, 117.15 seconds;
- refused HCM result preserves the open diagnostic and previous authority:
  1/1, 120.46 seconds;
- post-T2/pre-closeout recovery with a partial completion scratch: 1/1,
  140.79 seconds;
- generic markerless promotion missing its native result refuses with an exact
  state-tree snapshot: 1/1, 34.63 seconds;
- complete Context Resolution kernel: 31/31, 532.37 seconds;
- complete generic lineage: 55/55, 266.75 seconds.

Complete walls:

- `cargo test -p handbook-engine`: exit 0 in 1,187.9 seconds; all engine unit,
  integration, and documentation targets passed;
- `cargo check --workspace --all-targets`: exit 0;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  exit 0;
- `cargo fmt --all -- --check`: exit 0;
- `git diff --check`: exit 0;
- `cargo test --workspace`: exit 0 in 1,546.1 seconds across CLI, compiler,
  engine, flow, pipeline, and documentation targets;
- ordinary Handbook handoff validation: 85 records, 495 current dispatches,
  8 admitted legacy dispatches, 85 ledger entries, exit 0;
- historical v1 admission, v1.4 causal-contract, and orchestration-contract
  self-tests: all passed, exit 0.

The supplemental-2 wall adds:

- anchor-only and committed-only quarantine states refuse admission,
  completion, and reconciliation without mutation, while an exact anchor-bound
  retry reconstructs the open record: 1/1, 110.46 seconds;
- an identity-mismatched open record refuses before deleting a partial
  completion scratch: 1/1, 117.61 seconds;
- complete Context Resolution kernel: 32/32, 595.41 seconds;
- complete generic lineage: 55/55, 258.51 seconds;
- `handbook-engine` all-target/all-feature tests: exit 0 in 1,280.9 seconds;
- workspace all-target/all-feature tests: exit 0 in 1,630.2 seconds;
- workspace check, strict all-target/all-feature Clippy, formatting, and
  whitespace: exit 0.

The HCM-3.2 authority fixture replay is part of the 32/32 kernel wall and
verifies exact definition fingerprints plus repository profile selection. The
workspace wall replays the existing definition/profile/vocabulary and L0-L3
compatibility surfaces without changing their shipped identities.

## GitNexus and limits

Fresh upstream impact confirmed CRITICAL risk only for the selector-approved
seams: `validate_persisted_output_authority` (187 impacted),
`validate_request_subject` (27), `recover_pending` (52), and
`verify_committed` (25). `promote` is MEDIUM (10) and the other edited existing
seams are LOW. No unexpected HIGH/CRITICAL edited symbol was admitted.

An index-only exact-worktree analysis produced 20,033 nodes, 43,126 edges, 442
clusters, and 300 flows without repository file injection. Scoped all-change
detection reports 13 product files, 159 symbols, 36 affected processes, and
CRITICAL aggregate risk, matching the selected authority/currentness/recovery
surface. Compare-to-`main` detection is available but necessarily reports the
whole feature lineage: 1,355 files, 9,910 symbols, 259 processes, CRITICAL.
FTS/BM25 remains unavailable because the LadybugDB FTS extension is absent;
it is not reported GREEN.

Hand-written product additions are 2,036/2,400 across three/five production
paths: lineage 577/600, mutation 173/500, and Context Resolution 1,286/1,300.
Selected test additions are 1,573 lines with twelve material new tests, within
the 18-test ceiling.

## Authority and deferred boundary

Historical committed proof establishes only authentic displaced history.
Operational promotion, recovery, resolution, and consumption require the live
committed authority, current registry witness, retained publisher proof, exact
predecessor quartet, and exact current generic lineage. Revoked, stale,
marker-only, pending, ambiguous, substituted, or restored-old state refuses.

The staged H2 installation-authorization/H3 operational-activation design
remains deferred and non-authoritative. This implementation accepts the
selected fail-closed no-authority interval and does not implement continuous
availability.

The first different-fresh closure and causal supplemental 1 are recorded in
`20260805T072900Z--finalization-implementation-supplemental-remediation.md`.
The retained anchor-crash finding, exact bounded repair, new negative proof,
full rerun wall, and no-post-CLEAN-cycle condition are recorded in
`20260805T085700Z--finalization-implementation-supplemental-2-remediation.md`.
