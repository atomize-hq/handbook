# HCM-2.2 Planning Review 10 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_10`
- Dispatch:
  `20260719T181712Z--HCM-2-2--fresh-planning-review-10.json`
- Reviewed subject: 43 paths,
  `sha256:1775446e5431b8cb62533c52efc478fcbaf4e7ff2cbe7fd8906d1aafd4051d87`
- Result: `CHANGES_REQUIRED`
- Required findings: 6
- Optional findings: 0
- Nit findings: 0

Review 10 authorizes no commit.

## Exact remediation

1. **Complete registry record-set authority.** Made each registry journal
   operation-discriminated and bound every response, registration, assertion,
   state, transition, authenticator-use transition, and authenticator-use head
   path/fingerprint/byte required by that operation. Recovery now handles an
   arbitrary proper subset, never treats an uncommitted ceremony record as
   authority, restores a replaced use head on rollback, and requires crash
   injection around partial ceremony-record installation. Bootstrap and add
   success `changed_paths` now begin with the committed make-credential
   response path and contain four and five exact paths respectively.
2. **Opaque native CTAP boundary.** Replaced JCS-plus-partial-response port
   methods with opaque request-CBOR/response-CBOR transports. The engine builds
   the complete exact CTAP2 command maps, including client-data hash, RP, user,
   ES256 parameters, allow list, and options; it alone parses raw responses and
   creates semantics. Fake-port proof must capture exact command bytes and
   prove that no candidate, authority, approval, registry, or record identity
   crosses the boundary.
3. **Literal challenge key sets.** Added `$schema` to both exact challenge-key
   lists so the prose, schema, and seven transcript vectors agree.
4. **First-admission resource bounds.** Bounded every security-schema
   collection and every `_base64` field by `maxItems`/`maxLength` or an exact
   const value. Published 29 machine-checkable semantic-field/pointer groups in
   `security-boundary-limit-vectors-v1.0.json`, with exact accepted and one-over
   sizes. The admin vectors add one-over mappings, quorum, and next-actions
   negatives, for twelve positive and fifteen mandatory negative vectors.
5. **User-handle derivation.** Froze
   `SHA-256(UTF-8(exact lowercase repository fingerprint including sha256:))`,
   the CTAP user-ID and JSON hash encodings, two exact vectors, and five
   rejected alternate transformations.
6. **Shared replay/counter authority.** Added one canonical per-credential use
   head and immutable bounded use-transition chain shared by approvals and
   registry administration. Froze sequence/counter rules, full-chain nonce and
   challenge replay checks, the 4096-use bound, approval and registry journal
   inclusion, global lock ownership, interleaved-use proof, and crash/orphan/
   recovery behavior.

Security-schema byte changes regenerated the complete ten-entry schema closure,
all eleven normalized definition preimages, approval/waiver/lifecycle/kind/
intake/profile fingerprints, the intake's own fingerprint and raw SHA-256, and
the exact closure table. Existing runtime and authenticator transcript fixtures
remain unchanged. No product implementation was performed.

## Required replay before Review 11

The parent must reproduce ten schema documents/entries, eleven definition/JCS
vectors, thirteen runtime records with six published-schema validations,
twelve admin positives and rejection of all fifteen negatives, all 29 security
limit groups, two user-handle derivations and five negative transformations,
four/five ordered bootstrap/add changed paths, seven registration/assertion
transcripts, strict CBOR and distinct on-curve P-256 keys, the registry-resolved
DER ES256 signature, all 16 profile schema sources, exact intake/raw identity,
opaque-port/no-semantic-leakage wording, shared use authority, global locks,
links/tasks/diff/scope, all handoff modes, dependency object, retained schema
tests, and focused baselines. It must then seal a new complete immutable
subject and use a different fresh read-only reviewer. Only a later `CLEAN`
result over unchanged bytes can authorize the primary planning commit.
