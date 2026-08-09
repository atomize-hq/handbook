# Phase-3 strict-Clippy corrective selector

**Status:** pending fresh discovery review. This selector is not admitted for
implementation until its schema-valid v1.4 discovery dispatch is `CLEAN`.

**Parent orchestration:** `handbook-hcm-3-6-strict-clippy-corrective-20260809`

**Selected outcome:** `hcm-3-6-phase-3-strict-clippy-corrective`

## Authority and starting boundary

This selector consumes the immutable authority-boundary handoff
[`20260809T152000Z--HCM-3-6--orchestration--phase-3-exit-strict-lint-authority-boundary`](../../handoffs/records/20260809T152000Z--HCM-3-6--orchestration--phase-3-exit-strict-lint-authority-boundary.json)
as scoped context only. It neither resumes nor alters that parent, the completed
HCM-3.6 implementation parent, their dispatch populations, ledger history,
admission hashes, or validator semantics.

The operator-authorized increment starts at
`b1edd4d8e045ce64d20a77505ba3da87fe996704`, tree
`3ba93283c77dc653c9b7533954ee8b141cbdc820`, with required ancestor and local
`origin/feat/handbook-contract-membrane` observation
`1256e724a2b7da6b6250f57d6f63fced1e2cf949`. Publication is local-only through
`refs/heads/orchestration/handbook-phase-3-clippy-corrective-20260809` by an
expected-old compare-and-swap from that exact base. The protected checkout
`C:\\Users\\spmcc\\Documents\\__Project_Code\\handbook` is read-only and must
remain byte-identical, clean, unstaged, and uncommitted.

The permanent governance decision
[`20260809T150000Z--permanent-ordinary-validator-governance.md`](20260809T150000Z--permanent-ordinary-validator-governance.md)
governs only the exact historical raw failure
`20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation writes or advances before selector CLEAN`.
It keeps that ordinary-validator result failed/not GREEN and waives no current
strict-lint, P1/P2, proof, review, scope, or protected-path defect.

## Exact corrective subject

The only admitted product paths are:

1. `crates/engine/src/charter_authority_transaction.rs`
2. `crates/engine/src/charter_lifecycle_transition_v11.rs`
3. `crates/engine/src/charter_posture_transaction_intent_v1.rs`
4. `crates/engine/src/project_posture.rs`
5. `crates/engine/src/snapshot_memory/policy.rs`
6. `crates/engine/src/snapshot_memory/record.rs`
7. `crates/engine/src/snapshot_memory/redaction.rs`
8. `crates/engine/src/snapshot_memory/tests.rs`
9. `crates/engine/src/grounding.rs`

The parent may add only directly required current-v1.4 internal dispatches,
proof/control-pack records beneath `docs/specs/handbook-contract-membrane`, one
parent handoff, and the rebuilt ledger. It may not alter public APIs, schemas,
dependencies, Cargo/version policy, runtime configuration, unsafe policy,
generated assets, immutable evidence, the permanent decision, or any other
product path.

## Diagnosis and permitted remediation

At the bound base, the exact command
`cargo clippy --workspace --all-targets --all-features -- -D warnings` fails
with the contract-bound strict diagnostics: unreachable private HCM-3.6 posture
helpers, one needless lifetime, pointer/reference and Option simplifications,
a redundant closure, an unnecessary sort closure, a large private grounding
outcome representation, and two complex private grounding signatures.

The correction must make the diagnosed code genuinely lint-clean without any
lint suppression attribute, lint-policy/configuration change, disabled test,
or waiver. Every changed existing Rust item requires fresh upstream GitNexus
impact evidence before its edit. A required source/test path outside this list,
public or schema surface change, dependency, configuration, semantic policy
decision, unexpected HIGH/CRITICAL expansion without independent in-scope
review, or any additional validator failure is an authority stop.

## Required proof and review

Before a reviewer receives this selector, replay its complete manifest and
convergence wall and validate the immutable v1.4 discovery dispatch. The
discovery reviewer independently checks the exact lint subject and selector.
The parent consolidates any P1/P2, performs only one in-scope remediation, and
uses a different fresh delta reviewer after material remediation. No discovery
or supplemental cycle follows a `CLEAN` result.

Completion requires `cargo fmt --all -- --check`, the exact strict Clippy
command, targeted owner tests, `cargo test -p handbook-engine --all-features`,
`cargo test --workspace --all-targets --all-features`, applicable negative and
regression tests, ordinary handoff validation recorded as failed/not GREEN only
for the exact permanent historical condition, both required self-tests, scoped
and compare-to-main GitNexus detection, protected-checkout verification, a
reviewed primary commit, and a separate mechanical v1.4 handoff/ledger closeout
commit. This corrective outcome makes no Phase-3 exit claim.
