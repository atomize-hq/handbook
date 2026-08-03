# HCM-3.2 Fresh Selector Discovery Remediation

Parent: `20260802T232751Z--HCM-3-2--context-resolution-kernel-fresh`

This is the single consolidated remediation for discovery dispatch
`20260802T233000Z--HCM-3-2--fresh-selector-review`.

| Finding | Severity | Consolidated disposition |
|---|---:|---|
| `HCM32-FRESH-SEL-DISC-001` | P1 | SPEC and selector now bind four production paths, the exact four HIGH/CRITICAL symbols plus conditional LOW method, 2,000 production lines, three test families, and the two-method lineage-store exception. |
| `HCM32-FRESH-SEL-DISC-002` | P1 | Candidate and request construction now consume typed current/proposed envelopes and reject profile, stack, unchanged-bound, fabricated, and tuple mismatches. |
| `HCM32-FRESH-SEL-DISC-003` | P2 | Semantic-memory target/result constructors now consume exact opaque `Target`/`TargetMemory` admissions and registry admission rechecks the exact subject. |

No test, RED, or Rust edit occurred. A different fresh reviewer must close all
three finding IDs against the amended complete subject before implementation.

The first closure found the original scope contradiction still present and
directly unmasked two bootstrap deadlocks (`HCM32-FRESH-SEL-CLOS-001` and
`002`). Supplemental remediation removes the stale unchanged claim, makes
failed resolution carry its sealed typed proposal candidate into request
construction, and moves `Target` admission to promotion-request construction
after the target candidate binding exists. This remains inside the same public
name, path, symbol, and risk ceilings.

The first supplemental closed every cited item except the ownership path for
the sealed failed-resolution candidate. The final bounded remediation changes
the existing error `candidate` accessor to consume the error and return the
owned candidate. It adds no public name and exposes no field or constructor.
