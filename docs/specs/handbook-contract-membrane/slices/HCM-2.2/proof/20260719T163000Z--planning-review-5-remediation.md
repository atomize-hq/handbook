# HCM-2.2 Planning Review 5 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_5b`
- Dispatch:
  `20260719T154513Z--HCM-2-2--fresh-planning-review-5.json`
- Reviewed subject: 27 paths,
  `sha256:4ced6c6821a5702469b6ea080a484344fcf1347abc7b4ae20c7c8da4a35f8fb2`
- Result: `CHANGES_REQUIRED`
- Required findings: 6

The first assigned Review 5 agent did not complete; the parent stopped it and
used a new fresh replacement reviewer over the unchanged immutable dispatch.
The parent accepted all six replacement-review findings. No Review 5 result
authorizes a commit.

## Exact remediation

1. **Complete runtime identity records.** The twelve runtime vectors now use
   exact retained field names (including `intake_record_id` and
   `record_fingerprint`), contain complete records, and distinguish in-record
   IDs from registry state's derived path ID. Registry state, transition,
   registration, assertion, and trigger evidence validate against their exact
   published schemas. The other seven equal the retained `05` base plus only
   frozen HCM-2.2 additive fields.
2. **One bootstrap grammar.** The sole exact command is repeated
   `--initial-charter-quorum <class>=<authority>`. Empty pairs, empty quorum,
   and duplicates refuse; pairs sort class then authority and the engine adds
   the fixed admin mapping only after validating the explicit quorum.
3. **Unambiguous approval pair selection.** Approve now requires both untrusted
   `--approval-class` and `--authority-ref`. The engine re-derives and validates
   the pair against current required pairs, registry mapping, and the asserting
   credential before binding the pair into a fresh challenge.
4. **Closed CTAP2 transcripts.** Added exact registration-request and
   operation-specific challenge schemas plus six literal transcript vectors for
   bootstrap, all three registry mutations, initial approval, and amendment
   approval. The vectors freeze decoded objects, RFC 8785 JCS bytes,
   client-data hashes, authenticator data, and signed preimage bytes/hashes.
   Registration/assertion records bind the exact decoded-shape schema refs.
5. **Assurance terminology.** The remaining plan overclaim now uses exact
   `fido2_user_verified_repository_approver` wording and explicitly disclaims
   hardware provenance and real-world identity.
6. **Total lifecycle recovery.** Standalone lifecycle events now have exact
   journal steps and a total observed-state/marker precedence table. Every
   lifecycle writer recovers pending work before creating an intent, so an
   advanced current head with an older pending intent is a durability violation,
   not an implementation-selected retry/orphan branch.

## Required replay before Review 6

The parent must replay exact schema raw bytes/entries, eleven definition/JCS
vectors, all twelve complete runtime vectors with five published-schema
validations, all six transcript vectors with decoded-schema validation,
selected-profile source equality, intake/raw identity, links/tasks/diff/scope,
all handoff modes, retained profile-schema tests, and focused current baselines.
It must create a new immutable complete-subject dispatch and use another fresh
read-only reviewer. Only `CLEAN` over unchanged bytes can authorize the primary
planning commit.
