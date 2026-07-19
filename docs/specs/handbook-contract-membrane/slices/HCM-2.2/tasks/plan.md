# HCM-2.2 Implementation Plan

## Status

Planning-only. This plan becomes implementation authority only with the
review-clean [`../SPEC.md`](../SPEC.md) packet and a separately selected
top-level HCM-2.2 implementation session. No implementation task is executed in
the planning session that creates or closes this packet.

## Delivery strategy

Land one test-first constitutional-root vertical slice. Preserve every released
definition byte, add the versioned definition closure before selecting it, then
move intake, promotion, rendering, and consumers in dependency order. Editing
agents run sequentially where public types overlap. Review agents are fresh,
isolated, and read-only.

## Dependency graph

```text
P0 entry + critical impact refresh
  -> P1 RED definition/schema/lineage tests
  -> P2 versioned definition registries and selected profile
  -> P3 canonical Charter + fixed renderer
  -> P4 intake evaluation + candidate lineage
  -> P5 approval + atomic promotion + lifecycle
  -> P6 author/skill/setup/doctor adapters
  -> P7 Environment Inventory + flow cutover
  -> P8 complete proof and control-pack truth
  -> P9 fresh review/remediation loop
  -> P10 primary commit + parent closeout
```

## P0 — Revalidate entry and freeze the implementation subject

1. Resolve branch, HEAD, status, exact selected HCM-2.1 handoff, planning
   closeout, packet fingerprint, and HCM-1/HCM-2.1 dependency commits.
2. Reread the required skill chain and exact `00`-`08`, HCM-0.6 decision, and
   live definition/source/test sections named by the packet.
3. Refresh GitNexus and run upstream impact before every existing-symbol edit.
   Warn explicitly before the known CRITICAL profile-resolver change.
4. Capture the exact old definition/package manifest and selected profile
   decisions. Prove no old definition or generated package byte is staged.
5. Freeze the exact allowed-path list and current public DTO/version goldens.

**Checkpoint:** attributed clean entry, exact dependencies, no edit, critical
risk acknowledged, and immutable baseline manifest recorded.

## P1 — Write failing contract and security tests first

1. Add RED definition tests for every closed type-specific record, acyclic
   fingerprint closure, descriptor-only intake selection, old-byte preservation,
   standalone profile `1.1.0`, selected Charter closure, and non-selected
   `1.0.0` fallback refusal. Prove the one uniform RFC 8785 producer and exact
   typed dependency refs against the packet's literal schema documents,
   normalized-JCS lines, lengths, expected fingerprints, and direct equality
   between every resolved dependency and its current producer; retain capability-
   required validator `1.0.0` while adding the compatible record-schema/profile
   `1.1`/`1.1.0` validator.
2. Add RED canonical schema/typed-decode tests for the complete live-field
   preservation crosswalk, constants, bounds, unique identities, exact ordered
   nine dimensions, capability bindings, duplicate keys, multi-doc, unsafe YAML,
   and semantic failures.
3. Add RED intake tests for sixteen exact non-overlapping coverage IDs, all modes, provenance,
   inference/declaration boundaries, unknowns, contradictions, waivers,
   specificity/confidence, leaf-source bijection, value-versus-quality-waiver
   separation, and two targeted triggers.
4. Add RED immutable additive `1.1` intake/candidate/approval/promotion tests for
   exact refs, basis propagation, fingerprints, append-only behavior,
   `fido2_user_verified_repository_approver` bootstrap/registry/assertions with
   no hardware-proven or real-world identity claim, exact registration-request/
   decoded make-credential-response/operation-challenge transcript vectors,
   byte-closed status-plus-canonical-CBOR GetAssertion responses, exact 37-byte
   authenticator data with literal `0x05` flags, decoded response records, and
   a source-audited total disjoint received-status/refusal/retryability/exact-
   next-action table and end-to-end Admin-result projections,
   literal ES256/`fmt: none` protocol branch, distinct add-enrollment credential,
   closed approver-admin request/result/refusal schema and JSON vectors,
   bootstrap-without-mapping plus mapping-injection refusal, exact user-handle
   derivation vectors, bounded collection/raw-input one-over
   rejections, fake-port proof for exact opaque CTAP2 command/request/response
   bytes, including request key `2` equality to the current challenge JCS hash
   for every assertion, and no semantic field leakage, deterministic eligible-credential
   filtering/raw-ID ordering/descriptor selection, forged mappings, missing
   AT/UV/UP, CBOR/key/counter
   mismatch, stale/ambiguous class-authority pair, rejection, shared approval/
   admin replay and counter chains, concurrency/ABA, every partial ceremony-
   record journal state, and mandatory amendment basis versus create null.
   Validate the three closed challenge branches and both crossed-operation
   basis documents before any native call. Resolve every one of the fixture's
   17 content-addressed cross-record bindings through ref-basename, target-ID,
   declared-fingerprint, and exact-target-byte equality. Resolve all 17
   external definition/profile/policy/trigger/schema bindings directly to their
   current producers and reject a stale but internally consistent record.
