# HCM-3.2 fresh JCS capsule and replacement-recovery plan

Status: planning subject awaiting independent v1.4 review. No test, RED, or
Rust edit is permitted before planning CLEAN.

## Identity and objective

- parent: `20260804T124829Z--HCM-3-2--context-resolution-jcs-recovery`
- outcome: `hcm-3.2-context-resolution-jcs-recovery-whole-slice`
- checkpoint: `9368b5cee612bdb1ffd25562eef35e8d863ff930`
- publication: expected-old local-only compare-and-swap; never push

Complete HCM-3.2 through the reviewed private JCS capsule, real generic owner
pipeline, authenticated publisher proof, current operational admission,
minimum replacement/recovery floor, unchanged ordered-stack kernel behavior,
full proof, independent review, and two-commit v1.4 closeout. Prior stopped
parents and findings are source constraints only; their identities are not
continued, superseded, or reset.

## Frozen architecture

1. Generic intake carries one required string at `/declaration_jcs` and proves
   wrapper shape/provenance only.
2. The private HCM gate alone validates the exact nine-member RFC 8785/JCS
   declaration, inner 53-pointer payload, compatibility tuple, layer-specific
   bounds, three preserved generic fingerprint algorithms, and three exact
   tagged/length-prefixed HCM fingerprint domains.
3. Generic lineage supplies atomic complete-wrapper authorship/replacement.
   There is intentionally no per-leaf provenance, partial update, query, or
   generic whole-document framework.
4. Publication authority is external: an exact authenticated committed
   registry transition and closed nine-member publisher object authorize the
   exact outer output before generic promotion. The result preserves all prior
   mappings and adds one exact publication mapping. The real pipeline starts at
   an empty/current selected target and never direct-seeds authority.
5. Historical committed proof, publication-authorized pending commit, current
   authority witness, and corrupt/ambiguous state are private and cannot be
   interchanged. Historical and pending proof never become operational.
6. Every admission consumption revalidates live direct-successor registry head,
   credentials/mapping, committed generic lineage, predecessor, repository,
   profile, stack, bytes, and fingerprints.
7. Replacement preserves historical verification, converges each crash
   boundary by exact retry or bounded HCM quarantine, uses pending non-capability
   state until journal commit, and discovers an H2-to-T2 no-journal gap from the
   live registry head plus the already committed candidate. The exact retry key
   is `hcm32crpub_<outer64hex>` under registry-before-generic lock order.
8. Once H2 authorizes O2 and before T2 commits, neither O1 nor O2 is
   operational. Failed promotion never reactivates O1.
9. Staged H2/H3 continuous availability is documented future-only and is not
   implemented.

## Ordered packets

### P1 — selector authority

- verify identity, exact base/ref/tree/ancestor/remote, worktree cleanliness,
  protected-root observations, and live GitNexus availability;
- carry all historical findings and adversarial-review results as known
  constraints;
- align SPEC, selector, plan, checklist, and future-work decision;
- freeze exact public call surface, paths, named symbols/risks, line/test
  ceilings, proof wall, and stop conditions;
- obtain an ordinary fresh planning discovery review and, if necessary, one
  consolidated remediation and different-fresh closure.

Checkpoint: planning stage is independently CLEAN before any test or Rust edit.

### P2 — capsule, publisher authority, and current kernel

- rerun upstream impact immediately before every existing-symbol edit and warn
  for every reviewed HIGH/CRITICAL result;
- write focused RED tests for capsule canonicalization/fingerprint separation,
  generic/HCM golden vectors, layer-specific limit boundaries, exact publisher
  object/challenge/assertion/mapping, real owner-path publication, currentness,
  cache/clone refusal, and kernel behavior;
- implement the private bounded codec and semantic gate in
  `context_resolution.rs`;
- integrate the HCM-only publisher proof into `candidate_preview`, promotion
  request/plan validation, persisted-output validation, and the unchanged-
  signature mutation service entry points;
