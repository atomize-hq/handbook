# HCM-0.8 Authority-Continuation Evidence Remediation

Recorded at: 2026-08-02T05:59:00Z

Dispatch:
`20260802T055000Z--HCM-0-8--authority-continuation-final-review-remediation`

## Disposition

`HCM08-FINAL-P1-004` and retained `HCM08-DISC-P2-003` are remediated in the
bounded control-plane subject. A code-bearing v1.4 authority continuation now
requires an available, byte-hashed GitNexus compare artifact. The artifact
binds provider/version, exact command and scope, baseline and target
commit/tree, actual changed paths, complete changed symbols and their risks,
aggregate maximum risk, and a raw-output fingerprint.

The handoff summary, evidence artifact, replayed Git path delta, and immutable
grant must agree. Code-bearing evidence cannot report an empty symbol list.
Every observed symbol/risk pair must be granted, and both evidence and summary
must report the exact maximum observed risk. The evidence bytes must also be
present in a review dispatch manifest attested by a completed CLEAN review run
within the same successor handoff.

## Exact replay

The exact HCM-3.2 self-test uses an isolated temporary Git clone at the grant's
real baseline and creates one synthetic code-bearing commit. Its reserved
evidence path reports two authorized symbols, including one CRITICAL symbol,
and exact CRITICAL aggregate risk. Its synthetic authority attestation uses
only the reserved supplemental review path; the immutable first final review
remains FINDINGS and cannot attest authority.

Focused negatives reject:

- missing GitNexus evidence;
- empty symbol coverage for a code path;
- a symbol omitted between evidence and summary;
- an unauthorized symbol/risk pair;
- risk understated below the maximum observed symbol risk; and
- evidence without a completed CLEAN same-handoff attestation.

## Scope and gate

No product, Rust, Cargo, dependency, runtime, transport, shipped-identity, or
HCM-3.2 product artifact changed. The reserved supplemental dispatch and the
reserved future HCM-3.2 evidence artifact were not created. No handoff, ledger,
commit, push, publication, or terminal receipt is authorized by this proof.
Different-fresh supplemental aggregate review remains required before closeout.

## Validation wall

- Python compilation: PASS.
- Ordinary validation: PASS — five record schemas, five dispatch schemas, two
  templates, 82 records, 473 current dispatches, eight admitted legacy
  dispatches, and exact 82-entry ledger parity.
- Historical v1 admission self-test: PASS.
- Orchestration-contract self-test and exact synthetic continuation replay:
  PASS, including all focused evidence negatives above.
- `git diff --check`, strict UTF-8, and changed-subject trailing-whitespace
  scans: PASS.
- GitNexus scoped `all`: HIGH — 11 files, 46 symbols, nine affected processes;
  this is the bounded control-plane surface already authorized after upstream
  impact review.
- GitNexus compare-to-`main`: CRITICAL — 1,294 files, 9,438 symbols, 249
  affected processes; this is inherited branch-wide divergence and is not
  classified as this remediation's scoped risk.
- Live replay of the immutable remediation dispatch: expected stale-subject
  refusal at the changed v1.4 handoff schema after the exact eight-file input
  was replayed before edits.
