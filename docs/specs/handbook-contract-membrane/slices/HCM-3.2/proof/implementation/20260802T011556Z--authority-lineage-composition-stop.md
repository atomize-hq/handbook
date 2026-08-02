# HCM-3.2 authority/lineage composition stop

Date: 2026-08-02

Status: authority boundary; implementation not completed

## Bound subject

- orchestration: `20260801T202515Z--HCM-3-2--context-resolution-kernel`
- integrated outcome: `hcm-3.2-context-resolution-kernel-full-slice`
- selector: `slices/HCM-3.2/decision/2026-08-01-context-resolution-kernel-selector.md`
- resumed handoff: `20260801T213000Z--HCM-3-2--orchestration--context-resolution-authority-required`
- task thread: `019fbfc9-408e-7662-b4f8-3fd0b6a5776b`
- task host: `slingshot:env_e_6a3d9cd24b5483238ba55b699a98be35`
- dispatch nonce: `adf08a6ede3648bba2566bdd54334c58`

The fresh implementation-stage selector-admission review found five P1/P2
findings. One consolidated remediation closed all five, and a different fresh
reviewer returned CLEAN. This authorized RED/Rust only inside the selector's
three production paths and named risk ceiling.

## Real-path result

A bounded proof harness exercised the reviewed Option 2 composition:

1. initialize repository identity;
2. bootstrap a committed approver registry with the existing native P-256
   authenticator path;
3. install a closed HCM-3.2 authority-binding record in a selected artifact;
4. invoke the admission resolver through the existing repository authority
   guard and `GenericArtifactLineageStoreV1::evaluate_committed_read_with`;
5. attempt all eleven exact authority uses and the negative nonce, signature,
   counter, raw/semantic/currentness, and captured-response cases.

The proof source had SHA-256
`cdd72daa8e7aa743e58f056dbcceccf1207dedcc18c15cde146162de46ac4639`
before safe-stop removal. Both real-path tests failed before binding or native
assertion admission:

```text
ContextResolutionKernelError {
  kind: AuthorityRefused,
  detail: "generic artifact recovery refused authority admission",
  candidate: None,
}
```

The two exact commands were:

```text
cargo test -p handbook-engine --test context_resolution_kernel real_repository_admits_all_exact_authority_uses_and_rechecks_currentness -- --nocapture
cargo test -p handbook-engine --test context_resolution_kernel native_assertion_refusal_signature_counter_and_fresh_nonce_replay_fail_closed -- --nocapture
```

Each ran one test and failed at the same composition boundary.

## Root cause

A committed approver registry necessarily owns
`.handbook/state/transactions/registry`. The generic artifact lineage reader
first calls `GenericArtifactLineageStoreV1::validate_inventory`, whose closed
generic-family inventory rejects that registry transaction family. Even if the
inventory admitted the directory, `require_journal_authority` enumerates the
same transaction root and would parse the registry intent as a generic artifact
intent. The required committed-registry and generic-recovery primitives
therefore cannot coexist through their live contracts.

This is not fixed by deleting retained registry transactions in a test fixture:
that would stop proving the live committed-registry/currentness contract.

## Impact and authority boundary

Local GitNexus analysis against the assigned checkout reported:

- `GenericArtifactLineageStoreV1::validate_inventory`: CRITICAL, 30 upstream
  symbols, 8 direct callers, 6 affected processes, and 3 affected modules;
- `GenericArtifactLineageStoreV1::require_journal_authority`: LOW, 9 upstream
  symbols, 1 direct caller, and 1 affected process;
- FTS extension unavailable; impact results remained exact call-graph results.

The reviewed selector authorizes only three production paths and accepts
HIGH/CRITICAL edits only for `ContextResolutionStackDefinition`,
`AuthoredStack::resolve`, and
`ContextResolutionStackDefinition::load_bytes`. A repair requires the fourth
path `crates/engine/src/artifact_lineage_store.rs` and an additional CRITICAL
edit. No such authority exists. Editing it would also invalidate the frozen
path, symbol, and risk ceilings.

## Safe-stop disposition

The unreviewed partial Rust and proof harness were removed. No product symbol,
dependency, shipped definition/profile/vocabulary byte, or protected path is
preserved as a product change. The independently CLEAN selector amendment,
its dispatch lineage, and this failing-proof record are safe to preserve.

HCM-3.2 remains incomplete with an unresolved P1 authority/composition blocker.
HCM-3.3 and later slices remain unauthorized.

## Exact resumption condition

Resume the same parent/outcome/packets/budget only after a reviewed product and
lineage-owner authority record does one of the following:

1. authorizes a bounded, fail-closed generic-lineage composition repair in
   `artifact_lineage_store.rs`, explicitly accepting the additional CRITICAL
   impact and amending the production-path/symbol ceilings; or
2. names an existing safe recovery/currentness seam that satisfies the same
   committed-registry plus generic-artifact contract without bypassing either
   owner.

The amended selector must receive fresh independent closure before RED/Rust
resumes. No packet, outcome, selector, finding, or causal budget may be renamed
or reset.
