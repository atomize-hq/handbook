# HCM-3.5 P2 grounded Flow packet — source and impact record

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P2

**Status:** CLEAN

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant,
bound to task `019fdcd7-31ae-7881-8502-1b173d751c93`, host `local`, and its
recorded nonce.

## Complete-subject discovery

The local-only GitNexus query was attempted first; its FTS-disabled index
returned no keyword rows, so exact symbol lookup was used. The retained Flow
seams were reviewed before P2:

| Symbol | Direct callers | Processes | Risk | P2 disposition |
| --- | ---: | ---: | --- | --- |
| `handbook_flow::resolve` | 0 | 0 | LOW | Not edited. |
| `handbook_flow::resolve_with_contract` | 1 | 0 | LOW | Not edited. |
| `resolver` crate-root module | 0 | 0 | LOW | Not edited; GitNexus changed-scope attribution is container-only. |
| `ResolverResult` | CLI rendering and `resolve_with_contract` | — | Read-only context | Not edited. |

The existing resolver remains a separate legacy packet path. P2 adds neither a
field to `ResolveRequest`/`ResolverResult` nor a call to `resolve` or
`resolve_with_contract`; therefore the pre-existing CLI caller stays outside
the new typed path. No HIGH or CRITICAL symbol is edited.

## One remediation

The one P2 remediation adds `grounded_packet` and the purpose-named
`adopt_grounding_outcome` operation. Its input is solely the typed
`GroundingOutcome` owned by `handbook-engine`. The exhaustive two-variant match
creates a Flow-owned packet outcome as follows:

| Engine result | Flow packet state | Flow action |
| --- | --- | --- |
| Grounded; no typed omissions | `Ready` | Carry the opaque engine Flow packet grounding, bounded summary, provenance, and evidence. |
| Grounded; one or more typed omissions | `Omitted` | Carry the same engine-created values and omissions without recomputation. |
| Refused | `Refused` | Carry the typed engine refusal; do not call a legacy or raw-data path. |

The module has no filesystem, repository, delta, snapshot, signal, currentness,
redaction, `work_level`, JSON, or gate dependency. It cannot create a promotion
claim because it preserves the engine's non-promoting evidence unchanged.

## Closure subject

```text
crates/flow/src/lib.rs
crates/flow/src/grounded_packet.rs
crates/flow/tests/hcm_3_5_grounded_packet.rs
docs/specs/handbook-contract-membrane/slices/HCM-3.5/decision/20260807T163000Z--p2-grounded-flow-packet-selector.md
docs/specs/handbook-contract-membrane/slices/HCM-3.5/proof/20260807T164000Z--p2-grounded-flow-packet-*.md
```

The generated `AGENTS.md` and `CLAUDE.md` refreshes remain unstaged,
uncommitted, undiscarded, and excluded. The staged local GitNexus
`detect-changes` result reported six files, one container symbol (`resolver`),
zero affected processes, and LOW risk. Exact module impact was also LOW with
zero callers/processes. P2 is CLEAN; P3 has not begun.
