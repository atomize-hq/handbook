# HCM-2.3 Implementation Review 2 Remediation

> **Status (2026-07-22): Review 2 remediation completed; fresh Review 3 later
> returned `CHANGES_REQUIRED` and is tracked in its separate remediation record.** This record does not claim `CLEAN`, a primary commit,
> closeout, or authority for HCM-2.4 or any later phase.

## Review admission and verdict

Fresh isolated Review 2 admitted the immutable 47-path subject from
[`20260722T175647Z--HCM-2-3--fresh-complete-implementation-review-2.json`](../../../handoffs/dispatches/20260722T175647Z--HCM-2-3--fresh-complete-implementation-review-2.json),
aggregate
`sha256:322ef3a897cabe756ff44fd6f1cc5b51555d75fae2c8c60edfd27109f1e08abd`.
It returned `CHANGES_REQUIRED` with four Critical and five Required findings.
Every finding is accepted without waiver; the immutable Review 2 dispatch and
subject hashes remain unchanged.

## Accepted findings

1. The raw generic lineage store was publicly reachable and permitted callers
   to bypass repository authority.
2. Recovery trusted self-fingerprinted journals, paths, and runtime records
   without proving their semantic authority against current repository truth.
3. The exact operation context and plan were not revalidated immediately before
   establishment.
4. Native repository I/O retained path races around post-read identity,
   hard-link aliases, and install verification.
5. Structurally invalid intake was mapped to the wrong refusal class.
6. Public generic reads could bypass the common lock/recovery path.
7. The legal established-intent plus exact empty pending-shell crash state was
   rejected.
8. Canonical replacement scratch was not deterministically crash recoverable
   and did not recheck the basis immediately before publication.
9. The runtime contract and implementation evidence overclaimed these
   invariants before the executable implementation existed.

## Remediation rule and impact authority

Every repair began with a failing regression and remains within the selected
HCM-2.3 engine/test/control/proof owners. No Review 2 finding is waived. The
user's additional schema-root authorization remains a separate bounded CRITICAL
repair confined in production to `schema_registry.rs`: BuiltIn containment is
the fixed `definitions/schemas` package domain and allowlist-only byte source;
Repository containment is request-root-only; the HCM-2.3 request contains only
`.handbook/definitions/schemas`.

The exact schema impact evidence remains:

| Exact symbol UID | Command | Direct callers / affected / processes / modules | Reported risk / disposition |
|---|---|---:|---|
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | 2 / 2 / 1 / 2 | LOW / treated CRITICAL |
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | 1 / 3 / 1 / 2 | LOW / treated CRITICAL |

`SchemaRegistry::load_admitted_deferred_fingerprints` remains untouched. For
Review 2 native-I/O remediation, exact UID
`Struct:crates/engine/src/canonical_repo_support.rs:TrustedRepoFile` at depth 5
including tests reported CRITICAL: 1 direct caller, 108 affected symbols, 56
processes, and 20 modules. New unindexed helpers `reject_link` and
`path_has_single_link` each reported `UNKNOWN`, zero indexed callers, and zero
indexed flows/modules; both were explicitly treated as CRITICAL.

## Implemented remediation evidence

| Finding | Implemented evidence |
|---|---|
| 1 | `artifact_lineage_store` is private; raw store operations and types are crate-private or test-only. The public mutation/repository surface reexports only safe owned DTOs. A source-surface regression proves the raw module is not public. |
| 2 | Every intent persists its complete operation-discriminated request subject. Recovery validates exact transaction/path derivation, full nested runtime-record shape, staged and installed outputs, selected instance/context, schema closure, intake binding, promotion definitions, and canonical authority. Schema-valid fully refingerprinted forgeries refuse with zero state delta. |
| 3 | Planning is repeated under retained repository authority and exact plan equality is required immediately before intent establishment. Candidate requests bind the context fingerprint; promotion requests additionally bind the canonical ref. Authority drift before lookup or establishment refuses without a journal. |
| 4 | Strict reads retain target and parent handles, compare native before/after identity and timestamps, reject Unix and Windows hard-link aliases, retain bounded-read error precedence, and fail closed on unsupported identity checks. Install/recovery reopens exact bytes; canonical basis is checked immediately before publication. Long authored paths use the Windows provider only when `fsutil` cannot query the extended-length path. |
| 5 | Structurally invalid intake becomes the closed `StructuralValidationFailed` established refusal, persists no artifact/closure output, and replays byte-identically. Nonselected targets remain pre-establishment control failures. |
| 6 | All eleven public repository read families enter the same authority-held recovery-first session and return owned data. Markerless promotion completion and changed-selection refusal are proved before any read result is exposed. |
| 7 | Recovery admits only an established intent plus its exact zero-entry deterministic pending shell; nonempty, dual, extra, or mismatched states still refuse. |
| 8 | Canonical replacement uses deterministic `.<name>.generic-replace` scratch, accepts only exact intended bytes, and rechecks the expected basis immediately before atomic publication. Review 3 subsequently tightened this behavior: absent, partial, unequal, or native-identity/version-mismatched residue now refuses without rewrite or adoption. |
| 9 | The runtime contract and SPEC now state the private API, complete request subject, current-authority validation, exact empty shell, deterministic scratch, basis recheck, and cross-platform link behavior. Candidate classifications remain review-pending. |

## RED/GREEN evidence to date

- RED: the mixed BuiltIn/Repository fixture failed while its request carried
  only `.handbook/definitions/schemas`; GREEN: registration/kernel 13/13 and
  schema source-domain/direct registry proof pass without the removed root
  adapter.
- RED: a nested schema-invalid but fully refingerprinted candidate journal was
  accepted; GREEN: it refuses, as do forged current-context, definition,
  canonical, transaction, filename, staged, and installed bindings.
- RED: the canonical replacement basis test exposed old-basis bytes as though
  they were an installed intended output; GREEN: every legitimate old basis is
  distinguished and the exact replacement/restart/basis tests pass.
- RED: Windows long authored paths failed closed as `SourceReadFailure`, and
  bounded oversized reads lost `SourceLimitExceeded`; GREEN: schema registry
  19/19 restores both behavior and native link checks remain green.
- Current focused wall: generic lineage 33/33, registration 13/13, selection
  1/1, direct schema registry 19/19, and actual CLI 5/5 on Windows.

The complete HCM-2.2 wall passes 89 engine and 10 CLI tests. Windows all-target
Clippy, complete engine, complete CLI, and complete workspace tests pass. WSL
all-target Clippy, generic lineage 33/33, direct schema registry 19/19, and
source-domain 5/5 pass, including Unix hard-link and restored-mtime mutation
proof. Engine packaging, dependency tree, archive normal/self-test, all three
handoff validators, duplicate-safe JSON/schema replay, Markdown link/fence
validation, preservation hashes, scope scan, formatting, and diff checks pass.
CLI packaging reproduces only its unchanged baseline missing-version metadata
limitation. Pre-review GitNexus change detection reports the expected CRITICAL
reach: 26 indexed files, 68 changed indexed symbols, and 293 affected flows;
new untracked private modules remain incompletely represented. A fresh
complete-subject Review 3 remains pending before commit.
