# HCM-4.2 corrective documentation implementation proof wall

**Selector:**
`../decision/20260811T025558Z--hcm-4-2-causal-remediation-selector.md`
**Implementation dispatch:**
`../../../handoffs/dispatches/20260811T030610Z--HCM-4-2--causal-remediation-documentation-implementation.json`
**Scope:** bounded corrective planning documentation only
**Runtime status:** no DTO, schema, operation definition, catalog entry,
transport, Rust/Cargo, generated asset, or public API was implemented

This is additive corrective evidence. It does not rewrite or replace the
historical dated `20260811T005000Z--planning-proof-wall.md` or historical
review/handoff/dispatch bytes. Final review and closeout state belongs only in
later additive evidence owned by the parent orchestrator.

## Frozen-input verification

| Gate | Result |
|---|---|
| implementation-dispatch SHA-256 | PASS — `ef265d511174c37ed81742663f65c41c2e2b303cbde69a0f92b17e98fbdc4e30` |
| dispatch subject fingerprint | PASS — `sha256:e867a311e7e4cf38dce13039fa42a71c047ecec7b2a53b87267244ca9536b73e` |
| v1.4 `--verify-dispatch` before edits | PASS |
| external audit receipt SHA-256 | PASS — `3d30632a5827d8bdcec7adbe47aef77ff387380d5c143839ca9b1b4a8b8498d0` |
| immutable predecessor proof/selector/review/dispatch hashes | PASS — five exact hashes rechecked, zero mismatches |
| GitNexus | `UNAVAILABLE_NOT_GREEN`; no code symbol edited |

## Ten-finding disposition

| Finding | Corrective disposition and proof |
|---|---|
| `HCM42-CAUSAL-P2-001` | CORRECTED — raw mutation key is legal only as bounded input at the definition-pinned typed-body pointer; positive owner-delivery and literal scans of every output/replay/diagnostic/receipt surface are mandatory. |
| `HCM42-CAUSAL-P2-002` | CORRECTED — SPEC/05 contain the exact seven live request fields, exclude caller policy/approval/head inputs, and map all 14 live owner variants to exact status, Problem/details, idempotency, data, and two-or-zero receipts. |
| `HCM42-CAUSAL-P2-003` | CORRECTED — bootstrap request/response/refusal, descriptor, OwnerVersionEntry, and CapabilityEntry are closed with exact fields/tags/bounds/null/default/preimage/owner-version/transport rules and distinct descriptor content/schema identities. |
| `HCM42-CAUSAL-P2-004` | CORRECTED — LF-free JCS instance/fingerprint bytes, exactly-one-LF checked-in schema bytes, CLI zero-or-one LF, and Tauri-owned framing are separate. |
| `HCM42-CAUSAL-P2-005` | CORRECTED — the authoritative matrix has exactly 62 canonical rows: 26 `live_precursor`, 24 `absent`, 12 `phase5_deferred`; all 62 current operation transports are none and discovery results are `omit`. |
| `HCM42-CAUSAL-P2-006` | CORRECTED — current planning paths name the additive corrective selector and CLEAN selector review lineage; historical evidence remains unchanged. |
| `HCM42-CAUSAL-P2-007` | CORRECTED — 03/04/05/06, SPEC, plan, todo, strategy, matrix, and this additive proof wall align; immutable-primary files contain no future-stale review/closeout checklist state. |
| `HCM42-CAUSAL-P2-008` | CORRECTED — parity binds semantic/result/Problem/receipt/replay and `original_result_fingerprint`; each transport recomputes correlation-sensitive outer response identity; changed-valid and absent/null/invalid request-ID vectors are exact. |
| `HCM42-CAUSAL-P3-001` | CORRECTED ADDITIVELY — historical bad proof ref is preserved, identified as non-resolving, and superseded in current correction evidence by exact resolvable `git:0a4366802467edbe194da9afaf584686921c1e94`; parent closeout records the same fact without editing history. |
| `HCM42-CAUSAL-P4-001` | CORRECTED — authoritative wording and shape proof use four terminal fields: `data` plus `blockers`, `refusals`, and `errors`. |

