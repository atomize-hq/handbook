# HCM-2.2 Planning Review 8 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_8`
- Dispatch:
  `20260719T172911Z--HCM-2-2--fresh-planning-review-8.json`
- Reviewed subject: 39 paths,
  `sha256:fe3981a7ab00d76c61e10fb290f0b6fd62705b7372fca07d6ed8ed184da502e6`
- Result: `CHANGES_REQUIRED`
- Required findings: 4
- Optional findings: 0
- Nit findings: 0

Review 8 authorizes no commit.

## Exact remediation

1. **Representable platform refusal.** Added literal
   `AUTHENTICATOR_UNAVAILABLE` to the closed approver-admin refused-result
   union and published a seventh schema-valid result vector with null result
   identities and an empty `changed_paths` list. The specification and future
   tests require fail-before-registration/assertion/journal/record/registry/
   lifecycle/canonical delta.
2. **Distinct add-enrollment keypair.** Replaced the add-enrollment bootstrap-key
   reuse with a second valid P-256 ES256 keypair and regenerated its COSE key,
   authenticator data, attestation object, response ID, and response
   fingerprint. Replay decodes both keys, proves both on-curve, and requires
   credential, encoded-key, and coordinate inequality.
3. **Cryptographically valid runtime assertion.** Replaced the unregistered
   credential selector and truncated placeholder signature with the exact active
   bootstrap credential hash and a complete DER ES256 signature over the frozen
   `authenticatorData || clientDataHash` preimage. The assertion identity was
   regenerated. Replay resolves the active registry credential, decodes its
   COSE P-256 public key, and performs real signature verification.
4. **Global authority lock order.** Froze promotion-then-registry-then-lifecycle
   acquisition, reverse release, same-order recovery, and fail-before-call/
   intent behavior. Approval retains one canonical/candidate/definition/
   registry-pair observation while holding promotion+registry through the
   challenge, authenticator verification, recheck, and approval append.
   Registry mutation holds promotion+registry through commit; promotion holds
   all three through commit/finalization and binds/rechecks the retained
   registry pair. Reverse acquisition is forbidden and concurrency, recovery,
   starvation, and deadlock proof are mandatory.

The admin-schema byte change regenerated ten schema entries, all eleven
definition/JCS vectors, approval/waiver/lifecycle/kind/intake/profile
fingerprints, the intake's own fingerprint and raw SHA-256, and the exact
closure table. Thirteen runtime vectors and seven authenticator transcripts
remain planning fixtures only. No product implementation was performed.

## Required replay before Review 9

The parent must reproduce ten schema documents/entries, eleven definition/JCS
vectors, thirteen runtime records with six published-schema validations, all
seven admin API vectors, seven registration/assertion transcripts, strict CBOR
and two distinct on-curve P-256 keys, the registry-resolved DER ES256 signature,
all 16 ordered profile schema sources, exact intake/raw identity, the global
lock/retained-observation contract, links/tasks/diff/scope, all handoff modes,
the dependency Git object, retained schema tests, and focused baselines. It must
then seal a new complete immutable subject and use a different fresh read-only
reviewer. Only a later `CLEAN` result over unchanged bytes can authorize the
primary planning commit.
