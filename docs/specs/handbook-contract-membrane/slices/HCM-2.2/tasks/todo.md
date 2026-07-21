# HCM-2.2 Implementation Checklist

**Status:** the atomic-stage implementation selected from prior review-clean
planning authority reached fresh Review 3 and was rejected under
`HCM-2.2-AR3-001` because result, witness, and mutable sidecar binding bytes
could be coherently rewritten without changing candidate `1.2` identity.
User-selected `HCM-2.2-ESC-003` authorizes only a documentation/control-pack
repair selecting candidate `1.3` as the independent downstream exact-result
anchor. The rejected implementation is preserved untouched at
`486458acfe8373a977e595a4854ac786166e3e76` and remains non-authoritative
evidence. Do not resume Rust, mark implementation items complete, or start
HCM-2.3 until this complete repaired subject receives fresh `CLEAN` review and
parent closeout.

The immutable escalation handoff's `status/plan.md` and `status/todo.md` names
are erroneous. The authoritative mutable paths are this `tasks/todo.md` and
[`tasks/plan.md`](plan.md); the immutable record remains unchanged.

## Candidate 1.3 exact-result authority repair

- [x] Inventory all 1,464 tracked, 13 ordinary-untracked, and 25,235 ignored-
  untracked paths in
  `C:\hcm22ar` at exact HEAD
  `486458acfe8373a977e595a4854ac786166e3e76`; preserve all 26,712 files in a
  deterministic manifest and external recoverable archive; verify every
  extracted source entry byte-equal and source inventory stable.
- [x] Record manifest SHA-256
  `cbcb29fe7ef50184cbc2efcaa9489d07c99d0d4b51c7e92836e7e2cc3d1cf44a`
  and archive SHA-256
  `46dbed286dba9b3b6cedf7ef13340c43dcd56e068a5efa9af1deba0cabadd33d`;
  leave the rejected worktree HEAD/files/index/staging/status untouched.
- [x] Create clean worktree `C:\hcm22ar-doc-repair` and branch
  `codex/hcm-2-2-exact-result-authority-repair` from the exact baseline.
- [x] Preserve byte-for-byte the implementation proof wall, Review 1--3
  dispatches, Review 1/2 remediation proofs, Review 3 stop proof,
  `HCM-2.2-AR3-001` coherent-rewrite RED result, and the original non-
  authoritative anchor research note in the complete archive. Permit only the
  reviewed provenance/link correction in the documentation copy; do not
  rewrite immutable handoffs, dispatches, reviews, or proofs.
- [x] Select `HCM-2.2-ESC-003-D1`: candidate `1.3` keeps the complete
  fourteen-field subject fingerprint independent of result binding and adds one
  closed final-identity binding with exact result ref, semantic fingerprint,
  exact persisted JCS+LF SHA-256, and LF-inclusive byte length.
- [x] Keep result `1.0` binding only candidate-subject fingerprint; include the
  complete exact-result binding in final candidate identity; bind new approval
  to final candidate; bind promotion and intent `1.2` transitively; prove the
  topological graph is acyclic; add no result self-hash or mutable companion
  authority.
- [x] Freeze author-lock replay: independently recompute subject and expected
  semantic result authority without `validated_at_utc`; discover by their pair;
  admit zero/no-result first authoring; admit exactly-one only after candidate
  identity and exact digest/length; refuse more-than-one, unsafe, crossed,
  malformed, missing, rewritten, or mismatching authority before mutation.
- [x] Bound replay inventory to the exact candidate/result roots, 4,096
  immediate entries per root, 262,144 bytes per file, 1,073,741,824 aggregate
  bytes per root, 128 UTF-8 filename bytes, exact 79/97-byte ASCII grammars,
  and two equal complete no-follow scan tuple vectors; refuse every overflow,
  unsafe/unowned entry, or scan change before mutation.
- [x] Classify result-without-matching-candidate as an orphan; preserve and
  refuse; permit no adoption, overwrite, repair, wrapper, result-ref copy,
  deletion, or automatic crash completion. Require a separately reviewed
  author-publication transaction if automatic completion is later desired.