- revalidate current witness on every resolver/admission/envelope/mutation/
  memory/validation/escalation/transition consumption;
- preserve shipped HCM-3.1 definition/profile/vocabulary bytes and the exact
  public kernel signatures.

Checkpoint: focused capsule/kernel/promotion tests pass and no unreviewed
surface or risk has appeared.

### P3 — replacement and crash recovery

- write RED recovery tests for O1 -> O2 -> O3, historical displaced proof,
  installed-before-marker recovery, pending non-capability, H2-to-T2 cold
  restart without a request, exact retry/concurrent conflict, transition-keyed
  quarantine, and every supported fault boundary;
- distinguish current canonical verification from exact historical displaced
  verification through one unique direct successor;
- reconstruct a truthful markerless installed result only from durable facts;
- acquire promotion/registry locks before the generic evaluation lock and
  share currentness/effect linearization under that retained order;
- classify the exact current transition as committed, pending, retry-available,
  or one of the closed quarantine reasons by replaying zero/one/multiple HCM
  candidate closures;
- quarantine irrecoverable HCM pending state without weakening unrelated
  generic recovery, registry ownership, unknown-family refusal, or non-HCM
  validation.

Checkpoint: every crash converges or is diagnosably fail closed; installed-
uncommitted bytes and all historical/pending proof remain non-authoritative.

### P4 — proportional proof and causal review

- run focused negative proof, engine and lineage caller/process regression,
  full workspace all-target/all-feature tests, strict Clippy, rustfmt, diff and
  whitespace gates;
- replay exact shipped definitions/profile/vocabulary and unchanged L0-L3
  behavior;
- run ordinary handoff validation and both orchestration self-tests;
- run GitNexus scoped and compare-to-main detection before the primary commit;
  unavailable FTS/comparison remains unavailable, never GREEN;
- obtain fresh complete implementation and proof reviews, remediate/close only
  within the live causal budget, and leave no unresolved P1/P2.

Checkpoint: complete code-bearing subject and proof are independently CLEAN.

### P5 — two-commit closeout and local publication

- commit the reviewed primary state/stack;
- create the completed v1.4 parent handoff against the primary tip, rebuild the
  ledger, validate ordinarily and with both self-tests;
- commit mechanical closeout only in a separate commit;
- recheck the integration ref equals the expected base, atomically update it to
  the closeout commit, and verify final ref/tree, remote baseline, protected
  paths, clean worktree, and no push;
- send one structured terminal receipt to the bound meta task as the final tool
  action.

## Closed bounds

Absolute production ceiling: `context_resolution.rs`,
`context_resolution_registry.rs`, `artifact_mutation.rs`,
`artifact_lineage_store.rs`, and `lib.rs`. Only the first, third, and fourth are
planned code-bearing paths. Maximums: 80 changed production declarations,
2,400 hand-written production lines with selector-bound per-file allocation,
2,400 focused test/fixture lines, and 18 material test functions.

The selected live-risk seams are the four CRITICAL functions
`validate_persisted_output_authority`, `validate_request_subject`,
`GenericArtifactLineageStoreV1::recover_pending`, and
`GenericArtifactLineageStoreV1::verify_committed`, plus the exact lower-risk
callers named in the selector. Any additional HIGH/CRITICAL symbol, sixth
production path, new public API, owner, dependency/crate, unsafe/native/
platform/transport surface, shipped identity mutation, HCM-3.3+, or generic
framework is a stop before edit.

## Definition of done

All five packets complete monotonically; planning, implementation, proof, and
final-closeout reviews end CLEAN with no unresolved P1/P2; the minimum
replacement floor is proved through the real owner pipeline; primary and
mechanical closeout commits are distinct; expected-old local CAS succeeds;
remote/protected paths remain unchanged; and the bound receipt is delivered.
If the floor cannot fit the bounds, stop durably rather than infer bootstrap-
only scope or reduce the replacement claim.
