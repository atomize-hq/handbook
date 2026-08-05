# HCM-3.3 selector-plan discovery remediation

## Trigger and causal boundary

Fresh discovery run `/root/hcm33_planning_discovery` returned exactly three
valid P2 planning findings for the frozen subject
`sha256:aefa8a45c8da170d939e089392d75acc63d10fb3362044f9e0b1cc4cd54f8e01`.
This parent-owned consolidated remediation stays inside the same planning
outcome, packet, authority, paths, and no-code risk ceiling. It changes only
the selector/specification/plan/task ledger and this proof record.

## Findings and remediation

| Finding | Validated contract gap | Consolidated repair |
|---|---|---|
| `HCM33-PLAN-DISC-001` | “Generic” did not require a configured custom kind to use the same engine, allowing a hard-coded first-party-only future implementation. | Require declarative exact ref/fingerprint-bound configured custom-kind selection, same-engine execution, positive replay/provenance, invalid-configuration refusal, and no first-party branch/new operation/source-order fallback. |
| `HCM33-PLAN-DISC-002` | The plan omitted resolved-profile catalog admission and exact `none` versus snapshot-only `captured_revision` currentness semantics. | Require profile-catalog membership; null/empty `none`; snapshot-selector-only family/adapter/slot captured values; request/result equality to those values; and refusal for profile-unlisted, tuple-mismatched, stale, or equal-live-but-stale inputs. |
| `HCM33-PLAN-DISC-003` | The support evaluator was named but not constrained as exact/versioned/built-in/metadata-only with canonical inputs/reason order and fingerprint closure. | Require the exact evaluator identity, canonical metadata allowlist, deterministic first unsupported reason, substitution/registry/input negatives, and semantic-drift closure in definition/evaluation/result fingerprints. |

## Post-remediation verification

- All changed planning files remain UTF-8 with no trailing whitespace and
  balanced fences.
- `git diff --check` passes.
- The repaired subject is recomputed into the closure dispatch before review.
- No production, test, fixture, schema, profile, registry, API, dependency,
  Snapshot, pipeline, consumer, tool, validator, template, or remote state is
  edited.

## Next review boundary

One different-fresh closure reviewer may inspect only these three trigger
findings, their exact repairs, the resulting complete manifest, and P1/P2
directly caused or unmasked by this remediation. It must not reopen general
discovery. No review follows a CLEAN closure.