- [x] Freeze migration preserving candidate `1.0`, `1.1`, and `1.2` plus their
  approvals as immutable evidence while requiring `1.3` reauthoring, result
  recomputation, and new approvals; admit no dual read, upgrade, fallback,
  result-ref copy, or approval carry-forward.
- [x] Update authority-repair, runtime-record, and promotion-intent vectors with
  exact candidate/result/approval/promotion/intent identities, raw-document
  bindings, zero/one/many/orphan/migration matrices, and every required
  negative while keeping result `1.0` and intent `1.2` schemas byte-unchanged.
- [x] Add the exact decision and planning proof, and update SPEC, `05`, plan,
  checklist, and necessary mutable `00`/`03`/`04`/`06` surfaces without Rust or
  HCM-2.3 edits.
- [x] Freeze the review protocol: each immutable exact-subject dispatch goes to
  a different fresh isolated read-only reviewer over identity, persistence,
  replay, orphan, migration, promotion, recovery, vectors, negatives, and
  control-pack truth; accept every finding without waiver and use a different
  fresh reviewer after every remediation until `CLEAN`. The final dispatch and
  parent handoff, rather than this mutable checklist, record the outcome.
- [x] Freeze the closeout order: only after `CLEAN`, run all documentation/
  schema/vector/link/handoff/GitNexus/scope gates; create the scoped authority-
  repair commit; then add only the parent handoff and deterministic ledger in a
  separate closeout commit. Report the exact implementation selector and stop
  before Rust and HCM-2.3.

## Retained atomic whole-file stage authority repair

- [x] Inventory the exact dirty worktree at entry HEAD
  `4e164061e18da17cc24d576a26800ab1ecce69c2`; verify no deletions; archive all
  13 modified/untracked paths outside the repository; verify archive SHA-256
  `29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e`
  and extracted-byte equality; leave the dirty worktree untouched.
- [x] Preserve the authentic-byte-source research note as non-authoritative
  decision input and select Option A without importing Rust into this clean
  authority-repair worktree.
- [x] Keep `promotion-transaction-intent` `1.2` schema byte-for-byte unchanged;
  preserve the candidate-`1.2` amendment intent vector as historical evidence,
  and freeze the selected candidate-`1.3` amendment intent fingerprint,
  7,591-byte document SHA-256, and 72-byte marker.
- [x] Freeze exact `.output-staging/` path, 128-bit random purpose-typed scratch
  names, same-filesystem/create-new/no-follow/bounded-byte rules, retained exact-
  byte and type-specific verification, atomic rename-no-replace, ordered
  pending/scratch/transaction-parent fsyncs, and pending-stage reverify.
- [x] Replace only `canonical.new`, `promotion-record.new`, and
  `lifecycle-transition.new` authoritative states with
  `{absent, exact, mismatch}`; partial pending stages always preserve/refuse.
  Retain `canonical.old` and marker-temp prefix rules because each has an
  authentic complete source.
- [x] Repair `W5`-`W7`, `R0`/`R4`-`R6`, rollback-progress construction,
  recovery partition, state axes, negative vectors, and mismatch wording so
  scratch is never scanned, classified, deleted, or used as authority. Prove
  every writer/discovery/recovery/finalization pending path uses only the
  validated `transaction_id`, including unequal-`promotion_id` refusal.
- [x] Freeze the exact three-purpose by `S0`-through-`S11` fault Cartesian
  product, rejecting every missing or duplicate pair, plus crash injection
  after retained-byte/binding acquisition, every scratch create, bounded write
  prefix, file fsync, close, no-follow
  reopen, exact verification, scratch-dir fsync, atomic rename, pending-dir
  fsync, post-rename scratch-dir fsync, transaction-parent fsync, and exact stage
  reverify. Assert pre-rename stage absence and post-rename absent-or-exact.
- [x] Update the coupled `00`, `03`, `04`, `05`, `06`, SPEC, vector, plan,
  checklist, research, and additive proof surfaces only; preserve all immutable
  handoffs/dispatches/reviews/proofs and every Rust/test byte.
