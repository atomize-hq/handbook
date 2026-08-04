# HCM-3.2 JCS/recovery planning discovery remediation

Parent: `20260804T124829Z--HCM-3-2--context-resolution-jcs-recovery`

Discovery dispatch:
`20260804T125609Z--HCM-3-2--jcs-recovery-planning-discovery`

Discovery subject:
`sha256:0af7442bd1d1765c080165c88be407cc45194ad965594f637bba733b5b208ef8`

This is the one consolidated planning remediation. No test, RED, Rust edit,
implementation check, stage transition, commit, handoff, ledger update, or
publication occurred.

| Finding | Priority | Consolidated disposition |
|---|---:|---|
| `HCM32-JCS-PLN-DISC-001` | P2 | Generic domains 1-3 now preserve the exact existing canonical-JSON/canonical-YAML SHA-256 algorithms. Only HCM domains 4-6 use literal NUL-terminated ASCII tags plus unsigned 64-bit big-endian byte lengths. Exact byte/newline treatment and required golden/equality/substitution vectors are frozen. |
| `HCM32-JCS-PLN-DISC-002` | P2 | The wrapper artifact, decoded declaration carrier, decoded payload carrier, ordinary strings, aggregate decoded escapes, member count, and depth each have an explicit measurement layer. The two carrier strings are the only exceptions to the 8 KiB ordinary-string limit, with limit boundary and jointly satisfiable vectors required. |
| `HCM32-JCS-PLN-DISC-003` | P2 | `publisher_authority` is now an exact closed nine-member object with ref, fingerprint, RFC 4648 padded base64, decoded-length, and total-size rules. Its exact existing eleven-member registry challenge, operation ID, active administrator/publisher credential, one added publication mapping, full prior-state preservation, assertion/response replay, cardinality, and negative proof are frozen. The first closure kept this finding partially open because it collapsed the live retained response chain; the bounded causal repair now freezes transition authorization pair -> assertion -> decoded-response pair -> strict raw-response byte equality. |
| `HCM32-JCS-PLN-DISC-004` | P2 | The H2-to-T2 no-journal crash uses exact registry-head-driven discovery. A semantically gated non-authoritative candidate is committed before H2 disables the predecessor; H2 durably carries the exact outer/witness; the retry key is `hcm32crpub_<outer64hex>`; lock order is promotion/registry then generic; candidate replay/cardinality is exact; and a closed transition-keyed quarantine schema, phases, reasons, limits, retry resolution, cold-start, concurrency, and non-HCM coexistence behavior are frozen. |

The amended complete subject remains inside the same parent, outcome, packet,
five-production-path ceiling, exact public API ceiling, named risk set, and
causal budget. A different fresh read-only closure reviewer must close all four
finding IDs before planning can become CLEAN or any test/Rust work can start.
