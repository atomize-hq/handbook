# HCM-3.2 Context Resolution Kernel Selector

Status: authority-required; independently discovered planning P1/P2 block implementation
Selected slice: HCM-3.2 only
Parent orchestration: `20260801T202515Z--HCM-3-2--context-resolution-kernel`

## Selection and predecessor boundary

The user explicitly selected the complete HCM-3.2 slice. HCM-3.1 is completed
predecessor context and its vocabulary/profile closure is immutable. This
selector does not select HCM-3.3 or any later work.

## Orchestration registry

The parent has one integrated outcome and three packets:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/2026-08-01-context-resolution-kernel-selector.md","integrated_outcome_id":"hcm-3.2-context-resolution-kernel-full-slice","packet_ids":["HCM-3.2-P1-planning-selector","HCM-3.2-P2-kernel-implementation","HCM-3.2-P3-proof-control-closeout"]}]
```

The canonical JSON plus one terminal LF is the frozen outcome registry preimage.
Every v1.4 dispatch binds its recomputed registry fingerprint and the causal
budget derived from this parent orchestration plus the integrated outcome.

## Selected owner and call boundary

`handbook-engine` is the sole runtime owner. The new kernel consumes the exact
`ResolvedInstanceProfile::context_resolution()` selected by the existing
profile path and the profile's exact ref/fingerprint. It produces immutable
engine values only. No flow, pipeline, compiler, CLI, SDK, Tauri, Substrate,
HTTP, dock, or publication consumer is added.

This proposed call boundary is not yet admissible. No selected trusted owner
resolves the authority-bearing refs/fingerprints in envelope, escalation, or
promotion records to authenticated bytes and principals. The selector cannot
substitute caller coherence for admission authority.

## Definition identity selector

Preserve byte-for-byte and do not version:

- shipped stack ref/fingerprint
  `handbook.context-resolution.shipped-root@1.0.0` /
  `sha256:9e95fdef90b98e28acb60bfe96a72f122418b56b1531eafe8ab6cff3eb7668b4`;
- matcher ref/fingerprint `handbook.mutation-matcher.core@1.0.0` /
  `sha256:be585006043ca46096e85ad76f20a87439e6984bcace997924982f0247aa29f4`;
- escalation ref/fingerprint `handbook.resolution-escalation.core@1.0.0` /
  `sha256:1f7e04ed7d8a68f338b2c421db8ef0c49f16d9b4dcb001145083414efa408121`;
- memory-promotion ref/fingerprint `handbook.memory-promotion.core@1.0.0` /
  `sha256:1f0e07938432a159d2ba9c131ed6dc28536ec8aa30b817f24066e246c06c0bca`.

General stack definitions use the same schema/policies and uniform exact
identity algorithm. They are test/repository definitions, not new shipped
defaults. Same ref with different bytes, stale dependency fingerprints,
range/latest lookup, remote loading, and executable hooks refuse.

## Ordered-stack and six-dimension selector

The list is broad-to-narrow, non-empty, and linear. Stable level IDs are unique;
display labels are presentation only and may repeat. Exactly six non-empty
domains exist. Each has unique value IDs and contiguous ranks from zero, where
zero grants least reach/disclosure/durability/claim authority. Every level has
all six defaults, and each adjacent narrower level may preserve or lower each
rank but never raise one.

The active level selects a named default position only. It is not an authority
score. Envelope dimensions remain complete and independently comparable, so
two envelopes at one level may differ in any subset of dimensions.

## Inheritance selector

Root envelopes have no parent. Children cite exactly one supplied parent by
ref/fingerprint and repeat complete effective state. Parent/profile/stack
identity is equality-checked. Every child rank must be less than or equal to
the corresponding parent rank. Any increase returns no envelope and one typed
`dimension_rank_increase` escalation candidate naming the changed dimensions.

Candidate construction remains blocked until exact reviewed authority maps
each candidate class to an admitted trigger record, required typed missing
condition, requested-authority derivation, and evidence requirements. The core
policy's trigger-class strings alone are not exact trigger definitions.

Constraint inputs are exact sorted ref/fingerprint pairs and cannot become
merge parents. Lower-horizon observations cannot mutate higher authority.

## Mutation selector

Only `repository_path` is admitted. The exact core grammar and byte/segment
limits are enforced before evaluation. Matching is case-sensitive. Root grants
are allow-minus-deny; child grants are parent-intersection-child-minus-all-deny.
Empty allows deny all. Deny always wins a valid overlap.

Containment, if later authorized, compares the positive parent allow language;
inherited denies are accumulated separately and always subtract from the
result. A deny hole does not make an otherwise equal child allow an expansion,
and a child can never remove or weaken that hole. Child allow containment is
proved under the closed grammar. Exact equality,
literal specialization of a parent segment wildcard, and a child path/pattern
beneath a matching parent terminal `/**` prefix are admitted. A child terminal
recursive selector is admitted only beneath an equal or broader parent terminal
recursive selector. Any ambiguous pattern-language containment refuses as
indeterminate; it never guesses or grants. Possible expansion returns no child
and a typed `mutation_allow_expansion` candidate.

Malformed/unresolvable selector or target, unknown target kind, stale matcher,
or evaluation indeterminacy refuses. The matcher reads no filesystem state.

## Memory, validation, and escalation selector

Memory/validation compare requested and envelope ranks in their own domains.
At-or-below is authorized. Higher memory returns promotion-required; higher
validation returns not-authorized. Neither result mutates state or creates
green proof.

Escalation and promotion requests/dispositions are separate immutable records.
The registry admits one terminal disposition per exact request and rejects
duplicate, stale, changed-ID, invalid-outcome, or forbidden-authority records.
Requests have no preapproval effect. Approved escalation cites an exact
replacement envelope; applied promotion cites an exact new semantic-memory
result after compare-and-write validation. Other outcomes cite no result.

The registry is a pure semantic validator only. Durable storage, operation
catalogs, restart discovery, and adapter delivery are not selected.

That pure-validator boundary is insufficient for terminal authority. The live
contracts do not provide an admitted authority record/resolver for root
creation, requested/approving authority, decision/evidence pairs, or target
memory ownership. A caller could fabricate coherent pairs. No constructor or
registry may be implemented until reviewed resumption authority closes this
gap without silently adding a generic framework.

## Compatibility selector

HCM-3.1 stable-role/vocabulary/profile bytes and behavior remain exact. The
shipped empty vocabulary and profile's selected stack closure replay unchanged.

L0-L3 pipeline input/filtering is not a public Context Resolution mapping and
is not edited. Existing positive/negative scoped-filtering tests must pass.
This deliberately preserves the useful precursor until HCM-3.5 selects its
real migration; HCM-3.2 neither removes it nor freezes it into kernel semantics.

## Exact public symbols

The only new public engine names are the 15 names listed in `SPEC.md` under
`Public Rust surface`. Existing `ContextResolutionStackDefinition` and
`ResolvedInstanceProfile` remain compatible. Any additional public type,
method that bypasses validation, trait implementation with observable new
semantics, or exported untyped value requires selector amendment and fresh
impact/review before editing.

The 15 names do not freeze callable signatures or a changed-symbol counting
rule. They are therefore not an implementation allowance. Reviewed resumption
must enumerate every public constructor, evaluator, registry method, and
accessor signature, reconcile the symbol ceiling, and add ordinary-use,
misuse, and exported-surface proof.

## Exact ceilings

Production paths:

- `crates/engine/src/context_resolution_registry.rs`;
- `crates/engine/src/context_resolution.rs`;
- `crates/engine/src/lib.rs`.

Test/fixture paths:

- `crates/engine/tests/context_resolution_stack.rs`;
- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

Control paths are the exact HCM-3.2 slice files, review/proof dispatches and
records, affected HCM 00-06 rows after earned proof, one v1.4 handoff, ledger,
and a new `09` row only when required by a validated P3/P4.

Maximums proposed before discovery were: 3 production paths, 3 test families,
20 changed production symbols, 1,000 hand-written production lines, one
LOW/MEDIUM engine subsystem, no
ancillary allowance, and no Cargo/dependency/version/unsafe/schema/transport
surface. The external protected product checkout, meta checkout, and control
repository are not accessed or mutated. Frozen historical dispatch/record
corpora and every path outside these ceilings remain unstaged/uncommitted.

GitNexus reports HIGH impact for `ContextResolutionStackDefinition` and
CRITICAL impact for both `AuthoredStack::resolve` and
`ContextResolutionStackDefinition::load_bytes`. Generalizing configurable
stacks therefore contradicts the proposed risk ceiling. This selector does not
authorize those edits; resumption must name the exact CRITICAL symbols, impact
processes, proof wall, and accepted risk or choose an implementation that does
not edit them.

## Test selector

RED must fail because configurable stack application and envelopes do not yet
exist. GREEN must prove every SPEC proof obligation, including exact shipped
replay, non-shipped configurability, six independent dimensions, deterministic
fingerprints, child narrowing, deny overlap, indeterminate refusal, typed
memory/validation outcomes, no-effect escalation, transition cardinality, and
unchanged HCM-3.1/L0-L3 behavior.

No test may claim Projection, Snapshot, flow/pipeline adoption, posture, SDK,
transport, release, publication, or downstream proof.

## Review and proof selector

Planning must be independently CLEAN before Rust edits. Implementation review
uses the complete converged subject and exact proof wall. P1/P2 findings receive
one consolidated remediation plus different-fresh closure; only directly
causal unmasked findings may consume the two supplemental cycles. Proof and
final-closeout stages use fresh agents, and no cycle follows CLEAN.

Before the primary commit: focused/full tests, strict Clippy/format/diff,
validator/self-tests, exact definition replay, path ceiling, GitNexus scoped
and compare-to-main detection, remote baseline, and protected-root
non-access/non-mutation all have honest results. An unavailable FTS or
comparison is unavailable, not GREEN.

## Non-goals

- HCM-3.3 Projection or any Resolution-aware view;
- HCM-3.4 Snapshot Memory/delta;
- HCM-3.5 packet/flow/pipeline adoption or L0-L3 migration;
- HCM-3.6 posture;
- SDK, CLI/Tauri/Substrate/HTTP/dock/transport work;
- new dependency, crate, module framework, unsafe/native/platform machinery;
- shipped-definition/profile/vocabulary mutation;
- release, publication, push, or remote reconciliation;
- unrelated cleanup or immutable-history repair.

## Stop conditions

Stop for any required scope beyond the ceilings, new public/dependency/schema/
unsafe/transport authority, HCM-3.3+ behavior, changed shipped identity,
unresolved HIGH/CRITICAL impact, contract contradiction not repairable inside
HCM-3.2, mandatory delegation failure, exhausted causal budget, or unresolved
P1/P2. Local findings, proof gaps, and bounded remediation remain parent-owned.

The authority-resolution, candidate-mapping, exact-public-signature, and
CRITICAL-impact conditions are met. The same slice is stopped before RED/Rust.
Resume only from an exact reviewed authority ref that chooses the bounded
admission model and risk posture; do not infer authority from this planning
artifact or from the terminal receipt.