- [x] Validate JSON/Draft 2020-12, unchanged intent identity, matrix closure,
  links, handoffs/self-tests, whitespace, secrets, and documentation-only scope.
- [x] Dispatch the first exact 21-path subject to a fresh isolated reviewer;
  accept both Required findings and the Nit without waiver; repair promotion
  intent publication, `S0` fault closure, and research paths; record the
  additive Review 1 remediation proof.
- [x] Dispatch the Review 1-remediated exact 22-path subject to a different
  fresh isolated reviewer; accept its Required pending-path identity finding
  without waiver; repair the sole stale promotion-ID placeholder, add the
  unequal-ID negative, and record the additive Review 2 remediation proof.
- [x] Dispatch the Review 2-remediated exact 23-path subject to a third
  different fresh isolated reviewer; accept its Required terminal-marker
  durability finding without waiver; add symmetric pending-directory-fsync/
  payload-revalidation replay and eight crash pairs; record the additive
  Review 3 remediation proof.
- [x] Dispatch the Review 3-remediated exact 24-path subject to a fourth
  different fresh isolated reviewer; accept its Required rollback-destination
  and Nit source-integrity findings without waiver; add symmetric four-state
  terminal destination axes/negatives, cite immutable pre-repair evidence, and
  record the additive Review 4 remediation proof.
- [x] Dispatch the Review 4-remediated exact 25-path subject to a fifth
  different fresh isolated reviewer; accept both terminal-label and historical-
  citation Nits without waiver; normalize every mutable normative terminal axis
  to the machine labels, pin lifecycle-authority research to immutable pre-
  repair Git objects, and record the additive Review 5 remediation proof.
- [x] Dispatch the Review 5-remediated exact 26-path subject to a sixth
  different fresh isolated reviewer; accept its planning-versus-implementation
  status Nit without waiver; attach `review-clean` only to the Option 1 planning
  handoff, keep the stopped implementation explicitly non-clean, and record the
  additive Review 6 remediation proof.
- [ ] Create an immutable exact-subject review dispatch and obtain fresh
  isolated read-only review. Accept findings without waiver and obtain a
  different-fresh re-review after every repair until `CLEAN`.
- [ ] After `CLEAN`, create the reviewed documentation-only primary commit and
  separate parent handoff/ledger closeout commit. Do not resume Rust or HCM-2.3
  in this orchestration.

## Retained candidate 1.2 authority-repair planning packet

The unchecked boxes in this retained section describe the historical packet as
it appeared before its later review/implementation sequence. They are not
current candidate `1.3` work and do not authorize candidate `1.2` fallback.

- [ ] Freeze candidate `1.2` with the complete fourteen-field subject preimage,
  exclusions exactly `{candidate_id, candidate_fingerprint,
  candidate_subject_fingerprint, validation_result_refs}`, and validation-only
  subject-identity classification.
- [ ] Freeze the acyclic subject -> subject fingerprint -> engine result/ref ->
  final candidate fingerprint/ID -> human approval -> promotion sequence and
  exact create/amend fingerprint vectors.
- [ ] Freeze lifecycle-validation-result `1.0` closed schema, exact preimage and
  ID/ref grammar, 262,144-byte JCS+LF persistence, no-follow/create-new/rename-
  no-replace-or-exact-equal replay, audit-only timestamp behavior, and no
  caller-authored authority.
- [ ] Freeze exact one-result cardinality; complete ordered thirteen-definition,
  active-observation, and reopened-coverage sets; intake/content/canonical-
  basis/profile/policy/head/state bindings; and every negative currentness
  refusal before mutation. Freeze exact committed lifecycle-head observation
  order, including a retained multi-observation head whose event order differs
  from ref order; ref-sorted, duplicate, incomplete, or excess arrays refuse.
- [ ] Freeze additive migration: preserve candidate `1.0`/`1.1`, checkpoint
  candidates, approvals, and implementation as evidence; require `1.2`
  re-author/re-evaluate and new approvals; admit no implicit dual read.
