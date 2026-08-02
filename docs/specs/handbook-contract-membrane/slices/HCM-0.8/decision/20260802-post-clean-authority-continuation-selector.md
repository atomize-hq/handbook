# HCM-0.8 Post-CLEAN Authority Continuation Selector

Status: proposed; requires independent planning review

## Decision

Select one additive v1.4 `authority_continuation` grant and one matching
handoff summary. The grant is an authority-bound finite overlay on the existing
causal budget, not a new budget, outcome, packet registry, review identity, or
ordinary discovery allowance.

## Admission selector

The mechanism admits exactly one continuation when all conditions hold:

1. The referenced predecessor is the latest direct handoff for the same parent,
   is non-completed, and stopped at `authority_boundary`.
2. Its dispatch prefix is complete, immutable, and bound by the recorded
   population fingerprint.
3. The grant repeats the original parent, outcome, registry, budget, and packet
   identities exactly and fingerprints one typed, independently issued and
   separately CLEAN-reviewed repository authority artifact, immutable Git
   baseline, total subject-path ceiling, symbol delta, risk ceiling, and fixed
   review allowance. Issuer/owner provenance and the exact acyclic artifact
   payload projection must match; only authority path/hash/attestation are
   excluded from artifact bytes and remain fingerprinted in the full grant.
4. The first new dispatch is a read-only `authority_admission` discovery whose
   manifest replays the exact authority bytes.
5. No write or later review slot exists until that selector run is independently
   CLEAN. Any admission P1/P2 terminates the one-attempt immutable grant and
   requires new external authority at a true stop; it cannot be remediated by
   a continuation dispatch.
6. The only later slots are `implementation`, `proof`, and `final_closeout`, in
   that order, each retaining the existing discovery/closure/at-most-two-
   supplemental P1/P2 cadence and no-cycle-after-CLEAN rule.
7. Each cycle contains exactly one dispatch; bursts are not admitted. One
   completed final CLEAN consumes the grant and makes every later dispatch
   invalid. A parent cannot receive another grant.

## Exact schema surface

`internal-dispatch.v1.4.schema.json` adds optional
`authority_continuation`. It contains an immutable `grant`, its recomputed
`grant_fingerprint`, and the per-dispatch `review_slot`. The grant contains:

- `extension_id`;
- `predecessor_handoff_id` and
  `predecessor_dispatch_population_fingerprint`;
- unchanged parent/outcome/registry/budget/packet identity;
- `authority_ref.path`, `authority_ref.sha256`, typed issuer/owner provenance,
  issuance time, source task/thread/host/nonce, and an exact authority
  attestation naming a completed different-parent v1.4 handoff, its completed
  CLEAN review run, and that run's immutable dispatch ref/hash;
- `baseline_commit` and `baseline_tree`, exactly equal to the direct
  predecessor handoff's reviewed baseline;
- sorted `subject_path_ceiling`, sorted `symbol_delta`, `risk_ceiling`, and
  `scope_delta`;
- the fixed ordered four-slot review allowance and per-slot cycle ceiling;

`grant_fingerprint` is the lowercase SHA-256 of UTF-8 RFC 8785/JCS canonical
JSON for the complete immutable grant plus one LF, prefixed with `sha256:`.
The fingerprint and varying slot are outside the grant. There is no additional
Unicode/path normalization beyond schema path rules. A golden vector and one
mutation per stable field are mandatory.

`causal_control.review_stage` adds `authority_admission` and
`stage_transition.kind` adds `authority_extension`. Both are invalid without a
grant, and `authority_extension` is valid only on the first selector dispatch.

`handoff-record.v1.4.schema.json` adds optional
`authority_continuations`, with at most one entry containing the grant identity,
selector review run, exact consumed cycle and dispatch IDs by slot, baseline,
actual Git path delta, complete GitNexus changed-symbol/risk observations, and
active/consumed status. Existing records may omit it. Validator comparison of
the actual baseline-to-tip Git delta and observed symbol/risk set against the
grant is mandatory rather than trusting the summary.

