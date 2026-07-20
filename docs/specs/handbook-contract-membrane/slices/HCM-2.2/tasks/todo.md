# HCM-2.2 Implementation Checklist

**Status:** escalation `HCM-2.2-ESC-001` Option 1 selected for a
documentation-only authority repair. The implementation checkpoint remains
non-authoritative. A later implementation packet requires fresh CLEAN review of
this complete repaired planning subject and a new parent dispatch. Do not mark
implementation items complete or start HCM-2.3.

The immutable escalation handoff's `status/plan.md` and `status/todo.md` names
are erroneous. The authoritative mutable paths are this `tasks/todo.md` and
[`tasks/plan.md`](plan.md); the immutable record remains unchanged.

## Authority-repair planning packet

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
  publication from non-authoritative scratch, fifteen-name pending grammar,
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
- [ ] Add additive intake/approval/promotion `1.1`, candidate `1.2`, and lifecycle-validation-result `1.0` basis lineage plus exact ES256/`fmt: none` authenticator bootstrap/registry/UV/UP/assertion/class/revocation, distinct add-enrollment credential and P-256 keypair, real registry-resolved DER signature verification, exact `AUTHENTICATOR_UNAVAILABLE` fail-before-delta result, caller-intent-only request DTOs, operation-by-status/refusal result DTOs and mandatory crossed-state rejections, forged mapping, stale authority, replay, and ABA RED tests; mechanically prove every current task names candidate `1.2` and treats candidate `1.0`/`1.1` as historical-only.
- [ ] Add a retained lifecycle-head positive with at least two observations whose event-precedence order differs from ref order, plus ref-sorted/reordered, duplicate, incomplete, excess, and retained-head rewrite negatives.
- [ ] Add create-null versus mandatory-amendment-basis equality at every lineage stage plus the exact observed-state promotion recovery table, partial record install, preexisting-equal reuse, commit-marker, concurrent-reader, and no-visible-partial-authority RED tests.
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
  `1.2` subject, engine-owned lifecycle-validation result `1.0`, and final
  content-addressed candidate in the exact acyclic order with complete field-
  source mappings and frozen create/amend basis.
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
  name grammar, `W0`-`W15` writer order, atomically published exact 72-byte raw-
  intent-hash markers, and exhaustive suffix-disjoint Cartesian-product crash-
  state proof with separate old/new/promotion/lifecycle stage axes and exactly
  one recovery row per state; enumerate every amendment old-snapshot prefix and
  prove exact snapshot completion before rollback; enumerate every `R0`-`R9`
  crash from every admitted `W3`-`W9` origin, the exact five-name cleanup order
  with per-unlink directory fsync, all 0..72 rollback-marker prefixes, published
  rollback under `.pending`, terminal rename, and repeated parent fsync; prove
  retry reaches the exact `.rolled-back` terminal and every state outside the
  constructed closure preserves mismatch; enumerate published `committed` under
  `.pending` across absent/exact/mismatching/unsafe destinations, simultaneous
  suffixes, terminal rename, parent fsync, and exact `.committed` replay; only an
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
