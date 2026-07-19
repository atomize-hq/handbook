# HCM-2.2 Planning Review 9 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_9`
- Dispatch:
  `20260719T180228Z--HCM-2-2--fresh-planning-review-9.json`
- Reviewed subject: 41 paths,
  `sha256:e7a7a333266440547cf08bb5fc0dfd25f757efc01979c2c8733fcc6427fbdf9c`
- Result: `CHANGES_REQUIRED`
- Required findings: 2
- Optional findings: 0
- Nit findings: 0

Review 9 authorizes no commit.

## Exact remediation

1. **Engine-owned authenticator ceremony.** Kept the four one-shot public
   engine operations but changed every request branch to caller intent only:
   no registration/assertion ref, fingerprint, file, raw response, or semantic
   record is admitted. Froze the blocking engine-owned
   `NativeAuthenticatorPortV1` raw make-credential/get-assertion methods and
   response bytes, the exact in-call ceremony/transaction state machine, and
   the adapter prohibition on constructing security semantics. Bootstrap runs
   registration; add runs registration then a current-admin assertion; revoke
   and update run assertion. The engine constructs all JCS, records, refs, and
   transaction bytes, and every pre-prepare refusal discards ceremony state
   before any semantic record or filesystem delta.
2. **Operation-by-status/refusal closure.** Replaced the status-only result
   branches with exact bootstrap/add/revoke/update success and refusal branches.
   Each success freezes prior, registration, assertion, result, transition,
   refusal, and ordered changed-path null/non-null/arity invariants. Every
   refusal requires every semantic identity null and `changed_paths: []`, with
   an operation-specific refusal-code set. Published twelve positive request/
   result vectors and twelve mandatory adversarial rejection vectors covering
   missing/forbidden identities, prior-state crossings, unavailable artifacts,
   non-empty refusal changes, non-null success refusal, and path arity.

The admin-schema byte change regenerated ten schema entries, all eleven
definition/JCS vectors, approval/waiver/lifecycle/kind/intake/profile
fingerprints, the intake's own fingerprint and raw SHA-256, and the exact
closure table. Runtime and authenticator transcript fixtures were unchanged.
No product implementation was performed.

## Required replay before Review 10

The parent must reproduce ten schema documents/entries, eleven definition/JCS
vectors, thirteen runtime records with six published-schema validations, all
twelve admin positive vectors and rejection of all twelve negative vectors,
the caller-intent/no-semantic-ref request boundary, engine-owned raw native port
and state machine, seven registration/assertion transcripts, strict CBOR and
distinct on-curve P-256 keys, the registry-resolved DER ES256 signature, all 16
profile schema sources, exact intake/raw identity, global locks, links/tasks/
diff/scope, all handoff modes, dependency object, retained schema tests, and
focused baselines. It must then seal a new complete immutable subject and use a
different fresh read-only reviewer. Only a later `CLEAN` result over unchanged
bytes can authorize the primary planning commit.