## Mechanical matrix and source proof

The matrix parser selected only rows matching `^| M62-NNN | operation |` and
the canonical 05 inventory only between the SDK ordinary-use-case and
operation-definition headings. Results:

```text
matrix_rows=62 canonical_rows=62 unique_ids=62
missing=0 extra=0 duplicate_groups=0 sequence_missing=0
phase5=12 omit=62
live_precursor_rows=26 absent_rows=24 phase5_rows=12
HCM-4.1 plan count=62 missing=0 extra=0
predecessor HCM-4.2 inventory count=62 missing=0 extra=0
```

Twenty-six primary live-precursor row symbols plus the named posture and Flow
owner-chain symbols were checked in their exact source files:

```text
live_symbol_checks=28 failures=0
```

`repository.setup.plan` remains `absent` because its similarly named SDK helper
is `#[cfg(test)]`. `pipeline.route.resolve` remains omitted because the live
precursor persists route-basis state while the planned operation is read-only.
Neither case is silently promoted.

## Proof-ref supersession

Strict commit-object resolution produced:

```text
git:0a4366802467edbe194da9afaf584686921c1e94 exit=0
git:0a43668c5bdb2d80ee0ad9e6a6f2ce7af0d7b676 exit=128
```

The second value remains byte-for-byte inside the historical handoff as prior
evidence. Current correction evidence and the future additive completed
handoff use only the first exact proof ref.

## Changed-scope proof snapshot

The implementation changed only mutable documentation primary paths plus this
additive research/proof evidence. `git diff --check` passed and the scoped
Rust/Cargo/generated-schema diff count was zero. No selector, selector review,
dispatch, predecessor proof wall, historical review/handoff/dispatch, schema,
validator, template, code, Cargo, generated asset, protected checkout, ledger,
ref, or remote was edited. The parent owns any later complete-subject review,
commit, handoff/ledger rebuild, CAS, and terminal receipt.

Additional static validation passed:

```text
relative_links_checked=31 missing=0
utf8_files=10 decode_failures=0
windows_absolute_path_hits=0
trailing_whitespace_hits=0
stale/incorrect wording hits=0
v1.4 admission self-test=PASS
v1.4 causal + orchestration contract self-test=PASS
```

The self-tests emitted only the repository's line-ending advisory for the
ledger; post-test status/diff confirmed the ledger was not changed.

## Parent pre-review convergence

The parent orchestrator independently reproduced the implementation proof
before freezing the complete final-review subject. This section is part of the
primary subject; later review, commit, handoff, ledger, CAS, and terminal-
receipt facts must be additive and must not rewrite it.

### Repository and publication boundary

| Check | Reproduced result |
|---|---|
| detached `HEAD` / tree | `ea2be46856172e503e7f7668d2dbaac9d26d1d39` / `9dbad2c418a331d5b2a0c266c55280d81b676aca` |
| dedicated local integration ref | still exactly `ea2be46856172e503e7f7668d2dbaac9d26d1d39` before either commit |
| required ancestry | `git merge-base --is-ancestor ea2be46856172e503e7f7668d2dbaac9d26d1d39 HEAD` exit `0` |
| live origin baseline | `refs/heads/feat/handbook-contract-membrane` remains `1256e724a2b7da6b6250f57d6f63fced1e2cf949` |
| live origin integration ref | absent |
| external audit receipt | SHA-256 `3d30632a5827d8bdcec7adbe47aef77ff387380d5c143839ca9b1b4a8b8498d0`; exactly ten unique mandatory IDs |
| publication | `local_only`; no fetch, push, merge, rebase, reset, clean, or remote mutation |

### Protected-checkout and immutable-history proof

The content manifest excludes `.git` and the main root's build-only `target`
directory and hashes the sorted `relative-path NUL byte-length NUL file-sha256
LF` stream. Every value below equals the preflight value. The main-root
`target` metadata also remains exactly 34,600 files and 43,948,366,846 bytes.