5. Add RED renderer/canonical-emitter exact-byte and independent fingerprint
   goldens with adversarial control/unicode inputs.
6. Add RED compiler/CLI/skill/setup/doctor/Environment Inventory/flow tests for
   the selected path, result envelopes, no hidden agent, no setup authoring,
   and legacy Markdown irrelevance.
7. Add native non-Unix RED tests for read-only success and mutation refusal
   before any filesystem delta.

**Checkpoint:** failures demonstrate missing HCM-2.2 contracts, not fixture or
environment noise.

## P2 — Land the versioned exact-definition closure

1. Add schema documents/entries and typed registries/meta-schema validation for
   intake, renderer, lifecycle, approval, waiver, trigger, and semantic-
   validator definitions with their exact closed matrices.
2. Add the exact refs from the specification with the packet-published uniform
   fingerprints and fail if any literal JCS/schema byte reproduces a different
   value; add closed cross-reference validation that rejects a reproducible but
   stale producer fingerprint.
3. Add Project Authority schema/kind `1.1.0`, semantic validator `1.1.0`, and
   standalone shipped profile `1.1.0`; preserve old refs/bytes and all sibling
   selections. Keep intake out of the kind and select it only on the descriptor;
   retain the capability-required validator `1.0.0` as well.
4. Extend selected-profile decision resolution additively. Keep the critical
   resolver edit minimal and replay its five named processes immediately.
5. Update package manifests/install equality and prove changed/same-ref bytes,
   duplicate refs, missing targets, incompatible definitions, and remote or
   executable hooks refuse.

**Checkpoint:** old and new definition closures validate independently; only
the new shipped profile selects the new Charter closure.

## P3 — Land canonical Charter and deterministic rendering

1. Add the exact typed `CanonicalCharter`, complete live-field preservation
   crosswalk, and duplicate-safe schema-first semantic-capability decode path.
2. Implement the packet's closed deterministic YAML grammar, explicit nested
   key orders, and literal boundary bytes plus an independent reference emitter; retain one no-follow
   observation through parse/render/hash/recheck.
3. Implement the fixed Markdown matrix and literal full-byte fixture plus an
   independent reference renderer, media/fingerprint contract, no ambient
   input, and no persistent output.
4. Add exact typed inspection/refusal reasons and source/render fingerprint
   domains.
5. Keep old `CharterStructuredInput` callers until their tests and adapters move
   in P6; do not add a mapper or make it an alternate input to the new owner.

**Checkpoint:** engine canonical, capability, emitter, renderer, observation,
and old-definition regression suites pass.

## P4 — Implement intake evaluation and candidate lineage

1. Resolve the selected intake definition from the selected instance/kind;
   never select by filename, mode, or prompt.
2. Implement typed input envelopes and evaluation for declarations, evidenced
   inference, known unknowns, contradictions, waivers, confidence, freshness,
   sensitivity, specificity, and applicability.
   Reject overlapping coverage and require one authoritative leaf-source
   mapping; a waiver may relax only minimum-specificity quality over a retained
   user-declared value.
3. Prove all three modes share one evaluator and candidate builder. Identical
   semantic inputs produce identical normalized candidate bytes/fingerprint;
   mode and prompt events remain intake provenance only.
4. Finalize immutable intake records and content-addressed candidates with
   complete field-source maps, gap/eligibility results, and the create-null or
   mandatory amendment `basis_artifact_fingerprint`.
5. Use trusted append-only storage with safe refs, bounds, create-new equality,
   and no secrets/unrestricted evidence content.

