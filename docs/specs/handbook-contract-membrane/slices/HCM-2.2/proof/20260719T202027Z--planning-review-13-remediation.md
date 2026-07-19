# HCM-2.2 Planning Review 13 Remediation

## Rejected reviewed subject

Fresh isolated built-in reviewer `/root/hcm_2_2_planning_review_13` replayed
the immutable 52-path subject selected by
`20260719T194155Z--HCM-2-2--fresh-planning-review-13` and independently
reproduced aggregate
`sha256:399e9237022390501cb58cba6429eb29397ed11d037fa0a89c9530aa7bc2bc79`.
The verdict was `CHANGES_REQUIRED`; it authorizes no commit.

## Accepted Required findings

1. The opaque `get_assertion` port response was not byte-closed. The packet
   specified request bytes and semantic assertion fields but omitted the exact
   status/CBOR response map, required credential descriptor, deterministic
   multi-credential selection, strict DER boundary, malformed-response
   rejections, and raw response proof.
2. The required incomplete final-use `add_credential` lockout refusal was not
   representable because the add refusal branch omitted `lockout_refused` and
   no exact result vector proved zero native calls and zero authority delta.

## Parent remediation

- Added the exact closed
  `handbook.schemas.security.authenticator-get-assertion-response@1.0.0`
  schema and selected it in the approval-policy and shipped-profile closure.
- Froze success status `0x00`, the exact canonical CBOR `{1,2,3}` response map,
  mandatory returned credential descriptor, complete strict DER signature,
  exact nonzero-status mapping, raw response preservation, and the
  content-addressed `authenticator-get-assertion-responses/` partition.
- Froze eligibility as committed, active, sequence-below-4096, ES256, exact-
  authority-pair coverage; duplicate IDs refuse; the allow-list is capped at
  64 and sorted by raw credential-ID bytes. The returned descriptor must match
  exactly one frozen entry.
- Expanded all five assertion transcripts with exact request bytes, exact raw
  response bytes, decoded response records, and matching deterministic DER
  ES256 signatures. Added twelve malformed/status/descriptor/signature
  response negatives and four credential-selection vectors.
- Added the decoded response to the assertion schema and runtime identity
  chain. Add success now reports six ordered paths; revoke/update success report
  four, each with decoded response before assertion.
- Added `lockout_refused` to the add-refusal branch, one exact sequence-4095
  incomplete-replacement refusal with every semantic ref null, no changed path,
  zero native calls, and no registry/use-head delta, plus a crossed bootstrap-
  code rejection.
- Regenerated all affected schema entries, literal JCS, definition closure,
  selected profile/intake fingerprints, runtime identities, 33 bounded security
  groups, plan/todo obligations, and exact raw intake hash.

## Regenerated exact heads

- approver-admin API schema entry:
  `sha256:041f070af4d085db3140d6568ba0c8799036b76efff3677103361ac0430d19c4`
- authenticator GetAssertion response schema entry:
  `sha256:c98d1ae17e784321712568e20fb66dac34baacb1d3878bf9cd53c31f39ba921e`
- authenticator assertion schema entry:
  `sha256:5956c7b05342faaa2f40bccadd195009c56dcbde8440dc50fbca7fcf731da5c1`
- approval policy:
  `sha256:aece61723171122c5f7ad0c8cafa88665d3a5cb984174afcfd38fb14beabbe6d`
- waiver policy:
  `sha256:a8930621420a6d3ee96ae5e6faf69b61b5a71b58a5ef0821997c9ee478b41066`
- lifecycle policy:
  `sha256:d43def37e010ed41d8d73cf8f49f2964aac9899de9dfbdbe530236b8fe737e69`
- Project Authority kind:
  `sha256:e6e2fa824d5d614e03f56eea84dac571a928bb38da8ea0a752128dc2cde402f2`
- Charter intake:
  `sha256:4fd3a94b54103fc54c9b2eef5f7ff5de8a1fab968f537406eb2abb9160d65666`
- shipped profile:
  `sha256:f23ce328d8d1c4085f744f431b742b9db7dca4ec147579ea6a8b41144e94245d`
- exact 10,009-byte intake raw SHA-256:
  `83ec717440d3f75805f43bc9204598552fe687d77513997ace0d36bec9c2495a`

## Required replay before Review 14

The parent must replay eleven schema documents/entries, eleven definition/JCS
vectors, fourteen runtime identities with seven published-schema validations,
thirteen admin positives and seventeen negatives, all seven transcripts with
five raw GetAssertion response/signature positives, twelve raw negatives, four
selection vectors, all 33 security bounds, waiver/user-handle/cross-family/
final-use proof, all three handoff modes, links/tasks/diff gates, dependency
object, retained profile-schema tests, and focused engine/compiler/flow/CLI
baselines. A different fresh reviewer must receive a newly sealed exact subject.

## Parent replay result

PASS. `/tmp/validate_hcm22_review13.py` reproduced the complete 11/11 closure,
14 runtime records/seven published schemas, 13 positive/17 negative admin
vectors, seven transcripts with five valid raw assertion responses and twelve
negatives, four selection cases, 17 profile/nine security sources, 33 bounded
groups, exact 10,009-byte intake identity, waiver/user-handle/final-use/cross-
family invariants, and real ES256 verification for every assertion operation.

The retained baselines passed unchanged: profile schema shape 2, engine author
core 8, compiler author 49, canonical ingest 14, flow resolver 17, and CLI
author 22. All three handoff modes passed with 45 records, 196 current
dispatches, eight admitted legacy dispatches, and 45 ledger rows. `git diff
--check`, unchecked-task, exact-scope, absolute-path, dependency-object, and
relative-link checks passed across 21 Markdown files and 62 relative links.

No product code, runtime definition, canonical artifact, implementation
authority, primary commit, closeout handoff, or ledger row was created by this
remediation.