For every code-bearing delta the summary also names an exact GitNexus evidence
artifact path/hash and a completed CLEAN review run in the same handoff. The
artifact binds provider/version, command/scope, baseline/target commit/tree,
actual paths, complete symbols/risks, aggregate risk, and raw-output hash.
Validator replay requires exact actual-path/evidence/summary equality, nonempty
symbol coverage, risk equality, grant containment, and an immutable review
dispatch manifest entry for the evidence. Unavailable, empty, fabricated,
unattested, omitted-symbol, unauthorized-symbol, and understated-risk cases fail.

The attestation is mechanically replayed, not trusted by declaration: the
handoff must validate and be completed, the run must be a completed CLEAN
review with unchanged result subject, its dispatch manifest must contain the
exact authority path/hash, and issuer/reviewer/admission-reviewer/executor roles
must be distinct as specified. Missing, fabricated, unexecuted, FINDINGS,
same-parent, subject-mismatched, or role-colliding attestations fail closed.

Construction order is artifact payload projection, artifact hash,
different-parent CLEAN attestation dispatch/handoff, then complete grant and
grant fingerprint. The artifact excludes only its own path/hash and the later
attestation, eliminating mutual hash dependency without weakening replay.
Baseline equality is not a general Git-validity check: it must match the direct
predecessor handoff's reviewed commit/tree. The exact HCM-3.2 pair is
`fe62a4b57854830373a57ccdcc7acb9f4b7a9ac0` /
`671e0fac370d91bb4fe57073fa63ad37f55f4c40`.

This HCM-0.8 increment creates the exact slice-local HCM-3.2 authority artifact.
Only its eventual completed HCM-0.8 closeout handoff and final CLEAN run can
attest it; therefore the artifact cannot admit HCM-3.2 before this increment is
closed and independently reviewed.

## HCM-3.2 regression binding

The positive fixture consumes the real committed handoff
`20260802T012613Z--HCM-3-2--orchestration--authority-lineage-composition-required`,
its five exact dispatch refs, population fingerprint
`sha256:3fea1b8b2212916ae1aaa9a3cb249a04e85875365a9b7b2f2da640a441fd8a90`,
and causal budget
`sha256:9e317280367a3668c484802c4cbc0e4e090faad692692e8327a40c11b73346ae`.
The fixture adds a qualifying authority artifact with the exact recorded
meta/user issuer provenance and a separate CLEAN review, then appends extension
dispatches without copying, rewriting, reparenting, or relabeling the five
predecessors. It builds an active direct successor handoff and a completed
direct successor that consumes the grant. Negatives cover self-issued,
postdated, issuer/owner mismatch, fabricated/unexecuted/FINDINGS or same-parent
attestation, result-subject mismatch, role collision, authority parity drift,
artifact-projection mutation, predecessor-baseline drift, actual path/symbol/risk
ceiling breach, admission reuse, summary mismatch, second successor, and
post-consumption dispatch.

## Rejected alternatives

- Resetting the causal budget or parent/outcome identity.
- Reopening prior CLEAN stages as ordinary discovery.
- Treating final closeout as selector or implementation review.
- A free-form `extra_cycles` integer without typed slots and authority binding.
- Allowing more than one extension or allowing a completed parent to resume.
- A self-test-only exception keyed to HCM-3.2 IDs.

## Stop conditions

Stop if review finds that the grant cannot mechanically bind direct predecessor,
identity, subject ceiling, selector CLEAN, finite slots, and terminal
consumption without a generic reset or historical rewrite.

## Retained-finding lineage amendment

The identity-bound authority extension selects optional
`delegated_runs[].carried_finding_refs` as the smallest additive repair. It
does not change `findings[].source_run_id` or transfer ownership. A carrier may
name only P1/P2 IDs owned by exact `review_cycle.trigger_run_ids`, and only when
completed remediation re-reviews that owner into the carrier. A carrier with
FINDINGS is blocking even when it owns no new P1/P2. The immediately following
cycle's exact finding array is the owned-plus-carried union of its trigger
runs. Missing fields remain empty for old v1.4 records.