**Checkpoint:** mode-equivalence, coverage matrix, immutable-record, and
candidate schema/semantic tests pass; no canonical write exists yet.

## P5 — Implement approval, promotion, and lifecycle

1. Enforce the exact native CTAP2.1 bootstrap/registry/amendment trust boundary
   with user presence and verification, fixed initial-Charter quorum, immutable
   acyclic registry state/transition identity, byte-closed registration and
   assertion records, and no hardware/real-world identity overclaim. Reject
   every unattended/file/stdin/software fallback and append one additive `1.1`
   approval per required exact class/authority pair.
2. Re-resolve profile, kind, schema, intake, renderer, lifecycle, capability,
   validator, waiver, trigger, candidate, approval, and target state at
   promotion time.
3. Implement the exact promotion lock/journal/fsync/rename-no-replace-or-equal/
   marker/authority-commit-point/recovery protocol and total observed-byte
   table; add substitution, inode ABA, concurrent readers/writers, replay, and
   fault injection after every persistence boundary. Require identical basis in
   intake, candidate, approvals, CLI, intent, promotion, retry, and recovery;
   install the lifecycle transition in the same authority-visible journal;
   never delete final content-addressed records during rollback.
4. Implement the four exact named registry bootstrap/add/revoke/update engine
   signatures and their exact CLI-to-engine mapping through the closed
   caller-intent-only approver-admin request and operation-by-status/refusal
   result JSON schema, thirteen positive and seventeen mandatory rejection vectors,
   and an engine-owned opaque-CBOR `NativeAuthenticatorPortV1` transport
   that returns no semantic refs or records to adapters, with
   registration-bound bootstrap, assertion-bound later transitions, distinct
   add-enrollment credential and P-256 keypair inequality, a real registry-
   resolved DER ES256 assertion signature, the exact
   `AUTHENTICATOR_UNAVAILABLE` fail-before-delta result,
   content-addressed make-credential-response/registration/get-assertion-
   response/assertion/ref/ID/fingerprint/byte
   equality, exact bounded user-handle derivation, first-admission collection/
   raw-input limits across all 33 groups, canonical unique waiver acceptance,
   shared per-credential
   replay/sign-count authority with final-use/no-lockout replacement and the
   exact sequence-4095 incomplete-add refusal before/after authority fixture,
   fixed complete-domain registry-then-approval journal discovery, predecessor-
   ordered cross-family recovery and advanced-head/fork refusal, the exact
   72-byte intent-hash commit marker, total registry/approval journal recovery,
   no-admin/no-quorum-coverage lockout refusal, and no caller/adaptor-supplied
   ceremony identity or security semantics.
5. Implement immutable lifecycle observations, the candidate-amendment review
   trigger, exact targeted reassessment mappings, total precedence/state table,
   state fingerprints, standalone lifecycle-event journal steps and total
   recovery table, and addressing promotion. Use the global promotion-then-
   registry-then-lifecycle lock order, retain and bind one exact registry pair
   through approval and promotion commit/finalization, prove contention,
   approval-crash-to-admin/promotion and admin-crash-to-approval/promotion
   recovery, reverse-order refusal, deadlock absence, and concurrent-reader authority
   visibility, and clear only after valid new approved lineage.
6. Return typed refusals without canonical, record, or lifecycle mutation.

**Checkpoint:** authority, stale-state, atomicity, recovery, lifecycle, and
fail-closed matrices pass.

## P6 — Cut author, skill, setup, and doctor paths

1. Rewire compiler/CLI to the exact author/approve/promote/validate grammar and
   engine use cases. Keep adapters free of duplicated semantics.
2. Cut core input examples and generated/installable skills to the three
   acquisition workflows. The agent collects declarations/evidence, surfaces
   gaps, requests human approval, and invokes operations; Handbook never calls
   a nested model or asks terminal questions.
3. Remove direct old Markdown authoring and selected `0.1.0` input paths after
   all callers move. Conflicting legacy files must have zero influence.
4. Keep setup non-authoring under the new selected profile.
5. Advance doctor additively with definition closure, canonical/lifecycle,
   source/render fingerprint, and next-action fields projected from one retained
   engine observation. Preserve existing decision/readiness semantics.
6. Prove JSON/human exit mappings, validate-only, overwrite/refusal, installed
   skill behavior, and native non-Unix fail-before-mutation.

