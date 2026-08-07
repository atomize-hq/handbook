# HCM-3.5-P0 discovery remediation — complete public typed-boundary closure

## Carried finding

`HCM35-P0-DISC-001` was a valid P2 from the fresh complete-subject discovery
review. The original type list named `GroundingOperationError` and the four
opaque exact-reference concepts but omitted them from the exclusive Packet 1
public-symbol ceiling. It also left request construction, engine-owned exact
reference lookup, and result extraction implicit. The first external engine
test could therefore not construct a lawful request or inspect a bounded
result without future unadmitted API expansion, raw input, or private HCM-3.4
state.

## One consolidated documentation-only repair

The Packet 0 decision, SPEC, plan, architecture, and semantic record now
freeze exactly these additions and no more:

- `ground_resolution(repo_root: &std::path::Path, GroundingRequest) ->
  Result<GroundingOutcome, GroundingOperationError>`;
- opaque public snapshot, delta, definition, and disclosure reference values
  with only `parse_exact(&str) -> Result<Self, GroundingReferenceError>`;
- the sole `GroundingRequest::new(snapshot_ref, delta_ref, definition_ref,
  disclosure_ref, envelope)` factory;
- a private `GroundingSourceResolver` that maps semantic lookup/binding
  failure to a typed refusal and has no public operation or type;
- the closed public type list, including reference, parse-error, provenance,
  evidence-state, and bounded-summary-entry values; and
- named, read-only extraction only for the grounded/refused summary/evidence
  values. No map, serializer, raw-source accessor, generic dispatcher, or
  consumer implementation is admitted.

`GroundingEvidence` is explicitly restricted to
`EvidenceAvailability::{Unavailable, False}`. It remains non-promoting and
cannot express a gate success, a score, a policy evaluation, or a true value.
The later `handbook-contracts` owner remains the only possible evaluator.

## Scope and closure requirement

This repair changes documentation only. It does not amend HCM-3.4, source
visibility, Flow/pipeline/CLI/compiler behavior, Cargo metadata, a schema,
SDK/CLI/Substrate composition, gate runtime, package/registry state, remote
state, or the protected checkout. One different-fresh, delta-focused closure
review must verify only `HCM35-P0-DISC-001` and this repair; it must not begin
another discovery cycle.
