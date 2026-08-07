# HCM-3.5 P2 grounded Flow packet selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P2

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized P1-P4 recovery grant,
preserving the same continuation parent and P1 causal identity.

## Selected path

`handbook_flow::adopt_grounding_outcome(GroundingOutcome) ->
GroundedPacketOutcome` is the P2 purpose-named Flow path. Its only input is the
typed engine result. A grounded result is carried into a Flow-owned packet
value through the engine-created opaque `FlowPacketGrounding`, bounded summary,
provenance, typed omissions, and non-promoting evidence. A refused engine
result is carried as a Flow-owned refused packet.

The path reports `Ready` when there are no engine omissions, `Omitted` when
the engine result has one or more typed omissions, and `Refused` only when the
engine result was refused. It does not recompute currentness, redaction,
summary, or evidence; it does not read a snapshot, delta, signal, raw change,
source directory, or `work_level`.

## Regression boundary

The retained `resolve` and `resolve_with_contract` functions are unchanged.
Local GitNexus upstream impact found `resolve` LOW with zero callers/processes
and `resolve_with_contract` LOW with one direct caller. `ResolverResult` also
has a CLI caller, so P2 makes no change to its fields or to any legacy packet
path. A refused P2 path has no resolver fallback.

P2 modifies only its new Flow module, the Flow crate root module/export list,
its focused integration type-surface test, and its selector/proof records. It
does not change Cargo/dependencies, engine, pipeline, CLI, compiler, SDK,
Substrate, schemas/configuration, gate runtime, protected checkout, or remote.
