# HCM-3.5 resolution-aware adoption — authority-admission proof

**Status:** read-only admission evidence; CLEAN admission is consumed only for
the bounded documentation continuation. This is a slice-local proof note, not
a parent handoff, ledger entry, or global closeout record.

## Admission chain

| Check | Evidence | Result |
|---|---|---|
| Direct predecessor | `20260806T204100Z--HCM-3-5--orchestration--resolution-aware-adoption-planning-authority-boundary` | Direct non-completed authority-boundary predecessor retained. |
| Continuation grant | `authority/20260806-hcm-3-5-resolution-aware-adoption-planning-authority.json` | Same parent/outcome/packet/budget documentation continuation grant. |
| Different-parent attestation | Grant-bound completed attestation recorded by the authority-admission dispatch. | Prerequisite evidence only; it does not replace the direct predecessor. |
| Read-only selector | `handoffs/dispatches/20260807T000100Z--HCM-3-5--resolution-aware-adoption-planning-authority-admission-discovery.json` | CLEAN, as supplied by the parent orchestration. |
| Reserved continuation slot | `handoffs/dispatches/20260807T000200Z--HCM-3-5--resolution-aware-adoption-planning-documentation-implementation.json` | Documentation-only implementation-stage slot admitted. |

## Scope result

The admission permits only the direct HCM-3.5 documentation plan. It does not
reset the original planning cadence, select product behavior, or grant code,
runtime, public API, schema, configuration, dependency, SDK/CLI/Substrate,
consumer, publication, registry, remote, staging, commit, or ref authority.

The implementation child uses the narrower parent assignment ceiling: `00`
through `06`, HCM-3.5 `SPEC.md`, HCM-3.5 plan/todo, and the three reserved
slice-local continuation/proof markdown records. It leaves `07`/`08`/`09`, all
dispatches, records, ledger, original selector/preflight, and historical
authority bytes untouched.

## Availability and disposition

GitNexus MCP/CLI/index is unavailable. No code symbol is edited, so no impact
analysis is claimed or marked GREEN. The required next action is independent
complete-subject documentation review by the parent orchestration; this proof
does not create a handoff or declare completion.
