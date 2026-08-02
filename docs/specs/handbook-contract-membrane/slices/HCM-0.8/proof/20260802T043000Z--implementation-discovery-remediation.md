# HCM-0.8 Implementation Discovery Remediation

Recorded at: 2026-08-02T04:30:00Z

Reviewer: `/root/hcm08_authority_continuation_implementation_discovery`
Dispatch:
`20260802T041400Z--HCM-0-8--authority-continuation-implementation-discovery`
Reviewed subject:
`sha256:1ee91ab28d488029e6ebae5fab0d9d2b0de0c0d034961864d009577d9c612556`
Verdict: `FINDINGS`

## Consolidated disposition

### HCM08-DISC-P1-001 — P1 — remediated

Artifact parity now uses an exact acyclic payload projection that excludes only
`authority_ref.path`, `authority_ref.sha256`, and
`authority_ref.attestation`. Those fields remain inside the canonical
fingerprinted grant and are independently replayed. Construction order is
projection, artifact hash, separate CLEAN attestation, complete grant, grant
fingerprint. No mutual hash remains.

### HCM08-DISC-P1-002 — P1 — remediated

The baseline is exactly the direct predecessor handoff's reviewed commit/tree,
not a future artifact-containing commit and not an arbitrary valid Git pair.
For HCM-3.2 the frozen pair is `fe62a4b57854830373a57ccdcc7acb9f4b7a9ac0`
and `671e0fac370d91bb4fe57073fa63ad37f55f4c40`.

### HCM08-DISC-P2-003 — P2 — remediated by required proof

The implementation must construct the exact artifact-first HCM-3.2 sequence:
committed predecessor and five-dispatch prefix, projected authority artifact,
different-parent CLEAN attestation, complete grant, admission selector, active
successor, later material reviews, and consumed completed successor. Projection
mutation and predecessor-baseline drift are mandatory focused negatives.

## Closure gate

The partial implementation remains unaccepted until remediation converges and a
different fresh reviewer closes all three findings over the complete changed
subject. No product work or closeout is authorized.