| Protected root | HEAD / tree | index SHA-256 | files / bytes / content SHA-256 |
|---|---|---|---|
| user main checkout | `cc44a84c0f5b75f336301dac0c502cfae909c17e` / `54309db1e88e6cfeb1611cf18dfab2d07c10d487` | `801ca29cd4373a05350613f348e1a2b12d57ce832b1202b25f347958651252ee` | 2282 / 361501965 / `f5cc7ebe8eb2149def12f4ec53e869d4b47d55c27ec391e733ae467594d4f143` |
| worktree `1011` | `818d3662f57fcda867a59b7f9e035755fee1b393` / `6dd3872e93bcad6604e78534d05e2ec14a0f8aea` | `62222758fe2fce073776b03125c55fc0edb17f9217db4e2ef5f54945b1bd3e60` | 2259 / 29573494 / `2e88aadbe393be8cf06a65e57de816ed9766afd2453b6659b83a1d66867b10bc` |
| worktree `8993` | `818d3662f57fcda867a59b7f9e035755fee1b393` / `6dd3872e93bcad6604e78534d05e2ec14a0f8aea` | `06dbcce306c1a39627dea74692675fd2f0f398d65d9f31a6c040b9933f2d0105` | 2258 / 29568158 / `e69e6fedaf2913ed8b45b41ad6b9632c0efaa6309895ed880315ba5d8e58ad81` |
| worktree `f864` | `8c2e31b518007b4a2b14f36f128d35b04c6e632e` / `dfe7fac4370e9f6ca20fba0fa22e9b4dd4098f4d` | `be268ba516f02429d5b0f92cb56e6782979bc3973443311be63bc6e38ad8ee04` | 2238 / 29202569 / `ebd744e3e0d4337387963956f6b5665f15a844af4079997f28d6286976d5173c` |
| worktree `086a` | `ea2be46856172e503e7f7668d2dbaac9d26d1d39` / `9dbad2c418a331d5b2a0c266c55280d81b676aca` | `3407b2a8b6063c139d1205679a90dc2e0a5e3bb435ac49d69c106eb4bbc9dcf2` | 2270 / 29671931 / `988a153e42082cc6d82b5d50e801e8d88b8005b9554dc0a3d53a7df97765b092` |
| worktree `79cc` | `ea2be46856172e503e7f7668d2dbaac9d26d1d39` / `9dbad2c418a331d5b2a0c266c55280d81b676aca` | `27bfd4e81b3b0ce18413dd88ebd84212a11ecf919d5093d1ff5c539ebe51d731` | 2270 / 29671931 / `988a153e42082cc6d82b5d50e801e8d88b8005b9554dc0a3d53a7df97765b092` |
| worktree `34bb` | `ea2be46856172e503e7f7668d2dbaac9d26d1d39` / `9dbad2c418a331d5b2a0c266c55280d81b676aca` | `7288e778a3a9d24e074c63bfbf1fa3c77485c50e44b8b35f933940056f24b498` | 2270 / 29671931 / `988a153e42082cc6d82b5d50e801e8d88b8005b9554dc0a3d53a7df97765b092` |

The predecessor completed handoff, its two historical dispatches, selector,
two review files, and dated planning proof wall all remain equal to the base
bytes. Their current SHA-256 values are, in that order:

```text
9dfe1348054fb3850e8709dc1a73094d9d84d40fe3c720396d05534f366980b2
1f4030a270f47ed18273f729983e104bcc75eba2bfbc451cdaf0fea28e3758b3
f3ecaaff3ab294c5dce61edd0b0103b691af74635228ce7f5b0c46adec35a5d5
abab88e573178bbd94b17a64aa02dcf4456536df3397d6464a3dba641bef1f0f
e5f2c98b27c4531efca8a6bc1bc158bb4e7b1549633f6567584b0042bab9466b
00ec2c9aa7f56f7dcdc55eeea07a45fe852f18561d8b47f9fb2ea2c4df6429d1
089f596283877bc1f3ca1889197c9a0db3f5b4a52d284a8583a2df748f9d7798
```

