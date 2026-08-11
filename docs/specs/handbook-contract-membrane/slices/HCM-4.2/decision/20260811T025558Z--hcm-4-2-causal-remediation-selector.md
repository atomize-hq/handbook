# HCM-4.2 causal-remediation selector

**Decision timestamp:** `2026-08-11T02:55:58Z`
**Status:** frozen candidate for exact-subject selector review; no corrective
package edit is admitted until that review is `CLEAN`
**Parent orchestration:** `handbook-hcm-4-2-corrective-20260811`
**Integrated outcome:** `HCM-4-2-DOCS-CONTROL-PACK`
**Packet:** `HCM-4.2-PLANNING-CONTROL-PACK`

## Selection and predecessor boundary

This selector admits one additive, documentation-only correction of the
completed HCM-4.2 planning/control package. It consumes, but does not resume or
rewrite, completed predecessor handoff
`20260811T005500Z--HCM-4-2--orchestration--shared-dto-schema-planning-completed`
and its terminal parent `handbook-hcm-4-2-planning-20260811`. The predecessor
record, its two v1.4 dispatches, and its two historical review-evidence files
remain immutable evidence.

The external scheduling/correction input is
`C:/Users/spmcc/.codex/orchestration/handbook-hcm-4-2-planning-20260811T000324Z/reviews/HCM-4.2-causal-audit/receipts/review-completed-findings.json`
at exact SHA-256
`3d30632a5827d8bdcec7adbe47aef77ff387380d5c143839ca9b1b4a8b8498d0`.
That receipt is mandatory correction evidence, not a Handbook selector,
dispatch, review result, handoff, or repository authority.

The selected repository checkpoint is local integration ref
`refs/heads/orchestration/handbook-hcm-4-2-planning-20260811T000324Z` at
commit `ea2be46856172e503e7f7668d2dbaac9d26d1d39`, tree
`9dbad2c418a331d5b2a0c266c55280d81b676aca`. The observed remote baseline is
`origin` `refs/heads/feat/handbook-contract-membrane` at
`1256e724a2b7da6b6250f57d6f63fced1e2cf949`; it is observation-only.

## Exact objective

Repair HCM-4.2 as a planning-only package so the shared public
DTO/schema/bootstrap/catalog contract is internally consistent with frozen
control-pack authority and live HCM-4.1 owner truth; all 62 operation IDs have
exact honest implementation/admission evidence; selector and final-review
subjects are complete, immutable, replayable, and frozen before reviewer
launch; the completion checklist, proof wall, additive handoff, ledger, Git
refs, and terminal receipt agree; a fresh complete-subject review is `CLEAN`
with no unresolved P1/P2; and local-only two-commit closeout plus expected-old
CAS publication succeeds. Completion is HCM-4.2 planning readiness only.

## Mandatory correction set

All ten external-audit findings are mandatory inputs. None may be waived,
downgraded, renamed away, or omitted.

1. `HCM42-CAUSAL-P2-001` — permit the bounded raw idempotency key only in the
   definition-pinned typed mutation request body; prohibit it from responses,
   diagnostics, receipts, replay/public state, and output surfaces; require a
   positive request vector and negative disclosure proof.
2. `HCM42-CAUSAL-P2-002` — remove caller-supplied policy, approval,
   canonical/lifecycle-head, and other engine-resolved authority from the
   public posture request; freeze the live HCM-4.1 boundary and exhaustively map
   every owner result to shared status, Problem, details, idempotency, data,
   and receipts.
3. `HCM42-CAUSAL-P2-003` — freeze complete bootstrap request, response,
   refusal, descriptor, and `CapabilityEntry` field tables, tags, bounds,
   null/default rules, fingerprint preimages, owner versions, and declared
   versus currently available transports; separate descriptor-content
   identity from descriptor-schema identity.
4. `HCM42-CAUSAL-P2-004` — separate LF-free JCS instance/fingerprint bytes,
   one-terminal-LF checked-in schema-file bytes, and adapter-owned framing;
   preserve HCM-4.3 zero-or-one-LF CLI authority and HCM-4.4 Tauri framing.
5. `HCM42-CAUSAL-P2-005` — create one authoritative 62-row matrix in which
   every operation names an exact live `path::symbol` or explicit `absent` /
   `phase5_deferred` state; schema/owner source; mutability/idempotency;
   refusal/result/receipt binding; declared and current transport availability;
   and proof binding. Mechanically resolve live symbols and prove discovery
   omits incomplete and deferred rows.
6. `HCM42-CAUSAL-P2-006` — use this additive selector and a new v1.4 dispatch
   to establish one immutable exact selector subject before remaining package
   authoring; require equality among selector bytes, dispatch subject,
   delegated input/result, review evidence, and closeout claim.
7. `HCM42-CAUSAL-P2-007` — converge the complete primary subject, including
   every primary path and the dated proof wall; include exact transport
   request/response authority, align `tasks/todo.md` with evidence, freeze all
   final-review dispatches before launch, and obtain a different-fresh
   complete-subject review without in-flight rewrites.
8. `HCM42-CAUSAL-P2-008` — require semantic/result/receipt and
   `original_result_fingerprint` equality while each transport recomputes its
   correlation-sensitive outer `response_fingerprint`; cover absent and
   changed valid `request_id` vectors.
