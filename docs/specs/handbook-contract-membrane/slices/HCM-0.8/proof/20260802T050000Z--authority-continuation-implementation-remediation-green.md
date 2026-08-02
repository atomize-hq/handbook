# HCM-0.8 Authority-Continuation Implementation Remediation

Recorded at: 2026-08-02T05:00:00Z

Dispatch:
`20260802T043500Z--HCM-0-8--authority-continuation-implementation-remediation`

## Result

The v1.4-only authority-continuation control is implemented without Rust,
Cargo, dependency, public API, transport, HCM-3.2 product, historical-record,
or historical-dispatch edits. The implementation adds one canonical immutable
grant, exact authority-artifact projection and attestation replay, direct
authority-stop admission, selector-CLEAN gating, finite ordered slots,
baseline-to-tip path/symbol/risk reconciliation, successor summary parity,
and final single-use consumption.

## RED and GREEN evidence

The first orchestration-contract run rejected the new template field as an
unknown property. Remediation then added the optional v1.4 schemas/templates,
validator helpers, and the exact inert HCM-3.2 authority artifact. The focused
suite now accepts the exact committed five-dispatch HCM-3.2 prefix followed by
artifact-first different-parent CLEAN attestation, admission, active successor,
material/proof/final slots, and consumed successor.

Focused negatives reject stale or missing authority/review bytes, postdated or
parity-drifted grants, identity drift, predecessor-baseline drift, attestation
FINDINGS or subject mismatch, same-parent attestation, slot/cycle violations,
write-before-CLEAN, summary ceiling drift, second successor, and reuse after
consumption. The grant fingerprint has a fixed JCS+LF golden vector, changes
for every stable top-level grant field, and is invariant under slot-only change.

## Validation evidence

- Python compilation: PASS.
- Orchestration contract self-test: PASS.
- Ordinary validation: PASS, including 82 records, 471 current dispatches,
  eight admitted legacy dispatches, and exact 82-entry ledger parity.
- Historical v1 admission self-test: PASS.
- `git diff --check` and changed-subject trailing-whitespace scan: PASS.
- GitNexus scoped `all` detection: `HIGH`, 11 tracked control files, 39
  mapped symbols, seven affected validator/self-test flows. This is related to
  the reviewed v1.4 validator/control authority surface; no product, dependency,
  runtime, transport, or public API expansion was reported.
- GitNexus compare-to-`main`: `CRITICAL`, 1,294 files and 9,437 symbols. This is
  branch-wide inherited divergence and is not classified as slice GREEN; the
  exact scoped result above is the HCM-0.8 observation. FTS remained unavailable.
- Earliest planning-dispatch live-byte replay: expected stale-subject refusal
  after reviewed SPEC remediation; immutable prefix/schema/record replay remains
  covered by ordinary validation and the exact continuation fixture.

## Remaining parent-owned gates

Fresh final aggregate review, any finding remediation/closure, protected-path
verification, commits,
handoff/ledger closeout, local CAS publication, and the terminal receipt remain
with the parent orchestrator. This proof grants no HCM-3.2 execution authority.