### Reproduced product and process assertions

- Matrix, canonical 05 inventory, HCM-4.1 plan, and predecessor inventory are
  each exactly 62 unique IDs with equal sets. `M62-001..062` is contiguous;
  the matrix contains 26 live precursors, 24 absent rows, 12 Phase-5-deferred
  rows, 62 `omit` decisions, and 62 rows with no current admitted transport.
- All 29 exact `path::symbol` references in the matrix resolve mechanically.
  Twenty-eight are the live-precursor/owner-chain checks; the remaining
  `crates/sdk/src/setup.rs::plan_setup` reference is proven `#[cfg(test)]` and
  supports the explicit `absent` disposition.
- The live posture request has exactly seven fields, the public receipt has
  exactly seven fields, and the owner result space is exactly two success
  variants, one blocker, eight refusals, and three errors. The corrective
  tables cover every variant exactly once and bind status, Problem/details,
  idempotency, data, and two-or-zero realized receipts.
- Positive mutation-request proof requires the bounded raw key exactly at the
  selected definition's typed-body pointer. Negative proof rejects the literal
  raw value from responses, Problems, diagnostics, receipts, replay/tombstone
  state, captures, human output, and every other serialized or durable output.
- The generic response assertion requires the exact key set, `data` plus the
  three terminal arrays, and status exclusivity. Replay preserves semantic
  data, Problems, realized receipts, and original-result identity; valid
  correlation changes only the echo and recomputed outer response fingerprint,
  while absent/null/invalid correlation follows the frozen pre-owner refusal.
- The resolvable primary proof ref exits `0`; the malformed historical ref
  exits `128`. Current evidence uses only
  `git:0a4366802467edbe194da9afaf584686921c1e94`.

### Validation and exact primary manifest

Ordinary handoff validation is `FAILED_NOT_GREEN` with exactly the permanent
governance contradiction
`20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation
writes or advances before selector CLEAN`. No new failure is hidden or treated
as green. `--self-test-v1-admission` and
`--self-test-orchestration-contract` both exit `0`. The corrective selector
dispatch still verifies against its one-file subject; the frozen implementation
dispatch verified before launch and, after the authorized transformation,
correctly rejects the changed input subject rather than pretending it is a
post-edit dispatch.

GitNexus MCP is not exposed and `.gitnexus/run.cjs` is absent; status is
`UNAVAILABLE_NOT_GREEN`. Because no code symbol changed, exact source-symbol
checks plus base-scoped manual change detection are the bounded fallback.
UTF-8, JSON parse, relative-link, final-LF, trailing-whitespace, and
`git diff --check` checks pass. No Rust, Cargo, generated schema, validator,
handoff schema, template, ledger, historical artifact, or protected path is in
the diff.

The complete pre-review primary path population is exactly these 14 paths;
the final-review dispatch and reviewer-result evidence are additive review
transport/output and are not self-referential members of this subject:

1. `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`
2. `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
3. `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
4. `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
5. `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260811T025748Z--HCM-4-2--causal-remediation-selector-discovery-review.json`
6. `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260811T030610Z--HCM-4-2--causal-remediation-documentation-implementation.json`
7. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/SPEC.md`
8. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/decision/20260811T025558Z--hcm-4-2-causal-remediation-selector.md`
9. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/proof/20260811T033301Z--corrective-documentation-proof-wall.md`
10. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/proof/strategy.md`
11. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/research/20260811T031843Z--corrective-owner-admission-matrix.md`
12. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/review/20260811T030408Z--causal-remediation-selector-discovery-review.md`
13. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/tasks/plan.md`
14. `docs/specs/handbook-contract-membrane/slices/HCM-4.2/tasks/todo.md`

The registered integrated outcome has consumed one CLEAN selector-discovery
cycle. The next allowed cycle is one complete-subject final discovery review;
no supplemental cycle or extension authority has been used.
