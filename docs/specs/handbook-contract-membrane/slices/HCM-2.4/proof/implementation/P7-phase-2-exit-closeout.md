# P7 Phase 2 exit and HCM-2.4 closeout proof

Date: 2026-07-31

## Authority and reviewed baseline

The operator explicitly selected P7 from exact completed handoff
`20260731T005606Z--HCM-2-4--orchestration--p6-final-review-completed`.
The live baseline was branch `feat/handbook-contract-membrane` at
`4aecd1b42356592ccd4d093c85486a9c25410dca`; primary P6 commit
`1209430240eb67c7aa2baac5859d84cb9a84a3b2`, its closeout commit, and CLEAN
dispatch
`20260731T004500Z--HCM-2-4--p6-final-review-control-truth-supplemental-2-closure`
were present and validated. Ledger parity was exact.

P7 changed no Rust, test, fixture, definition, schema, profile, Cargo,
dependency, public API, package, or runtime byte. The pre-existing modified
GitNexus skills, `AGENTS.md`, `CLAUDE.md`, and untracked
`docs/ideas/intent-to-outcome-fidelity.md` stayed user-owned and excluded.

## Exact Phase 2 exit mapping

| Exit obligation | Positive P7 evidence | Result |
|---|---|---|
| One editable canonical truth per targeted artifact | Project Context, Project Authority, Environment Context, Work Specification, Decision Record, and Risk Record canonical/view tests replayed. Legacy Project Context, Environment Inventory, Feature Spec, Decision, and Risk Markdown mutations or decoys have zero canonical influence. | GREEN |
| Intakes converge on kind-selected schema and expose missing coverage | The five P1A intake definitions, complete typed coverage vectors, duplicate/unknown/missing-coverage refusals, and guided/express/agent-assisted equality replayed while the released Charter intake remained unchanged. | GREEN |
| Charter boundary remains auditable and non-competing | Direct released 1.1 and selected compatible 1.2 paths produce the same released Charter identity and bytes. Every Project Authority descriptor mutation, second-read mutation, atomicity, recovery, replay, lifecycle, approval, promotion, transaction, and immutable-record negative replayed. The P6 common loader retained committed authority. | GREEN |
| Custom kind works without enum or generated command | The HCM-2.3 registry-brief definition, real repository selection, generic operation, actual-binary mutation/read/validation, replay, recovery, concurrency, and lineage targets replayed without a product kind variant or generated command. | GREEN |
| Derived views are fixed deterministic first-party renderers only | The six fixed renderer rows and their full-byte goldens replayed with separate source/render fingerprints. Renderer definitions carry no Resolution input and no generic configured renderer executes. | GREEN |
| Generic custom-kind Projection/Resolution stays deferred | No Projection or Resolution artifact-kind definition exists; no P7 runtime surface was added; HCM-3.2/HCM-3.3 authority remains unchanged and unstarted. | GREEN |
| No migration or dual-read promise | Legacy Markdown mutation/deletion and decoy tests replayed with no selected-output effect. No fallback read, migration warning, alias, importer, or compatibility promise is present. | GREEN |
| Every Phase 2 bridge is deleted | Recursive Rust scan found zero occurrences across both bridge IDs, four bridge types, `rendered_projection_for_path`, three fixed-sibling loaders, the fixed order constant, and both path-switch names. Descriptor-selected packet behavior, budgets, fixtures, blockers, refusals, order, provenance, and fingerprints replayed. | GREEN |

Rejected P2S/P2A condition-evidence and native-adapter material remains historical
only: the active runtime has no evaluator, evidence-platform, task-gate, or
rejected-proof influence.

