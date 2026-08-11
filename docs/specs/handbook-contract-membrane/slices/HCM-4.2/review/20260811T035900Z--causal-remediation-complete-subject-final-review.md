# HCM-4.2 causal-remediation complete-subject final review

**Dispatch:**
`../../../handoffs/dispatches/20260811T034900Z--HCM-4-2--causal-remediation-complete-subject-final-review.json`
**Dispatch SHA-256:**
`36c7e08479b8feb1f60190330df279426f069b1c5bbefb58e832365c942fc392`
**Exact subject:**
`sha256:e47e7af466bc0089b91d9620aeceb12c648d1cc453e6da2a53cac29f40c946ba`
**Fresh reviewer:** `/root/hcm42_final_review`
**Verdict:** `CLEAN`

The fresh isolated reviewer returned no P1, P2, or P3. One P4 wording advisory
is retained below. Under the live priority and causal-cadence contract, CLEAN
ends review: the frozen subject is not rewritten and no post-CLEAN review cycle
runs merely to polish the advisory.

## Findings

| Priority | Finding | Disposition |
|---|---|---|
| `P4` | `proof/strategy.md:18` says “missing/malformed/missing/mismatched/stale definition pin,” duplicating `missing` and obscuring the distinct `definition_registry_missing` case that 05 correctly distinguishes. | Retained as `HCM-RF-0003`. The normative 05 contract is correct, no proof is missing, runtime behavior is unaffected, and the frozen exact subject remains unchanged after CLEAN. |

No other finding was returned. In particular, all ten external-audit inputs
remain individually corrected and none is waived, renamed, downgraded, or
treated as authoritative merely because it originated in the external receipt.

## Independent replay

- The dispatch SHA-256 matched before and after review, and
  `validate_handoffs.py --verify-dispatch` passed.
- All 14 manifest entry hashes matched. The independently reconstructed
  ordinal `repo-path NUL sha256 LF` aggregate matched the exact subject.
- Strict UTF-8, final LF, trailing whitespace, JSON parsing, 31 relative links,
  and `git diff --check` passed.
- The worktree remained at exact base/tree; the observed path population was
  the 14 primary subject paths plus the one final-review dispatch. No Rust,
  Cargo, generated schema, runtime, transport, historical, or protected path
  was present.
- The matrix replay returned 62 unique contiguous IDs and exact set equality
  with 05, HCM-4.1, and the predecessor inventory: 26 `live_precursor`, 24
  `absent`, 12 `phase5_deferred`, 62 current transports `none`, and 62
  discovery decisions `omit`.
- All 29 matrix `path::symbol` references resolved. The test-only setup helper
  remained excluded, and `pipeline.route.resolve` was confirmed to persist a
  route basis, supporting its explicit semantic-mismatch omission.
- Live posture source has exactly seven request fields, seven receipt fields,
  two successes, one blocker, eight refusals, and three errors. SPEC and 05
  map the complete owner result space.
- The corrected proof ref resolved at exit `0`; the malformed historical ref
  remained preserved and failed at exit `128`.
- Seven immutable predecessor artifacts matched their preserved SHA-256
  values. Selector discovery -> implementation -> final review used the same
  parent, packet, causal budget, and registered integrated outcome.
- Both v1.4 self-tests passed. Ordinary validation failed only on the exact
  permanently governed HCM-3.5 selector contradiction and remained
  `FAILED_NOT_GREEN`.
- GitNexus MCP and local runner remained unavailable, recorded as
  `UNAVAILABLE_NOT_GREEN`; no code symbol changed.

## Parent disposition

The parent may commit the reviewed HCM-4.2 planning-readiness state, retain
`HCM-RF-0003`, and perform the separate mechanical v1.4 handoff/ledger plus
expected-old local-only CAS closeout. This review authorizes no runtime,
public-contract implementation, publication, push, remote mutation, or next
increment.
