# HCM-4.2 proof strategy

**Subject:** shared typed DTO, operation-definition, schema, and immutable
bootstrap/catalog planning boundary
**Proof status:** corrective planning primary; additive review/closeout
evidence alone records completion status; no runtime implementation

## Proof wall

| Area | Required proof and negative cases | Authority/result |
|---|---|---|
| Source of truth | Rust DTO plus checked-in Draft 2020-12 schema are one reviewed subject; regeneration into a temporary tree is byte-identical; sorted manifest has exact refs, bytes, and fingerprints; missing/extra/hand-edited/stale assets fail. | HCM-4.2 Packet 1; no runtime implementation here |
| Serialization and framing | LF-free UTF-8 RFC 8785/JCS instance/fingerprint bytes; checked-in schema canonical JSON plus exactly one LF with that LF in its schema fingerprint; CLI zero-or-one adapter LF; Tauri-owned framing; lowercase SHA-256, exact nested preimages, stable object ordering, preserved domain array order, non-finite number/duplicate-member rejection. | HCM-4.2 Packets 0-1; no universal wire LF |
| Discriminants | Explicit closed tags for every public union; unknown fields, unknown tags/enums, absent/null/default mismatch, duplicate members, invalid bounds, unsafe locators, and unbounded collections refuse. | Packet 0 |
| Compatibility | Full-SemVer schema/operation refs; supported major/minor matrix; patch byte stability; higher minor only if advertised; breaking meaning/removal requires major and exact migration ref; no range/latest. | Packets 0-2 |
| Bootstrap | Exact closed request/response/refusal/descriptor/OwnerVersionEntry/CapabilityEntry field sets, tags, null/default rules, bounds, distinct descriptor-content/schema identities, complete fingerprint preimages, declared/current transports; cold/stale/tampered/checksum mismatch; unsupported/cross-major descriptor; unknown bootstrap operation; malformed response; no fabricated domain response. | Packet 3 and SPEC corrective tables |
| Catalogs | Immutable snapshot root; page/cursor fingerprint; contiguous ordered entries; empty/single/multi-page; concurrent change; expiry/restart; page-size overflow; gap/duplicate/reorder/count mismatch; final catalog fingerprint. | Packet 3 |
| Admission | Missing/malformed/unknown operation; missing/malformed/missing/mismatched/stale definition pin; missing/malformed/incompatible API context; exact first-failure precedence; separate preselection refusal envelope. | Packets 2-3 |
| Request/result/problem | Generic request/response closure; request/object/response/problem/details fingerprints; all four terminal fields (`data` plus three terminal arrays) and exact status/exclusivity key-set assertions; schema manifest closure; null/default and correlation matrix; raw mutation key accepted only at the definition-pinned typed-body pointer and absent everywhere else. | Packet 0 and operation fixtures |
| Owner/refusal | SDK maps owner validation/mutation/refusal without duplicating authority; owner errors remain typed; no public private HCM-3.6 record; capability absence is honest. | Packets 0, 2, 4 |
| Idempotency/receipts | First execution, exact replay, changed-key conflict, stale/CAS refusal, write-set cardinality/atomic group, recovery/retention/tombstone, no raw-key disclosure, no undeclared writes. | Packet 4; owner remains authoritative |
| Posture equivalence | The seven-field live request is exact and supplies no caller policy/approval/head; all 14 live result variants map exhaustively to status/Problem/details/idempotency/data/two-or-zero receipts; direct Rust and future adapters preserve semantic data, Problems, receipts, replay state, and `original_result_fingerprint`; each transport recomputes its correlation-sensitive outer fingerprint; same/changed valid and absent/null/invalid request-ID vectors are explicit. | Packet 4 corrective mapping |
| 62-row admission truth | Exactly 62 unique contiguous `M62-*` rows equal the 05 inventory; each names an exact live precursor symbol, `absent`, or `phase5_deferred`, plus owner/schema source, mutability/idempotency, current result/refusal/receipt gap, declared/current transports, and `omit`. | `research/20260811T031843Z--corrective-owner-admission-matrix.md`; every row currently omitted |
| Negative transport ownership | CLI help/examples/prose/ambient files/Tauri names do not discover capabilities or branch domain behavior; no CLI/Tauri implementation in HCM-4.2; no compiler resurrection or Phase-5 owner exposure. | Packets 2-5 |
| Repository/proof | Documentation links/paths, line endings, diff checks, selector/dispatch manifest replay, scoped and compare-to-main detection, protected paths, two self-tests, handoff/ledger parity, and local expected-old CAS. | Parent closeout |

## Proof classification

The current repository proves only named direct typed Rust/owner precursors and
existing owner behavior. It does not prove any HCM-4.2 DTO/schema/definition/
catalog/operation/transport implementation. The authoritative matrix therefore
omits all 62 rows from discovery. GitNexus query/context/impact and compare
facilities are unavailable in this session; the final record must retain that
exact unavailability and cannot call it GREEN. No native platform gate is
declared for this documentation-only slice.

## Required final promotion

The only permitted promotion is from `TargetOnly` planning authority to an
implementation-ready planning package, and only additive fresh complete-
subject review/closeout evidence may record that promotion. The frozen
corrective selector and its CLEAN selector review are additive lineage; the
predecessor dated proof wall and historical reviews remain immutable prior
evidence, not current completion evidence. No runtime seam, operation
capability, transport, or phase exit may be promoted by this primary.