9. `HCM42-CAUSAL-P3-001` — leave the historical completed handoff unchanged;
   additively supersede only its bad proof-ref fact with exact primary
   `0a4366802467edbe194da9afaf584686921c1e94` and prove strict proof-ref
   resolution.
10. `HCM42-CAUSAL-P4-001` — replace “four terminal arrays” with “four terminal
    fields” or “data plus three terminal arrays” and add an exact response-shape
    assertion to the planning proof.

## Allowed material paths

Material correction is closed to:

- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`;
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`;
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`;
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`;
- existing mutable planning files and new additive decision, research, proof,
  and review files beneath
  `docs/specs/handbook-contract-membrane/slices/HCM-4.2/`;
- new v1.4 HCM-4.2 dispatches beneath
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/`;
- one new v1.4 parent handoff beneath
  `docs/specs/handbook-contract-membrane/handoffs/records/`;
- deterministic
  `docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`;
- `docs/specs/handbook-contract-membrane/09-review-finding-inventory.md`
  only for exact mechanically required P3/P4 registration or resolution.

The external audit receipt and all protected checkouts are read-only. Historical
records, historical dispatches, historical review evidence, schemas,
validators, templates, runtime orchestration state, and every path not listed
above are excluded from writes.

## Risk and authority ceiling

The maximum product risk is documentation-only planning correction with no
existing code-symbol delta and no ancillary surface. GitNexus MCP and the local
runner are unavailable, so that result is `UNAVAILABLE_NOT_GREEN`; no code
symbol is edited and exact live source/path/symbol plus scoped manual change
detection is required. Any Rust, Cargo, dependency, generated schema, runtime
test, public API, transport, CLI, Tauri, skill, contract/dock runtime, Phase 5
or 6, publication, release, push, remote mutation, protected-path mutation, or
new semantic-owner decision exceeds the ceiling and stops the run.

## Review and causal lineage

This selector is the sole subject of a new planning-stage `discovery` review
under parent `handbook-hcm-4-2-corrective-20260811`. It preserves the
predecessor packet and integrated-outcome identities; a new task, selector,
dispatch, cycle, or fingerprint cannot rename away the audit findings or reset
causal lineage. The dispatch must be schema-valid, content-hashed, and frozen
before a fresh isolated `gpt-5.6-sol` or `gpt-5.6-terra` reviewer at `xhigh`
starts. No package correction beyond this selector may be authored until the
review returns `CLEAN` with no P1/P2.

After selector CLEAN, one consolidated correction pass addresses all ten
findings. The complete applicable primary wall must converge before a final
discovery review or same-fingerprint disjoint-lens burst. If that review finds
P1/P2, the parent performs one consolidated remediation and a different-fresh
delta-focused closure. At most two supplemental causal cycles are available
only for P1/P2 directly caused or unmasked by the immediately preceding repair
inside unchanged scope, authority, and risk. No general discovery reopens; no
cycle follows CLEAN; mechanical closeout consumes no review cycle.

## Required proof and completion boundary

Completion requires exact base/ref/tree/ancestry/remote and protected-path
proof; exact audit digest and ten-ID equality; exact 62-ID set equality and
complete row fields; mechanical live-symbol resolution or explicit absent /
deferred state; corrected request/response/bootstrap/framing/fingerprint
tables and negative cases; immutable selector and final subjects plus
pre-launch dispatch hashes; a complete primary manifest including the dated
proof wall; aligned checklist, proof wall, handoff, ledger, commits, and
receipt; applicable documentation/link/path/UTF-8/JSON/whitespace checks;
`git diff --check`; ordinary validation plus both v1.4 self-tests; honest
permanent-governance treatment of the sole historical HCM-3.5 validator
failure; scoped/manual change detection; final CLEAN; and explicit P3/P4
disposition.

The exact preserved ordinary-validator failure is:

```text
20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation writes or advances before selector CLEAN
```

It remains failed/not GREEN and is governed only by
`docs/specs/handbook-contract-membrane/phase-3-exit/decision/20260809T150000Z--permanent-ordinary-validator-governance.md`.
Any other validator failure blocks.

Successful completion creates a reviewed primary documentation commit, a
separate mechanical v1.4 handoff/ledger closeout commit, and an expected-old
CAS update of the dedicated local integration ref from
`ea2be46856172e503e7f7668d2dbaac9d26d1d39` to the closeout commit. It does not
push. The additive handoff consumes the predecessor as context and supersedes
only the predecessor completion recommendation and facts replaced by this
corrective evidence.

## Explicit non-goals and stops

- Do not implement or claim DTO, schema, catalog, bootstrap, discovery, or any
  of the 62 operations.
- Do not edit Rust, Cargo, tests, generated assets, dependencies, public APIs,
  CLI, Tauri, skills, runtime contracts/docks, Phase 5/6, release, publication,
  remote state, or protected user paths.
- Do not expose private posture records, invent owner symbols, mark an absent
  operation implemented, advertise incomplete/deferred rows, resurrect
  `handbook-compiler`, or grant caller authority the engine resolves.
- Do not rewrite immutable history, waive P1/P2, broaden authority or risk,
  merge, rebase, reset, clean, force-update, fetch, pull, or push.
- Stop for base/protected drift, mandatory delegation unavailability,
  immutable-history mutation pressure, a product/public-contract choice not
  fixed by named authority, or unresolved P1/P2 after the permitted cadence.
