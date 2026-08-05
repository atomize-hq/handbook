# HCM-3.3 implementation selector discovery remediation

## Discovery evidence

The schema-valid planning discovery dispatch
`20260805T194532Z--HCM-3-3--implementation-selector-discovery` executed through
fresh built-in run `/root/hcm33_i1_selector_review` using `gpt-5.6-sol`, high
reasoning effort, and `fork_turns=none`. It returned `FINDINGS` for reviewed
subject
`sha256:8cf363314ee96355f421ea23c83fc488937442ffe409234a476f2d6506a65e0c`
and made no repository changes.

## Finding validation

### `HCM33-I1-SEL-001` — valid P2 / local remediation

The original seven-path selector did not admit a way for sibling private
`projection.rs` to validate the private resolved-profile, Resolution-stack,
dimension-rank, and currentness state of a `ContextResolutionEnvelope`. Public
envelope accessors alone cannot establish the required cross-compatibility.

The parent admits exactly `crates/engine/src/context_resolution.rs` and only one
new crate-private method on the existing envelope impl. It calls the existing
private currentness guard before returning an opaque, non-mutating Projection
authority view. GitNexus reports the exact impl edit LOW risk with zero upstream
dependants, processes, or modules. No existing method or public API changes.

### `HCM33-I1-SEL-002` — valid P2 / local remediation

The original wall did not enumerate every fixed non-envelope input or the full
currentness/evaluator matrix already required by the immutable HCM-3.3 spec.
The selector now fixes operation, surface, purpose, and complete currentness
closure across envelope cases; requires distinct/equal output accounting;
enumerates `none` and snapshot-only captured-revision cases; requires
stale/profile/stack/envelope refusal; and includes evaluator substitution,
staleness, dependency closure, forbidden input, reason precedence, and semantic
drift fingerprint behavior.

## Consolidated remediation and scope

Only the selector was semantically repaired, and this proof record was added.
No code, test, fixture, profile, schema, registry, completed handoff/dispatch,
ledger, protected path, integration ref, or remote state changed. The parent,
outcome registry, packet, causal budget, authority, and LOW risk ceiling remain
unchanged. A different fresh reviewer must close both P2 findings against the
new complete subject before implementation begins.