- [ ] Freeze complete promotion intent `1.2` closed schema and exact amendment
  fingerprint/document/marker vector, literal nested keys, whole-file intent
  publication from non-authoritative scratch, separate atomic whole-file
  canonical/promotion/lifecycle stage publication from `.output-staging/`,
  absent-or-exact authoritative new-output stages, fifteen-name pending grammar,
  ordered `W0`-`W15` cumulative writer boundaries, terminal name sets,
  independent canonical-old/new/record-stage and suffix axes, amendment prefix
  completion, finite crash-state domain, ordered `R0`-`R9` rollback-recovery
  boundaries with exact cleanup order/marker-prefix/pending-finalization/parent-
  fsync replay, symmetric committed-terminal rename-no-replace/collision refusal/
  parent-fsync replay, and exhaustive disjoint one-row recovery partition; any mismatch
  must preserve the entire journal and refuse without mutation.
- [ ] Freeze module-private `#[cfg(test)]` fault injection and production
  rustdoc/public-API/all-features absence proof.
- [ ] Reopen/correct the SPEC, `00`, `03`, `04`, PR-009, PG-INTAKE-01,
  PG-INTAKE-02, PG-CHARTER-01, and BR-HCM-2-CHARTER-FLOW-01 status surfaces.
- [ ] Validate JSON schemas/vectors, fingerprints, cross-record/negative
  matrices, links, ledgers, immutable-record scope, and no-Rust diff.
- [ ] Dispatch the complete revised planning subject to a fresh isolated
  reviewer; repair and repeat with another fresh reviewer until CLEAN.
- [ ] Run GitNexus detect-changes, create the primary reviewed documentation
  commit, add the parent planning handoff/ledger closeout, detect changes again,
  and create the mechanical closeout commit.
- [ ] Stop before Rust/implementation mutation and before HCM-2.3.

## Entry and authority

- [ ] Revalidate the selected HCM-2.1 closeout, planning closeout, clean implementation entry, and packet fingerprint.
- [ ] Reread the required skill chain and named `00`-`08`/HCM-0.6 sections.
- [ ] Prove setup create-news and preserves `.handbook/repository-identity.v1`, registry equality is fail-closed, reset-state cannot select it, and every approve/admin operation ID comes from the engine-owned frozen derivation with no caller/adapter identity input.
- [ ] Prove the exact typed operation-to-allocator-token table, direct-engine opaque-ID compatibility, both pre-invocation refusal branches and crossed-nullability negatives, the distinct closed unsafe-recovery refusal envelope, predecessor-ordered recovery before identity resolution, no new operation-owned delta beyond separately attributable recovery-table effects, and request/challenge/raw-intent/result/output equality without adding operation IDs to semantic authority records.
- [ ] Refresh GitNexus and run upstream impact before every existing-symbol edit.
- [ ] Refresh impact and warn before `evaluate_charter_intake` (known HIGH),
  generic lineage `validate_record` (known CRITICAL), the selected-profile
  resolver, and promotion recovery (known HIGH); prefer a dedicated lifecycle-
  validation service/store to avoid widening the generic validator.
- [ ] Capture the exact old definition/package/install manifest and staged allowed paths.

## RED proof

- [ ] Reproduce every literal schema document, normalized-JCS line, length, expected definition/profile fingerprint, old-byte closure, exact dependency ref/current-producer equality, acyclic graph, descriptor-only intake selection, package, and selected-profile vector before implementation.
- [ ] Add complete live-field crosswalk and canonical Charter schema/typed/capability positive and exhaustive negative tests.
- [ ] Add all-mode equivalence and sixteen-item non-overlapping coverage/source/bijection/unknown/contradiction/waiver RED tests.
- [ ] Add additive intake/approval/promotion `1.1`, candidate `1.3`, and lifecycle-validation-result `1.0` basis lineage plus exact result-document binding/replay/orphan closure and exact ES256/`fmt: none` authenticator bootstrap/registry/UV/UP/assertion/class/revocation, distinct add-enrollment credential and P-256 keypair, real registry-resolved DER signature verification, exact `AUTHENTICATOR_UNAVAILABLE` fail-before-delta result, caller-intent-only request DTOs, operation-by-status/refusal result DTOs and mandatory crossed-state rejections, forged mapping, stale authority, replay, and ABA RED tests; mechanically prove every current implementation task names candidate `1.3` and treats candidate `1.0`/`1.1`/`1.2` as historical-only.
- [ ] Add a retained lifecycle-head positive with at least two observations whose event-precedence order differs from ref order, plus ref-sorted/reordered, duplicate, incomplete, excess, and retained-head rewrite negatives.
- [ ] Add create-null versus mandatory-amendment-basis equality at every lineage
  stage plus the exact observed-state promotion recovery table, partial scratch
  writes with absent authoritative stage, partial pending-stage mismatch
  preservation, atomic exact record install, preexisting-equal reuse, commit-
  marker, concurrent-reader, and no-visible-partial-authority RED tests.
