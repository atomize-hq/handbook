# HCM-2.2 Planning Review 6 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_6`
- Dispatch:
  `20260719T163123Z--HCM-2-2--fresh-planning-review-6.json`
- Reviewed subject: 32 paths,
  `sha256:bf48190166092b6b67caa73ff29689bc1534afc07fb3305bf21c443e27d149d8`
- Result: `CHANGES_REQUIRED`
- Required findings: 4
- Optional findings accepted: 1
- Nit accepted: 1

Review 6 authorizes no commit.

## Exact remediation

1. **Bootstrap transition authorization.** Transition authorization is now a
   closed conditional union: bootstrap binds its exact verified registration
   with `authorization_kind: registration` and null prior/admin fields; add,
   revoke, and update bind a fresh current-admin assertion with every prior/admin
   field non-null. The bootstrap runtime vector is a coherent registration →
   registry state → transition chain and contains no impossible assertion ref.
2. **Protocol-valid make-credential response.** Added a decoded-response schema
   and complete response runtime vector. The literal `fmt: none` attestation
   object is valid CBOR, carries empty `attStmt`, and embeds authenticator data
   with matching RP hash, `UP|UV|AT`, zero counter, AAGUID, credential ID, and a
   complete ES256 COSE key using the P-256 base point. Registration, decoded
   response, registry credential, and bootstrap transition fingerprints/bytes
   cross-match. The packet requires strict CBOR/trailing-byte/AT/key/counter/
   credential cross-field negatives.
3. **Callable registry administration.** Added exact typed engine use cases and
   CLI/JSON grammars for add credential, revoke credential, and update mapping.
   Add has a separate closed make-credential enrollment request and transcript;
   its candidate registration is unauthoritative until a current admin assertion
   over the derived result state commits the transition.
4. **Total registry recovery.** Registry writers now recover every pending
   journal before admitting a new intent. Exact steps and a total observed-state
   table cover empty/partial intent, staged records, one/both finals, marker
   disagreement, advanced head, bootstrap races, and mismatched bytes. Final
   content-addressed records are never deleted.
5. **Lockout invariant.** Every result state must retain an active admin and an
   active credential covering each immutable quorum/current Charter-required
   pair; a revoke/update that removes the last coverage refuses before staging.
6. **Source-count correction.** The profile prose now says seven security schema
   sources, matching the complete selected list.

## Required replay before Review 7

The parent must reproduce nine schema documents/entries, eleven definition/JCS
vectors, thirteen runtime records with six published-schema validations, seven
registration/assertion transcripts, decoded CBOR and cross-record bootstrap
coherence, all 15 ordered selected-profile schema sources, exact intake/raw
identity, registry and lifecycle recovery tables, CLI grammar uniqueness,
assurance/lockout scans, links/tasks/diff/scope, handoff modes, retained schema
tests, and focused baselines. It must then seal a new complete immutable subject
and use another fresh read-only reviewer. Only a later `CLEAN` result over
unchanged bytes can authorize the primary planning commit.
