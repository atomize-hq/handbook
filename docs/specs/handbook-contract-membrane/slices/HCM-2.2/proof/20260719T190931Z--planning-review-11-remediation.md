# HCM-2.2 Planning Review 11 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_11`
- Dispatch:
  `20260719T185028Z--HCM-2-2--fresh-planning-review-11.json`
- Reviewed subject: 47 paths,
  `sha256:c0e7255568a751a16a0d498d15c5322d99e5cc618074dd56cd68fe399e13b429`
- Result: `CHANGES_REQUIRED`
- Required findings: 3
- Optional findings: 0
- Nit findings: 0

Review 11 authorizes no commit.

## Exact remediation

1. **One make-credential response partition.** Replaced the stale
   `authenticator-responses/` registration target with the sole admitted
   `authenticator-make-credential-responses/` partition. Regenerated the exact
   registration, registry-state, and bootstrap-transition fingerprints/IDs/
   refs while preserving the response bytes and fingerprint. Cross-record
   replay now requires the registration ref to resolve the same exact response
   identity installed and exposed by the journal/API path.
2. **Complete security-limit closure.** Added the missing exact-const
   `allowed_algorithms` collection from both registration-request branches.
   The fixture now publishes 30 semantic field/pointer groups and carries the
   accepted `['ES256']`, changed `['RS256']`, and one-over
   `['ES256', 'ES256']` values for explicit schema replay.
3. **Cross-family authenticator-use recovery.** Made registry and approval
   journals one recovery domain under promotion then registry locks. Every
   approval, registry-admin operation, and promotion discovers registry then
   approval paths in fixed lexical order, validates a shared predecessor graph,
   and recovers the unique current-head successor before observation, platform
   I/O, or a new intent. Froze exact approval/use intent bindings, the
   cross-family advanced-head table, committed-descendant behavior, duplicate-
   prior fork refusal, and approval-crash-to-admin/promotion plus admin-crash-
   to-approval/promotion fault proof. Recovery never restores a head older than
   the committed chain.

No security-schema or definition byte changed in this remediation, so the ten
schema-entry and eleven definition/JCS fingerprints remain fixed. Three runtime
record identities changed solely because the corrected response ref is semantic.
No product implementation was performed.

## Required replay before Review 12

The parent must reproduce ten schema documents/entries, eleven definition/JCS
vectors, thirteen runtime identities with the corrected response-registration-
state-transition chain and six schema validations, twelve admin positives and
fifteen negative refusals, all 30 security limit groups including exact/changed/
one-over algorithm values, both user-handle derivations, seven transcripts,
strict CBOR/P-256 and real registry-resolved DER ES256 verification, all 16
profile schema sources, exact intake/raw identity, the shared cross-family
recovery domain and table, links/tasks/diff/scope, all handoff modes, dependency
object, retained tests, and focused baselines. It must then seal a new immutable
subject and use a different fresh read-only reviewer. Only a later `CLEAN`
result over unchanged bytes can authorize the primary planning commit.