- [ ] Add literal canonical YAML/Markdown fixtures, independent emitter/renderer, and fingerprint RED goldens.
- [ ] Add lifecycle review/reassessment total-transition/precedence/fingerprint/targeted-reopen/no-auto-regeneration RED tests.
- [ ] Add compiler/CLI/skill/setup/doctor/Environment Inventory/flow/legacy-irrelevance RED tests.
- [ ] Add native non-Unix read-only and pre-mutation-refusal RED tests.

## Versioned definitions and canonical owner

- [ ] Add typed closed definition registries/meta-schema validation for intake, renderer, lifecycle, approval, waiver, triggers, and semantic validators.
- [ ] Add the exact HCM-2.2 definition refs and packet-published uniform fingerprints; refuse any reproduced-byte mismatch.
- [ ] Add Project Authority content schema/kind and compatible constitutional semantic validator `1.1.0` while retaining capability-required validator `1.0.0`, old bytes, and kind/intake acyclicity.
- [ ] Add standalone shipped profile `1.1.0` with only the exact Charter selection delta.
- [ ] Extend selected-profile resolution additively and replay setup/doctor/author/flow/preflight/CLI immediately.
- [ ] Land typed `CanonicalCharter`, complete retained-field crosswalk, duplicate-safe selected-schema parse, semantic-capability validation, and closed canonical emitter.
- [ ] Land retained no-follow observation plus exact source fingerprint.
- [ ] Land fixed in-memory Markdown renderer plus exact rendered fingerprint.

## Intake, candidate, approval, promotion, lifecycle

- [ ] Resolve intake by selected instance/kind exact refs, never filename/mode/prompt.
- [ ] Evaluate all sixteen coverage IDs with exact authority/source, non-overlap, leaf-source bijection, and value-versus-minimum-specificity-waiver rules.
- [ ] Preserve declarations, evidence, defaults, unknowns, contradictions, waivers, prompt events, and mode as typed provenance.
- [ ] Prove guided-adaptive, express, and agent-assisted produce the same candidate schema and equivalent bytes for identical inputs.
- [ ] Append additive `1.1` immutable intake records, then construct candidate
  `1.3` subject, engine-owned lifecycle-validation result `1.0`, exact persisted
  result JCS+LF binding, and final content-addressed candidate in the exact
  acyclic order with complete field-source mappings and frozen create/amend
  basis; admit no candidate `1.2` fallback or result-ref copy.