**Checkpoint:** focused engine/compiler/CLI/skill/install/setup/doctor suites
pass with no selected legacy path and no hidden acquisition semantics.

## P7 — Cut Environment Inventory and flow

1. Move only Environment Inventory's Charter reference/preflight carriers to
   the selected YAML descriptor and exact source fingerprint. Do not convert
   its content authority.
2. Add `BR-HCM-2-CHARTER-FLOW-01`: load selected canonical Charter, validate
   current lifecycle/authority, render in memory, and project source/render
   fingerprints and selected owned path.
3. Remove every Charter Markdown/input influence from flow selection, budgets,
   manifest, freshness, blockers, logs, summaries, sections, notes, fixtures,
   and compiler/CLI carriers.
4. Advance C04 flow/compiler result versions together to
   `reduced-v1-m8.3`; accept `.3`, reject `.2`; preserve C03
   `reduced-v1-m8` generation `1` only with exact unchanged preimage proof.
5. Preserve HCM-2.1 Project Context behavior, fixed Environment Inventory
   content, Feature Spec, and frozen pipeline behavior beyond exact selected
   Charter reference carriers.

**Checkpoint:** installed all-three flow uses selected Project Context and
Charter YAML, fixed Environment Inventory content, no legacy selected roots,
and exact DTO/version/budget/freshness bytes.

## P8 — Run the complete proof wall and update truth

1. Run every packet verification gate, focused negative/security/concurrency/
   fault/lifecycle/mode-equivalence test, and real installed-skill/flow smoke.
2. Run HCM-1.1-HCM-1.4, HCM-2.1, full workspace, fmt, Clippy, docs, native
   Windows, definition/package/archive, handoff, link, scope, secret,
   whitespace, and diff gates.
3. Update only affected `00`, `03`, `04`, `06`, packet task state, bridge row,
   and immutable implementation proof wall.
4. Promote only the Charter-specific intake/Charter gates supported by real
   product-path evidence. Keep program-wide gates open.

**Checkpoint:** code, definitions, product paths, proof, and control-pack
classifications agree with an exact subject manifest.

## P9 — Fresh review and remediation loop

1. Write an immutable schema-valid final review dispatch bound to the complete
   subject/proof manifest.
2. Spawn one fresh isolated read-only built-in `default` reviewer; require
   Critical, Required, Optional, Nit findings first.
3. Parent-validate findings, remediate valid ones, replay the full wall, create
   a new immutable dispatch/fingerprint, and use a different fresh reviewer.
4. Repeat until `CLEAN` or a genuine packet stop condition. Do not mutate bytes
   after CLEAN.

**Checkpoint:** one fresh reviewer returns CLEAN over the exact final subject.

## P10 — Commit and close parent orchestration

1. Run GitNexus change detection, exact staged-path equality, manifest replay,
   cached diff/secret/whitespace checks, and old-definition byte proof.
2. Commit the reviewed implementation/control-pack subject with one scoped
   Conventional Commit.
3. Create one parent-owned current-schema completed handoff referencing the
   primary commit and delegated review lineage; rebuild/validate the ledger in
   all three modes.
4. Commit only the handoff/ledger mechanical closeout in a second commit.
5. Stop without starting HCM-2.3.

## Principal risks

| Risk | Mitigation |
|---|---|
| exact `1.0.0` definitions cannot express rich intake | additive schema/kind/profile closure; byte-preservation proof; no same-ref mutation |
| critical profile resolver widens across product paths | RED version-selection tests first; additive minimal edit; immediate replay of five processes; stop on wider impact |
| agent inference becomes constitutional authority | typed source kinds, normative declaration rule, candidate/approval separation, no nested model |
| immutable records become competing truth | candidate/content store separate from canonical path; promotion is sole authority transition |
| crash leaves physical partial files | shared-lock readers expose only a matching committed journal; total observed-state rollback/roll-forward and exhaustive fault injection |
| renderer or Markdown becomes authority | typed canonical-only input, in-memory output, zero persistent write, conflicting-legacy probes |
| trigger regenerates unrelated policy | exact non-empty coverage mappings, append-only reassessment, approval required for any new promotion |
| sibling/pipeline scope expands | exact carrier list, regression goldens, stop instead of dual truth or cleanup |
