# HCM-2.2 Planning Review 7 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_7`
- Dispatch:
  `20260719T165652Z--HCM-2-2--fresh-planning-review-7.json`
- Reviewed subject: 35 paths,
  `sha256:93b53a1b94a134f7b974d84a403f3d8783a6e1f2723c54022ee1176c9c53b508`
- Result: `CHANGES_REQUIRED`
- Required findings: 6
- Optional findings: 0
- Nit findings: 0

Review 7 authorizes no commit.

## Exact remediation

1. **Live dependency commit.** Corrected the nonexistent HCM-2.1 dependency
   hash to reviewed commit
   `d61670eb2906c6725d0c268c8f63245297808b6f` in the specification and proof
   wall. Git object lookup is required in the replay.
2. **Content-addressed bootstrap refs.** Regenerated the bootstrap registration,
   registry state, and transition so every registration/result-state/
   authorization ref names the exact target ID. Replay now asserts ref basename,
   target ID, fingerprint, and exact target bytes together rather than accepting
   fingerprint equality alone.
3. **Distinct add enrollment.** Published a distinct protocol-valid add-
   enrollment credential/response and bound its exact credential-ID hash into
   the following add assertion. Bootstrap and add credential bytes, response
   IDs, and response fingerprints differ; a cross-vector equality check is
   mandatory.
4. **One admitted protocol variant.** Narrowed every exact approval, registry,
   registration-request, registration, assertion, and transcript contract to
   `ES256` with `fmt: none` and attestation policy `none`, matching the closed
   decoded-response schema. No advertised EdDSA or self-attestation branch
   remains for implementation to invent.
5. **Callable admin DTO closure.** Added exact engine operation names/signatures,
   CLI-to-engine mappings, closed request/succeeded/refused DTO JSON Schema
   `handbook.schemas.security.approver-admin-api@1.0.0`, six schema-valid JSON
   vectors, approval-policy dependency closure, and the sixteenth profile schema
   source. The prior Review 6 prose claim is now backed by normative bytes.
6. **Exact commit markers.** Registry, standalone lifecycle, and promotion
   journals now write exactly the 72 ASCII bytes
   `sha256:<64 lowercase hex>\n`, where the hash covers exact raw `intent.json`
   bytes. Matching is byte equality; missing LF, extra bytes, uppercase hex,
   truncation, or parsed-value hashing refuses.

The exact-byte cascade regenerated ten schema entries, all eleven definition
and JCS vectors, approval/waiver/lifecycle/kind/intake/profile fingerprints, the
literal intake's own fingerprint and raw SHA-256, thirteen runtime identity
vectors, and seven authenticator transcripts. Released definition bytes remain
unchanged and no product implementation was performed.

## Required replay before Review 8

The parent must reproduce ten schema documents/entries, eleven definition/JCS
vectors, thirteen runtime records with six published-schema validations, all six
admin API vectors, seven registration/assertion transcripts, strict decoded CBOR,
content-addressed bootstrap refs, distinct add-enrollment equality, all 16
ordered selected-profile schema sources, exact intake/raw identity, registry/
promotion/lifecycle marker and recovery contracts, links/tasks/diff/scope, all
handoff modes, retained schema tests, and focused baselines. It must then seal a
new complete immutable subject and use a different fresh read-only reviewer.
Only a later `CLEAN` result over unchanged bytes can authorize the primary
planning commit.