- [ ] Bootstrap and maintain immutable acyclic registry state/transition chains only through exact native CTAP2.1 registration-request/challenge/transcript contracts; validate the three closed challenge branches plus both crossed create/amend basis negatives before native I/O; freeze one non-empty quorum CLI grammar, require untrusted class+authority selectors for one typed `1.1` approval per exact pair, and reject unattended, file/stdin, caller-identity, unsigned, replayed, wrong-RP, or weaker fallback paths.
- [ ] Expose the four exact named bootstrap/add/revoke/update engine signatures and CLI-to-engine mappings through caller-intent-only requests, including a bootstrap branch with quorum but no caller mappings, an engine-owned opaque-CBOR `NativeAuthenticatorPortV1` transport, exact GetAssertion request key `2` equality to each current challenge JCS hash, byte-closed status-plus-canonical-CBOR response decoding with exact 37-byte `0x05` authenticator data, a source-audited total disjoint `0x01..0xff` status-to-refusal/retryability/non-empty exact-next-action mapping and Admin projection, deterministic eligible-credential filtering/raw-ID ordering/descriptor selection, and the operation-by-status/refusal closed JSON schema with thirteen positive and seventeen mandatory rejection vectors; bind bootstrap transition to its coherent decoded make-credential response/registration, bind add to its distinct enrolled credential and later transitions to fresh decoded GetAssertion response/admin assertion pairs, permit no caller/adaptor-supplied ceremony record/ref or bootstrap mapping, and refuse result states that lose final usable admin or required-pair coverage.
- [ ] Implement the exact repository-fingerprint-to-user-handle derivation vectors, all 33 first-admission collection/raw-input groups and exact/changed/one-over rejections, unique/canonical waiver-ref/fingerprint ordering and duplicate refusal, and the shared per-credential bounded authenticator-use head/transition chain across approval and registry-admin operations, including zero/nonzero sign-count rules, sequence-4095 final-use replacement/no-lockout behavior with exact add `lockout_refused` and byte-identical refusal pre/post authority, non-adjacent replay refusal, fixed complete-domain registry-then-approval discovery, predecessor-ordered cross-family advanced-head/fork recovery, approval-crash-to-admin/promotion and admin-crash-to-approval/promotion proof.
- [ ] Before any mutation, re-resolve every definition/profile/candidate/
  approval/target precondition plus exact subject/result, intake/content,
  lifecycle policy/head/state, complete active observations, and complete
  reopened coverage; refuse every missing/forged/stale/duplicate/reordered/
  incomplete/excess authority.
- [ ] Implement exact recovery-before-intent lock/journal/fsync/create-new-or-
  equal protocols, closed promotion intent `1.2` schema/vector, JCS+LF identity,
  complete scratch verification plus atomic whole-intent publication, fifteen-
  name grammar, complete verified atomic canonical/promotion/lifecycle
  publication from non-authoritative `.output-staging/`, absent-or-exact pending
  new-output stages, `W0`-`W15` writer order, atomically published exact 72-byte raw-
  intent-hash markers, and exhaustive suffix-disjoint Cartesian-product crash-
  state proof with `canonical.old` prefix states, separate absent/exact/mismatch
  new/promotion/lifecycle stage axes, and exactly
  one recovery row per state; enumerate every amendment old-snapshot prefix and
  prove exact snapshot completion before rollback; enumerate every `R0`-`R9`
  crash from every admitted `W3`-`W9` origin, the exact five-name cleanup order
  with per-unlink directory fsync, all 0..72 rollback-marker prefixes, published
  rollback under `.pending`, mandatory pending-directory-fsync replay plus
  terminal-payload revalidation before terminal rename, and repeated parent
  fsync; cross `.rolled-back` destination
  `{absent, exact_pre_existing, mismatching, unsafe}`, permit rename only for
  absent, and prove
  every collision/unsafe/simultaneous suffix preserves pending evidence and
  refuses; prove
  retry reaches the exact `.rolled-back` terminal and every state outside the
  constructed closure preserves mismatch; enumerate published `committed` under
  `.pending` across `{absent, exact_pre_existing, mismatching, unsafe}`
  destinations, simultaneous
  suffixes, mandatory pending-directory-fsync replay plus terminal-payload
  revalidation before terminal rename, parent fsync, and exact `.committed`
  replay; only an
  absent destination may rename and every collision must preserve pending
  evidence and refuse; preserve all 17 fixture-bound
  content-addressed ref-basename/target-ID/
  declared-fingerprint/exact-byte equalities, all 17 external exact-ref/current-
  producer equalities, and total observed-state tables for registry mutations
  and canonical/promotion/lifecycle records; preserve the complete pending
  journal and refuse without mutation on every mismatch, orphan final content-
  addressed records rather than deleting them, and prove end-to-end basis,
  validation-result, registry-pair, ABA, crash, concurrent-reader, and replay
  equality.
