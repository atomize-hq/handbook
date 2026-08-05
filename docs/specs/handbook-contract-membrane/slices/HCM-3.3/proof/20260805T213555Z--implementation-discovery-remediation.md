# HCM-3.3 implementation discovery consolidated remediation

## Discovery result

Fresh isolated reviewer `/root/hcm33_i1_impl_review` ran with
`gpt-5.6-sol`, `high` reasoning effort, and `fork_turns=none` against subject
`sha256:e0619dd19d146a31a28143ccd1466f7c0d9232a53eb30b35aebf431280b0b164`.
The review returned four P2 findings and no P3/P4 advisory. Focused tests were
GREEN but did not discharge these contract gaps.

The parent validates all four findings:

1. `HCM33-I1-IMPL-DISC-001`: exact currentness accepts non-snapshot selectors
   and compares only caller expectations with source-captured values. It lacks
   an independently bound live observation and therefore cannot distinguish an
   unrelated equal live revision.
2. `HCM33-I1-IMPL-DISC-002`: validated narrower target ranks are ignored during
   rule evaluation and result provenance, so a requested collapse can disclose
   at the broader authorized envelope.
3. `HCM33-I1-IMPL-DISC-003`: the authored profile/configuration bytes that own
   catalog, purpose, surface, vocabulary, and configured-kind selection are
   validated but not bound into request/result identity.
4. `HCM33-I1-IMPL-DISC-004`: semantic dependency pairs are checked for exact-ref
   syntax and equality but are not resolved from trusted local definitions or
   recomputed, allowing substituted fingerprints after enclosing fingerprints
   are repaired.

## One consolidated remediation

The remediation remains inside the eight admitted implementation paths:

- restrict exact currentness to snapshot selectors and introduce one private,
  deterministic live-observation closure whose family/selector/adapter/slot
  identities and revisions are independently compared before payload access;
- use the validated requested target ranks as the effective evaluation and
  result-provenance boundary for non-widening collapse;
- bind the byte-fingerprinted authored profile/configuration pair separately
  from the underlying Resolution-profile pair in requests, results, and result
  fingerprints;
- add bounded trusted private semantic definitions/fingerprint recomputation
  for every selected schema, capability, target-schema, derivation, matcher,
  classification, pointer, policy-registry, and evaluator dependency family;
- add the exact missing negative/positive tests named by all four findings,
  including no payload reads on refusal and deterministic replay.

No public API, Cargo/dependency/default catalog, external registry, remote
lookup, executable extension, transport rule, source mutation, consumer,
Snapshot Memory, or later-slice surface is admitted.

## Impact boundary

After the local index learned the new module, upstream impact was rerun before
remediation. `execute_projection` is `MEDIUM` with eight direct private-test
callers and one affected test process. `validate_currentness_requirements`,
`validate_request_currentness`, `validate_request`, and `validate_pair` are
`LOW`; the `ProjectionConfiguration` impl/struct candidates are each `LOW`.
No target is `HIGH` or `CRITICAL`. The staged complete delta remains `HIGH`
because it contains 480 indexed symbol identities and 14 cross-community
flows; closure must preserve and reassess that exact aggregate without
understatement.

The four findings consume the implementation-stage discovery. This document
authorizes one consolidated remediation only; a different-fresh delta-focused
closure must close all four triggers before implementation review can end.
