# HCM-4.2 proof strategy

**Subject:** shared typed DTO, operation-definition, schema, and immutable
bootstrap/catalog planning boundary
**Proof status:** future implementation gates; planning package only

## Proof wall

| Area | Required proof and negative cases | Authority/result |
|---|---|---|
| Source of truth | Rust DTO plus checked-in Draft 2020-12 schema are one reviewed subject; regeneration into a temporary tree is byte-identical; sorted manifest has exact refs, bytes, and fingerprints; missing/extra/hand-edited/stale assets fail. | HCM-4.2 Packet 1; no runtime implementation here |
| Serialization | UTF-8 RFC 8785/JCS bytes, one wire LF, lowercase SHA-256, exact nested preimages, stable object ordering, preserved domain array order, non-finite number/duplicate-member rejection. | HCM-4.2 Packet 0 |
| Discriminants | Explicit closed tags for every public union; unknown fields, unknown tags/enums, absent/null/default mismatch, duplicate members, invalid bounds, unsafe locators, and unbounded collections refuse. | Packet 0 |
| Compatibility | Full-SemVer schema/operation refs; supported major/minor matrix; patch byte stability; higher minor only if advertised; breaking meaning/removal requires major and exact migration ref; no range/latest. | Packets 0-2 |
| Bootstrap | Cold descriptor-pinned request; stale/tampered/checksum mismatch; unsupported/cross-major descriptor; unknown bootstrap operation; malformed response; no fabricated domain response. | Packet 3 |
| Catalogs | Immutable snapshot root; page/cursor fingerprint; contiguous ordered entries; empty/single/multi-page; concurrent change; expiry/restart; page-size overflow; gap/duplicate/reorder/count mismatch; final catalog fingerprint. | Packet 3 |
| Admission | Missing/malformed/unknown operation; missing/malformed/missing/mismatched/stale definition pin; missing/malformed/incompatible API context; exact first-failure precedence; separate preselection refusal envelope. | Packets 2-3 |
| Request/result/problem | Generic request/response closure; request/object/response/problem/details fingerprints; status/data/terminal-array exclusivity; schema manifest closure; null/default and correlation matrix; no raw secrets. | Packet 0 and operation fixtures |
| Owner/refusal | SDK maps owner validation/mutation/refusal without duplicating authority; owner errors remain typed; no public private HCM-3.6 record; capability absence is honest. | Packets 0, 2, 4 |
| Idempotency/receipts | First execution, exact replay, changed-key conflict, stale/CAS refusal, write-set cardinality/atomic group, recovery/retention/tombstone, no raw-key disclosure, no undeclared writes. | Packet 4; owner remains authoritative |
| Posture equivalence | Direct Rust and future adapters produce equivalent typed semantic data, outcomes, receipts, and fingerprints; recommendation/policy/approval/head/reassessment/exact-byte/CAS binding survives; private records remain private. | Packet 4 |
| Negative transport ownership | CLI help/examples/prose/ambient files/Tauri names do not discover capabilities or branch domain behavior; no CLI/Tauri implementation in HCM-4.2; no compiler resurrection or Phase-5 owner exposure. | Packets 2-5 |
| Repository/proof | Documentation links/paths, line endings, diff checks, selector/dispatch manifest replay, scoped and compare-to-main detection, protected paths, two self-tests, handoff/ledger parity, and local expected-old CAS. | Parent closeout |

## Proof classification

The current repository proves only the HCM-4.1 direct typed Rust precursor and
existing owner behavior. It does not prove HCM-4.2 DTO/schema/bootstrap
implementation. GitNexus query/context/impact and compare facilities are
unavailable in this session; the final record must retain that exact
unavailability and cannot call it GREEN. No native platform gate is declared
for this documentation-only slice.

## Required final promotion

The only permitted promotion is from `TargetOnly` planning authority to
`implementation-ready planning package` for the HCM-4.2 shared DTO/schema/
bootstrap boundary. No runtime seam, operation capability, transport, or phase
exit may be promoted by this package.