- [ ] Prove output scratch path/type/random-name/create-new/no-follow/same-
  filesystem rules and every S0-S11 boundary for all three purposes; enumerate
  all bounded-fixture scratch prefixes; prove recovery/selected reads ignore and
  never delete scratch; prove every partial authoritative output stage preserves
  the complete journal and refuses without mutation.
- [ ] Prove pre-commit recovery requires the exact bound prior lifecycle state,
  commit advances to the intent-bound successor, and later committed successors
  preserve historical journal validity without requiring old target/head bytes
  to remain latest forever.
- [ ] Implement exact lifecycle observation/transition schemas, review trigger, reassessment mappings, precedence, state fingerprints, standalone event journal/recovery, the global promotion-then-registry-then-lifecycle order with reverse-order refusal and deadlock/concurrency proof, and total transition table.
- [ ] Require new approved lineage to clear a lifecycle state; never mutate or auto-regenerate truth.
- [ ] Keep all fault-injection traits, constructors, fail points, and controls
  module-private under `#[cfg(test)]`; expose no production feature, symbol,
  CLI/compiler option, environment variable, or DTO control.

## Product-path cutover

- [ ] Rewire compiler/CLI to exact author/approve/promote/validate grammar and typed results.
- [ ] Cut core/generated/installed skills to skill-directed agent acquisition with no nested CLI wizard/model.
- [ ] Keep setup non-authoring and extend doctor additively from one retained observation.
- [ ] Remove selected old Charter input/Markdown authoring paths after caller cutover with no alias/mapper/importer.
- [ ] Cut Environment Inventory's Charter reference only to selected canonical YAML.
- [ ] Install `BR-HCM-2-CHARTER-FLOW-01` with selected Charter source/rendered fingerprints.
- [ ] Remove legacy Charter influence from all flow/budget/manifest/freshness/log/fixture/carrier paths.
- [ ] Advance C04 flow/compiler together to `reduced-v1-m8.3` and preserve C03 only with exact preimage proof.
- [ ] Preserve Project Context, Environment Inventory content, Feature Spec, and frozen pipeline behavior outside enumerated carriers.
- [ ] Delete obsolete fixed Charter constants/validators/templates only when no selected caller remains.

## Proof and closeout

- [ ] Run focused structural, semantic, intake, provenance, lineage, authority, atomicity, renderer, lifecycle, and security tests.
- [ ] Run real author/approve/promote/validate, installed skill, setup/doctor, Environment Inventory, and all-three flow smoke.
- [ ] Run native Windows proof and every HCM-1/HCM-2.1/workspace/fmt/Clippy/docs regression.
- [ ] Replay definition/package/archive/install trees by path, size, SHA-256, and bytes.
- [ ] Update only affected `00`, `03`, `04`, `06`, bridge, task, and proof rows.
- [ ] Run all handoff, link, archive, scope, secret, whitespace, diff, and GitNexus gates.
- [ ] Assemble exact final subject manifest and immutable review dispatch.
- [ ] Obtain fresh isolated review; remediate and use different fresh reviewers until CLEAN.
- [ ] Replay CLEAN bytes in staging and create the primary reviewed implementation commit.
- [ ] Create the parent handoff, rebuild/validate ledger, and create the separate mechanical closeout commit.
- [ ] Stop without starting HCM-2.3.

## Explicitly not done

- [ ] Do not edit released exact-definition bytes or silently fall back to `1.0.0`.
- [ ] Do not import/map/migrate/dual-read old Charter input or Markdown.
- [ ] Do not persist or edit renderer-derived Markdown.
- [ ] Do not let an agent, inference, default, waiver, validator, or setup self-approve/promote constitutional truth.
- [ ] Do not build a nested CLI questionnaire or call an LLM inside Handbook.
- [ ] Do not implement generic custom-kind intake/rendering, arbitrary executable hooks, or HCM-2.3.
- [ ] Do not implement Phase-3 Resolution/Projection/Snapshot/posture-kernel work.
- [ ] Do not widen into remaining artifact families, pipeline redesign, SDK/Tauri/Substrate/dock/publication, or unrelated cleanup.
