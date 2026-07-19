# HCM-2.2 Planning Review 14 Remediation

- Recorded at: `2026-07-19T21:03:59Z`
- Review dispatch: `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T202418Z--HCM-2-2--fresh-planning-review-14.json`
- Reviewed subject: 55 exact paths, aggregate `sha256:ed462cc33364734f183a175889a4e46ba7e07b175e6a587bd35d493da034f24b`
- Fresh reviewer: `/root/hcm_2_2_planning_review_14`
- Review verdict: `CHANGES_REQUIRED`
- Commit authorization: none

## Required findings accepted

1. The successful GetAssertion boundary did not close the complete
   authenticator-data byte string: exact length, literal allowed flags, AT/ED,
   reserved/backup flags, and trailing bytes were not all refused and proven.
2. The nonzero CTAP status mapping classified every value except `0x2e` as
   `AUTHENTICATOR_UNAVAILABLE`, conflating received authenticator outcomes with
   port/transport unavailability and leaving refusal/retryability semantics
   untyped.
3. The final-use add lockout vector froze only an output. It did not publish the
   exact request, sequence-4095 registry/use-head prestate, incomplete mapping,
   native-call log, all-null result, or byte-identical before/after authority.
4. The normative runtime-record matrix omitted the decoded GetAssertion
   response row and retained the stale count of six schema-governed rows.

## Remediation

- Closed both GetAssertion response and assertion schemas to exactly 37 decoded
  authenticator-data bytes and canonical 52-character base64, literal flags
  `0x05`, and explicit false AT/ED facts.
- Added validly signed AT, ED, reserved-flag, and trailing-authenticator-data
  negatives; the response negative set is now 16. Re-signed the missing-UV
  negative over its mutated preimage so the flag violation is isolated.
- Added `ctap-status-mapping-v1.0.json`, a disjoint total partition of every
  received `0x01..0xff` value into typed refusal/retryability/next-action groups,
  with representative success, group, reserved, extension, and vendor vectors.
  A complete received status never maps to `AUTHENTICATOR_UNAVAILABLE`.
- Extended all four operation-specific refused-result branches with the exact
  received-status refusal vocabulary while preserving the all-null/no-delta
  result shape.
- Added `final-use-add-lockout-vectors-v1.0.json` with the exact add request,
  sequence-4095 registry/use-head prestate, incomplete required-pair coverage,
  all-null `lockout_refused` result, empty native-call and semantic-record logs,
  and byte-identical authority before/after.
- Added the decoded GetAssertion response to the normative runtime-record
  matrix and corrected the published-schema count from six to seven.
- Recomputed the three changed schema-registry entries and every dependent
  approval/waiver/lifecycle/kind/intake/profile closure fingerprint. The exact
  CTAP-status and final-use fixtures are fingerprint-bound dependencies of the
  approval definition.

## Proof replay

PASS:

- exact closure regeneration reproduced 11 schema entries, 11 definition/JCS
  vectors, and the dependent approval/waiver/lifecycle/kind/intake/profile
  fingerprints;
- 14 runtime identities validated, including all seven published-schema rows;
- 13 positive and 17 mandatory-negative admin vectors validated;
- all five raw GetAssertion successes and 16 response negatives reproduced;
  the flag/length negatives carry valid signatures over their malformed
  authenticator data, and all five successes verify against the registry key;
- the total CTAP table covered every `0x01..0xff` value exactly once, and the
  complete final-use refusal reproduced byte-identical authority;
- all 33 first-admission bounds, all 17 profile schema sources, and all nine
  security sources remained exact;
- handoff validation passed in all three modes with 45 records, 197 current
  internal dispatches, eight admitted legacy dispatches, and 45 ledger rows;
- `git diff --check`, unchecked-task, absolute-path, exact 59-path scope, and
  relative-link gates passed (22 Markdown files, 65 relative links); and
- retained baselines passed: profile schema 1, engine author 8, compiler author
  49, canonical ingest 14, flow resolver 17, and CLI author 22 tests.

This remediation does not authorize a commit. The parent must dispatch the
expanded exact-path subject to a new fresh independent reviewer.