## Proof wall

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| P1A definition/schema/kind/intake/renderer/profile wall, including engine lib and HCM-2.3 registration preservation | PASS |
| P1B direct 1.1 / exact 1.2 compatibility plus every HCM-2.2 engine, compiler, and CLI preservation target | PASS |
| P1C native and exact Unix doctor/CLI/substitution-ABA targets | PASS; all three Unix-selected tests GREEN |
| P2 Environment Context, readiness, flow, budget, mutation, legacy non-influence, and rejected-machinery scans | PASS |
| P3/P3B pipeline capture/compile/handoff, CLI refusal, fixture-contract, provenance, and real Stage 10 paths | PASS |
| P4 Decision Record complete target and all five named negative surface/path tests | PASS; 10/10 |
| P5 Risk Record complete target | PASS; 13/13 |
| P6 descriptor-selected flow, retained bytes, non-fixed identity, JSON label projection, provenance, and pre-mutation manifest identity | PASS |
| HCM-2.3 generic lineage / real-binary recovery and concurrency | PASS; 54/54 |
| `cargo test --workspace --all-features -j1` | PASS; exit 0 in 891.9 seconds, including workspace doctests |
| locked feature tree | PASS; 333 lines, no Cargo or dependency delta |
| `cargo check --workspace --all-targets --all-features --locked` | PASS |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --all-features --no-deps --locked` | PASS |
| `cargo package -p handbook-engine --allow-dirty --no-verify --locked` | PASS; 225 archive members, 80/80 definition files byte-identical, archive SHA-256 `8cfbddb7b57be95fad688e0c45b738ebe1d7982c90977c4402acccdcb2dcd5a2` |
| package manifest equality | PASS; zero `Cargo.toml`/`Cargo.lock` delta from the P6 closeout and zero packaged-definition mismatch |
| archive boundary normal check and negative self-test | PASS |
| handoff validation, historical-v1 admission self-test, and orchestration/causal-contract self-test | PASS; 72 records, 432 current dispatches, 8 admitted legacy dispatches, 72 ledger entries |
| retired bridge/fixed-selector recursive scan | PASS; 12 exact terms, zero Rust hits |
| fresh GitNexus analysis and staged `detect-changes` | PASS; index current at `4aecd1b`; exact staged path inspection contains only authorized documents and immutable dispatches; identical 1.6.9 replays reported 10, 12, and 14 documentation symbols, so that cardinality is diagnostic rather than an exact gate; every replay reports 0 affected processes and LOW risk |

A diagnostic `cargo package --workspace --allow-dirty --no-verify --locked`
attempt packaged `handbook-engine` and then stopped when Cargo tried to resolve
the unreleased local `handbook-engine = 0.2.0` dependency for
`handbook-flow` from crates.io, where only `0.1.1` is published. This is the
repository's unchanged publication-boundary limitation, not its package gate.
The established engine package replay above is GREEN and the package/Cargo
manifest equality proves P7 did not alter that boundary.

The repository-required compare-to-`main` GitNexus command was run with both the
current CLI and `1.6.9`. On this Windows host both processes terminate after the
known unavailable-FTS local-backend warning before returning a comparison. This
diagnostic is not called GREEN. The exact staged subject succeeds under the same
fresh index with the LOW/zero-process result above, and direct Git scope proof
confirms no production symbol or flow is in P7.

## WSL baseline re-evaluation

Fresh Ubuntu 24.04 execution of the whole `handbook-cli` `cli_surface` target
returned 100/101. The sole failure is
`pipeline_state_set_field_rejects_invalid_paths_and_values`: Unix correctly
reports that `/tmp/CHARTER.md` `must not be absolute`, while the Windows-authored
exact-string assertion expects `must be a clean repo-relative path`. The CLI
refuses with no mutation in both cases, the compiler and pipeline negative tests
expect and receive the Unix wording, and Git history places the exact CLI
assertion in the earlier HCM-2.2 checkpoint. All three P1C Unix-selected-profile
tests are GREEN.

Therefore the failure is not relabeled as a passing P7 target and is not a Phase
2 waiver: it is a cross-platform exact-message test advisory outside every
Phase 2 exit row and outside P7's no-test-edit ceiling. It is registered as
non-blocking review debt in `09-review-finding-inventory.md`.

## Review and stop boundary

Immutable v1.4 discovery dispatch
`20260731T022800Z--HCM-2-4--p7-phase-2-exit-final-review` returned FINDINGS with
P2 `HCM-P7-FR-0001` (contradictory P7 selection and stale landing text) and P2
`HCM-P7-FR-0002` (an incorrect staged GitNexus symbol count). The parent
consolidated both findings and changed only this authorized documentation/proof
subject. Different-fresh closure dispatch
`20260731T024500Z--HCM-2-4--p7-phase-2-exit-remediation-closure` closed
`HCM-P7-FR-0001` but returned FINDINGS because repeated GitNexus runs on the
unchanged stage reported 10, 12, and 14 documentation symbols. The parent
therefore removed symbol cardinality as an exact gate while retaining exact
path inspection, zero affected processes, and LOW risk as the reproducible
change-detection proof. P7, HCM-2.4, and Phase 2 exit remain unaccepted until
supplemental-fresh closure dispatch
`20260731T025730Z--HCM-2-4--p7-phase-2-exit-gitnexus-supplemental-closure`
returned FINDINGS. It confirmed the GitNexus proof correction and found no new
runtime or Phase 2 proof gap, but P2 `HCM-P7-FR-0003` proved the old parent's
reused finding ownership cannot produce a truthful completed v1.4 handoff.

The operator authorized the lineage repair recorded in
`../../decision/20260731-p7-final-review-lineage-repair.md`: all three old
dispatches remain immutable non-accepting context, and a distinct parent,
registry, and causal budget own a new complete-subject review. No prior finding
ID may be newly owned. P7, HCM-2.4, and Phase 2 exit remain unaccepted until
`20260731T034000Z--HCM-2-4--p7-final-review-restart-discovery` returns CLEAN on
the new exact fingerprint.

CLEAN authorizes only the local primary commit and separate parent-owned
handoff/ledger closeout. It does not authorize HCM-3.x, release, push, or
automatic continuation.
