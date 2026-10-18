# Focused causal review

Applied `agent-code-review-and-quality`: read behavioral tests first, then check
correctness, readability, architecture, security and performance. A fresh review
subagent inspected the isolated source and harness without editing or launching
competing workloads. The author retained execution and evidence responsibility.

The review found these Required issues during development; they describe earlier
bytes, not outstanding final defects:

| Trigger | Correction / regression |
| --- | --- |
| Same-ID/same-intent concurrent deletion; loser sees absence then prepares after winner deleted target | Under-slot dedup precedes signal/import/preparation; speculative preparation failure rechecks original under lock. Deterministic request-unresolved barrier plus accepted late cancellation signal checks original result |
| Selecting a formerly accepted candidate after its snapshot retired/reclaimed | Accepted-result dedup precedes candidate liveness; explicit select retry after reclamation returns original accepted result |
| Death after atomic retirement gate but before publication-side discoverability marker | Audit and historical read use authoritative retired-attempts gate; retirement-gate-durable process exit followed by audit and reclamation |
| Second reader reused an existing pin name for a new root | Same binding is idempotent; different binding refuses. Old reader's retirement remains refused until explicit release |
| Sequential collection of worker outputs inflated early completion times | Dedicated collector per OS process captures start/end; request latency ends at initial successful completion or final retry endpoint, preserving all attempt intervals |
| Bash copy verified only its own copied manifest, then a draft fix reread selection | Capture exact path and selected manifest digest in one validated publication read; copied manifest must match that bound digest before file checks |
| Cancellation/import refusal after acquiring slot omitted hold time | Wrap all ordinary post-acquisition slot work in one timed result path, including refusals |
| Explicit historical selector named a hash-valid publication that never selected | Point, packet and pin routes require positive membership in selected lineage; publication-installed interruption creates an orphan and all three historical routes refuse it |
| A navigation edit escaped source-only view-audit and was overwritten by resume | Author reproduced the v3 gap, then v4 audit checks NAVIGATION and packet outputs too. Rebuild preserves divergent derived bytes before replacement, verifies packet source against selected manifest, and tests death immediately after preserving navigation divergence |

The reviewer found no further high-confidence authority/GC reachability defect
after these corrections. Preparation/collector lock order is bounded: collector
holds writer then tries exclusive preparation gate without waiting; slot prepares
under writer/shared gate; speculative preparation drops shared gate before taking
writer. A paused preparation remains protected. Actual COW root/page references,
including large metadata indirections, participate in marking.

The reused COW adapter still loads the full graph for commit validation and
rendering. Its point-path and changed-page savings must not be described as
constant-time end-to-end commit. Reclamation leaves directories, emitted packet
copies, publication/outcome lineage and conservative unsealed preparations. It is
a bounded explicit lifecycle, not a production collector. Noncooperating same-user
namespace replacement, OS reboot/power loss and external rollback freshness remain
outside the tested model.

Final Rust source and native binaries are frozen in `evidence/pins.json`; raw
events carry their actual executable SHA-256, including earlier development runs.
Full development fault suites and final-byte focused regression suites are kept
distinct. Final execution verification and verdict are recorded after measurements
in the evidence matrix and verification output; no early pass is substituted for
unrun later work.

The final reporting review required two additional safeguards. Native events are
now bound to the exact lane/session manifest, in addition to validating each
manifest's archived source hashes. Point medians exclude failed/censored samples;
completed commit rows require `published` plus `accepted`, retaining delivery
status separately from top-level engine success. The reviewer checked 1,059
Windows events and found no actual session mismatch. `reporting_checks.py`
exercises valid evidence, independently checks rejection of a wrong harness and
unknown session after recomputing archive checksums, and checks that failed point
samples/uncertain publication delivery cannot enter completed result tables.

The bounded post-cap continuation review additionally required exact selected
publication binding for a complete view marker and a measured additional-file
reserve. The helper now measures missing/divergent projection bytes, includes
backup and two full metadata/packet representations, rounds file allocation and
keeps a 128 MiB margin for this fixed fixture. Its source-size bound includes
the 64 KiB attachments and admitted edit suffixes. It never bootstraps an
unselected initialization or changes the original censored observation.

Closeout review also found that response parsing preceded raw-event recording:
termination during a partial final JSON response could lose the observed bytes.
After both measured schedulers exited, the controller was amended to record exact
Base64 stdout, length, hash and parse error before reporting timeout/malformed
response. Rust source and both native binaries stayed unchanged; the exact old
controller remains archived. Synthetic Rust transport probes passed on Windows
and WSL for valid JSON, partial JSON at timeout, invalid UTF-8 and empty successful
output. These are controller checks, counted separately from real-engine events.
The verifier binds the new fields to their bytes and parsed result, requires them
for current-controller sessions, and exempts only the exact archived old source.
Adversarial export checks reject wrong stdout hash/length/result and omitted
capture fields in addition to the earlier session and table-boundary mutations.
The final interpretation comparison preserves JSON types: an adversarial
`true`-versus-`1` output mutation refuses despite Python's ordinary equality rule.
After final collection, a bounded independent consistency review found no
actionable discrepancy among the reports, phase-labeled tables, retained original
censors and exported summaries. The author ran archive verification and table
shape checks; their association counts are recorded in `evidence/verification.json`
and `evidence/table-shape-check.json`, not presented as independent test cases.
