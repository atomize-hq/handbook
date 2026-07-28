# HCM-2.4 implementation checklist

Status: partial implementation under Option C planning containment.
P0/P1A/P1B/P1C and P3/P3B are review-clean. P2S is review-clean and committed
at `755acc56`; P2A is immutable non-production feasibility evidence. Future
native-adapter selection is superseded. P2 remains unresolved, fail-closed, and
not GREEN. The P4/P5 token prerequisite is review-clean and committed at
`00dde01`; P4's separate negative-surface/path proof remains open. P6, P7, and
Phase 2 exit remain incomplete.

## Entry

- [x] Verify the completed HCM-2.4 planning handoff, reviewed planning commit,
      final subject fingerprint, clean worktree, and origin 0/0.
- [x] Confirm implementation authority repeats all SPEC non-goals and stops.
- [x] Treat selection as read-only P0 authority only; do not edit any
      implementation surface until P0 freezes and passes its exact live gate.
- [x] Refresh GitNexus and replace every UNKNOWN/intended symbol with an exact
      UID upstream impact result.
- [x] Report every HIGH/CRITICAL result before editing.

## P0 — Inventory lock

- [x] Replay exact fingerprints for all six shipped kinds and schemas, both
      root profiles, existing Charter intake/renderer, and HCM-2.3 registry brief.
- [x] Freeze the exact three selected root descriptors.
- [x] Freeze the complete
      `shipped_profile_request` → `resolve_shipped_profile_decisions` →
      `CharterDefinitionRegistry::validate_selected_decisions` path and every
      HCM-2.2 record producer/currentness consumer.
- [x] Freeze the released HCM-2.2 Charter profile ref/fingerprint pair and the
      full Project Authority descriptor field table.
- [x] Freeze every fixed family/path/bridge/helper/test selector and packet file
      manifest.
- [x] Add the exact P2 `crates/cli/src/main.rs`, two named `cli_surface.rs`
      tests, and the one consumed environment-inventory help snapshot to the
      live manifest.
- [x] Freeze focused HCM-2.1, HCM-2.2, HCM-2.3, flow, pipeline, package, and
      archive proof commands.
- [x] Resolve every exact P1B/P1C UID, inventory compiler
      `doctor_report_from_inspection` as the sixth existing registry caller,
      and confirm only the planned seventh caller from
      `evaluate_charter_intake`, with no eighth caller or new process/module
      beyond the SPEC impact ceilings.
- [x] Record status/evidence:
      `proof/implementation/P0-inventory-lock.md`; independent evidence closure
      review CLEAN.

## P1A — Definition support and admission

- [x] Add immutable exact-version successors for Project Context, Environment
      Context, Work Specification, Decision Record, and Risk Record.
- [x] Add one schema-backed first-party intake and fixed deterministic renderer
      for each.
- [x] Add the exact three-instance root profile successor.
- [x] Prove its `project_authority` row equals the released 1.1 descriptor in
      every field and freeze the literal authored 1.2 profile fingerprint in an
      immutable HCM-2.4 vector.
- [x] Add exact built-in sources and admit only exact package-owned first-party
      renderer refs while retaining all other later-owned dependency refusals.
- [x] Admit only the five exact frozen Project Context, Environment Context,
      Work Specification, Decision Record, and Risk Record descriptor rows,
      including kind, intake, singleton renderer, role/path, requiredness, and
      empty later-owned fields; retain lifecycle/Projection/overlay and every
      mismatched dependency refusal.
- [x] Prove duplicate-safe parsing, coverage, fingerprints, exact refs, null
      Resolution input, and full-byte render goldens.
- [x] Classify only a type-absent, string-valued `const` coverage leaf as
      String in `ResolvedSchema::collect_coverage_leaf_shapes`; prove every
      other indeterminate case remains refused, including string `const` with
      an unsupported explicit type, and stop on a sixth production symbol or
      broader inference.
- [x] Prove no public API/Cargo/external-dependency/package-boundary change.
- [x] Record status/evidence:
      `proof/implementation/P1A-definition-support.md`; different-fresh closure
      review CLEAN at dispatch
      `20260726T205749Z--HCM-2-4--p1a-complete-subject-closure-review`.

## P1B — Charter compatibility foundation

- [x] Keep `shipped_profile_request` selecting 1.1 throughout P1B.
- [x] Extend only
      `CharterDefinitionRegistry::validate_selected_decisions` to accept the
      exact released 1.1 tuple and the exact P1A-authored 1.2 tuple.
- [x] Compare every Project Authority id/kind/role/capability/label/path/
      requiredness/condition/dependency/lifecycle/intake/renderer/Projection/
      overlay/extension field and exact subordinate definition.
- [x] Reject wrong fingerprints, unlisted versions, range/prefix acceptance,
      descriptor mutations, missing definitions, fallback, second resolution,
      migration, and dual reads.
- [x] Make the existing HCM-2.2 profile identity constants crate-visible
      without changing either value.
- [x] Update only the exact registry, intake, approval-currentness,
      promotion, lifecycle-result/authority, and transaction-preflight symbols
      listed in SPEC so Charter records retain the HCM-2.2 pair.
- [x] Keep compiler `doctor_report_from_inspection` read/proof-only and prove
      its exact Charter definition closure remains resolved for direct 1.1 and
      selected 1.2 decisions.
- [x] Add exact
      `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs` and prove
      `invalid_compatible_profile_decisions_cannot_produce_charter_intake`
      refuses before any intake/candidate record is produced.
- [x] Do not edit `validate_candidate_v13`,
      `validate_promotion_intent_v12`, candidate/result schemas or vectors,
      approval/lineage/lifecycle formats, committed authority, or public APIs.
- [x] Prove direct 1.1 and compatible 1.2 decisions produce byte/fingerprint
      identical intake, candidate, result, approval, promotion, intent, lineage,
      transaction, recovery/replay, and committed authority.
- [x] Replay every HCM-2.2 negative unchanged and all HIGH/CRITICAL callers.
- [x] Confirm the shipped resolver still selects 1.1 at P1B exit.
- [x] Record status/evidence:
      `proof/implementation/P1B-charter-compatibility.md`; different-fresh
      closure review CLEAN.

## P1C — Shipped-root adoption

- [x] Select only the new shipped-root profile ref and exact P1A source closure
      in `shipped_profile_request`.
- [x] Admit only exact Project Context kind `1.0` or `1.1` through the existing
      private `selected_contract_matches` predicate while preserving the
      unchanged schema `1.0`, canonical path, equality checks, and every other
      mismatch refusal.
- [x] Before editing the predicate, prove RED for exact `1.1`; retain exact
      `1.0`; then prove both positives plus crossed directions, unrelated
      identity, unlisted version, prefix, range, `latest`, bare/fallback, wrong
      schema, and wrong path refusals in the existing unit test.
- [x] Assert non-null exact intake refs and singleton exact renderer refs for
      both `project_context` and `environment_context`.
- [x] Preserve exactly three root descriptors, Charter authority, condition
      semantics, vocabulary, and Context Resolution sources.
- [x] Prove the real resolver returns generic shipped-root 1.2 decisions,
      immediately passes the exact Charter membrane, and every Charter record
      still carries the released 1.1 pair.
- [x] In Unix compiler test
      `doctor_api_projects_the_exact_stable_project_context_row`, change only
      report schema expectation `1.1.0` to `1.2.0` and Project Context kind
      expectation `1.0.0` to `1.1.0`.
- [x] In Unix CLI test `doctor_reports_ready_when_required_artifacts_present`,
      replace only its obsolete inline Charter schema `1.0` setup with the
      existing `write_valid_selected_charter` helper and change only its
      Project Context kind expectation `1.0.0` to `1.1.0`; do not edit the
      helper, fixture assets, another assertion, or production code.
- [x] In WSL with an isolated Linux target directory, run those two exact
      Unix projection tests plus
      `doctor::tests::doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba`.
- [x] Replay the selected-profile targets, P1B compatibility target, unchanged
      HCM-2.1 integration target at 12/12, complete HCM-2.2 wall, HCM-2.3
      preservation wall, and all CRITICAL upstream flows; prove
      Work/Decision/Risk remain unselected.
- [x] Record status/evidence:
      `proof/implementation/P1C-shipped-root-adoption.md`; different-fresh
      closure review CLEAN at dispatch
      `20260726T235106Z--HCM-2-4--p1c-shipped-root-adoption-closure-review`.

## P2 — Environment Context

- [x] Draft the exact planning-only evidence/evaluator amendment:
      `decision/20260727-p2-managed-operational-surface-evidence-evaluator-planning-amendment.md`.
- [x] Rebuild GitNexus at `00dde01`, record the three CRITICAL resolver
      ceilings and the incomplete-edge LOW reason-enum result, and run the
      9/9 non-production authenticator-security feasibility probe.
- [x] Obtain fresh discovery review of the planning amendment; reviewer
      `/root/hcm_2_4_p2_planning_review` returned seven P2/Required findings
      covering rollback, source fingerprints, closed schemas, freshness,
      outcome reachability, API compatibility, and packet decomposition.
- [x] Remediate all seven findings in the planning/control-pack subject by
      adding the monotonic evidence/authenticator head boundary, retained
      source schema and preimage, total closed records/closure, per-source
      freshness, reachable precedence, exact compatibility API, and separate
      P2S/P2A/P2P/P2R/P2I/P2V packets.
- [x] Obtain different-fresh closure review; reviewer
      `/root/hcm_2_4_p2_planning_closure` closed five findings and returned two
      causal P2/Required gaps for simultaneous two-head rollback and incomplete
      dependency-byte observation binding.
- [x] Remediate both causal findings with a safely enumerated unique maximal
      immutable transaction chain and ordered closed observation rows for every
      attempted dependency class.
- [x] Obtain supplemental causal review 1; reviewer
      `/root/hcm_2_4_p2_planning_supplemental_1` closed rollback resistance and
      returned one causal P2 for transaction-directory failures that precede a
      sequence-bearing observation row.
- [x] Remediate that finding with an exact first
      `evidence_transaction_store` row, canonical raw-filename inventory
      fingerprint, closed error codes, and six store-level proof vectors.
- [x] Record supplemental causal review 2's final P2: the six store failures
      lacked a normative one-to-one observation/nullability/error/fixture/
      closure matrix.
- [x] Repair only that gap with exact absent, unsafe, unreadable,
      invalid-entry, alias, and over-limit rows plus deterministic Unix/Windows
      first-row fixtures.
- [x] Freeze the one-review rule: start a new matrix-only discovery lineage,
      not a third supplemental cycle; any valid P1/P2 stops without another
      repair, while CLEAN permits the reviewed planning commit and human
      approval request only.
- [x] Record the matrix-only discovery review's P2: a complete over-limit
      directory envelope is unbounded against the frozen 131072-byte ceiling.
- [x] Record the later unadmitted discovery signal: exact transaction names
      first exceed 131072 bytes at Unix entry 1102 and Windows entry 558, before
      the count ceiling; the signal did not enter a review gate.
- [x] Repair only the newly authorized gap with checked 4096-entry and
      131072-byte projected canonical-envelope ceilings, exact
      `directory_limit_exceeded` nullability/error/first-row/closure semantics,
      one fixed closure for all count/byte/arithmetic over-limit stores, and
      count 4096/4097, Unix 1101/1102, Windows 557/558, exact
      131072/131073-byte, long-name, and arbitrary-suffix vectors.
- [x] Obtain exactly one fresh complete-subject review; any valid P1/P2 stops
      without remediation, while CLEAN permits only the reviewed planning
      commit, separate mechanical closeout, and distinct P2S/P2A approval.
- [x] Implement and independently review the eight P2S schemas, exact evaluator
      definition, semantic/preimage/nullability/dual-ceiling vectors, and
      registration-absence proof without a Rust production edit; commit
      `755acc56`, reviewed subject `sha256:7b1d0d4444b84613334625191f2d7272e2a866a7ba38d1b67aca875bc78c5f04`.
- [x] Run the bounded P2A non-production probe: preserve the supporting 9/9
      authenticator-security wall, prove unavailable-port filesystem
      zero-write 1/1 with discarded temporary material, and record the
      present Windows WebAuthn surface plus unavailable safe binding/live
      transport/cancellation behavior.
- [x] Select Option C planning containment: supersede future production
      native-adapter selection and preserve P2S/P2A as immutable historical
      evidence.
- [x] Freeze the containment boundary: no WebAuthn/FIDO/CTAP, USB/HID, device,
      platform UI, cancellation/error, unsafe, Cargo/lockfile, dependency,
      producer, evaluator, resolver, or native integration work is authorized.
- [x] Preserve the released fail-closed result: no structured Environment
      Context input, `applicability_basis`, profile toggle, artifact presence,
      or `indeterminate` coercion can establish applicability.
- [x] Trace `artifact_decision` through inspection/readiness classification,
      setup action selection, doctor/setup rendering, and CLI exit policy.
      Record the missing/structurally-valid `INDETERMINATE` path, the
      higher-precedence invalid/unsafe/unreadable `INVALID` path, nonzero exit,
      and the invariant that shipped-root 1.2 cannot reach READY.
- [x] Record the bounded A–E product analysis in
      `decision/20260728-p2-product-decision-reassessment.md`, including all
      requested experience, security, burden, contract, versioning, program,
      READY, and next-packet dimensions.
- [x] Recommend, without selecting, an additive default-profile successor that
      temporarily postpones Environment Context, preserves Charter and Project
      Context authority, and leaves shipped-root 1.1/1.2 immutable.
- [x] Obtain isolated Option C containment review: dispatch
      `20260728T134152Z--HCM-2-4--p2-option-c-containment-review` returned CLEAN
      with no findings.
- [x] Obtain isolated product-analysis discovery review, remediate
      `PRODUCT-STATUS-001` by recording `INVALID` precedence, and obtain
      different-fresh CLEAN closure at dispatch
      `20260728T140357Z--HCM-2-4--p2-product-decision-reassessment-closure-review`.
- [ ] BLOCKED: obtain a later explicit human product decision selecting a
      released-authority direction. Option C itself does not select an
      authority model or authorize implementation.
- [ ] Establish a focused RED proving Markdown is currently authoritative.
- [ ] Write only selected `.handbook/project/environment.yaml` canonical bytes.
- [ ] Cut CLI/setup/doctor/compiler/flow reads to selected retained YAML.
- [ ] Update only `AuthorCommand::EnvironmentInventory` help,
      `author_help_matches_snapshot`,
      `author_environment_inventory_help_matches_snapshot`, and the one consumed
      subcommand snapshot added by the amended manifest.
- [ ] Render Markdown only from canonical bytes.
- [ ] Remove legacy Environment Inventory authority helpers/templates/fallbacks.
- [ ] Prove condition, schema, safe path, no-follow, atomicity, retained
      observation, concurrency, full-byte view, and zero legacy influence.
- [ ] Record status/evidence: pending.

## P3 — Work Specification

- [x] Establish focused RED for authoritative `FEATURE_SPEC.md`.
- [x] Capture schema-valid fixed-path Work Specification YAML.
- [x] Add/select the exact
      `example.profile.hcm-2-4-work-specification@1.0.0` fixture profile and
      frozen `work_specification`/`delivery_unit` descriptor before capture.
- [x] Bind Stage 10 provenance, identity, trust, and handoff decisions to YAML.
- [x] Generate the fixed Feature Spec Markdown view deterministically and reject
      it as canonical input.
- [x] Replay capture/cache/rollback/provenance and every affected handoff
      emit/validate process.
- [x] Prove descriptor selection rather than equality with a hard-coded path.
- [x] Prove no public bundle/API, command, dynamic path, or package change.
- [x] Record status/evidence:
      `proof/implementation/P3-work-specification.md`; closure reviewer
      `/root/hcm_2_4_p3_closure_review` returned CLEAN.

## P3B — CLI surface proof integration

- [x] Record the eight-test workspace-wall RED and exact test-only selector in
      `decision/20260727-p3b-cli-surface-proof-selector-repair.md`.
- [x] Obtain fresh built-in review of the first exact selector; reviewer
      `/root/hcm_2_4_p3b_selector_review` returned CLEAN.
- [x] Establish focused RED showing the recursively copied fixture stage still
      declares the superseded single Markdown output.
- [x] Record the exact additive fixture-only selector in
      `decision/20260727-p3b-fixture-contract-selector-repair.md`.
- [x] Obtain fresh built-in review of the additive fixture selector before
      editing any fixture authority input; reviewer
      `/root/hcm_2_4_p3b_fixture_selector_review` returned CLEAN.
- [x] Synchronize only the ten selected fixture-authority inputs; do not commit
      generated fixture-repo outputs.
- [x] Establish focused RED showing synchronized fixture authority reaches the
      existing durable repository-identity prerequisite and refuses before
      authoring.
- [x] Prove with the existing engine repository-open test that a fresh
      setup-owned identity can coexist with repository profile selection.
- [x] Record the exact test-only prerequisite amendment in
      `decision/20260727-p3b-repository-identity-prerequisite-selector-repair.md`.
- [x] Obtain fresh built-in review of the repository-identity prerequisite
      amendment before implementation resumes; reviewer
      `/root/hcm_2_4_p3b_identity_selector_review` returned CLEAN.
- [x] Establish focused GREEN for standalone Stage 10 preview/apply and raw
      compile-payload refusal after fresh temporary-repository identity setup.
- [x] Establish focused RED showing the foundation-flow Work Specification
      inputs would replace the preserved M4 handoff feature ID with
      `example-record-work`.
- [x] Record the exact two-fixture identity repair in
      `decision/20260727-p3b-foundation-feature-identity-selector-repair.md`.
- [x] Obtain fresh built-in discovery review of the exact two-fixture identity
      amendment; reviewer
      `/root/hcm_2_4_p3b_feature_identity_selector_review` returned one
      P2/Required schema-pattern finding.
- [x] Remediate
      `p3b-foundation-feature-identity-selector-repair-discovery-1-P2-1` by
      using schema-valid `fs.m4.foundation.journey-2026-04`, which preserves
      the required slug.
- [x] Obtain different-fresh closure review of the remediated two-fixture
      selector before changing either model output; reviewer
      `/root/hcm_2_4_p3b_feature_identity_selector_closure` returned CLEAN and
      closed the discovery P2.
- [x] Establish focused RED showing the M5 test consumer still requests the
      generated Markdown view as an authoritative handoff input.
- [x] Record the exact canonical-YAML consumer/evidence selector in
      `decision/20260727-p3b-m5-canonical-consumer-selector-repair.md`.
- [x] Obtain fresh built-in review of the M5 canonical-consumer selector before
      editing the three helpers or newly selected evidence paths; reviewer
      `/root/hcm_2_4_p3b_m5_consumer_selector_review` returned CLEAN.
- [x] Run the complete CLI surface and isolate the remaining 3/98 failures to
      the paired shared compile payload/explain goldens.
- [x] Record the exact two-golden repair in
      `decision/20260727-p3b-shared-compile-golden-selector-repair.md`.
- [x] Obtain fresh built-in discovery review before regenerating either shared
      golden; reviewer `/root/hcm_2_4_p3b_compile_golden_selector_review`
      returned one P2/Required missing-consumer finding.
- [x] Remediate
      `p3b-shared-compile-golden-selector-repair-discovery-1-P2-1` by adding
      `crates/pipeline/tests/pipeline_compile.rs` as a read/proof-only consumer.
- [x] Obtain different-fresh closure review before regenerating either shared
      golden; reviewer
      `/root/hcm_2_4_p3b_compile_golden_selector_closure` returned CLEAN and
      closed the discovery P2.
- [x] Repair only Stage 10 setup/assertions in `crates/cli/tests/cli_surface.rs`.
- [x] Regenerate only the happy/skip journey transcript proof bytes.
- [x] Remove transient fixture-repository outputs before packet verification.
- [x] Establish packet-wall RED showing the missing-Work-Specification refusal
      test depended on an untracked generated YAML file before product
      execution.
- [x] Record the exact one-assertion test-only selector in
      `decision/20260727-p3b-negative-fixture-independence-selector-repair.md`.
- [x] Obtain fresh built-in selector review; discovery reviewer
      `/root/hcm_2_4_p3b_negative_fixture_selector_review` returned one P2
      selector-coherence finding, and different-fresh reviewer
      `/root/hcm_2_4_p3b_negative_fixture_selector_closure` returned CLEAN
      after the exact SPEC-row remediation.
- [x] Replace only the fallible deletion with an explicit committed-fixture
      absence assertion and preserve all refusal assertions.
- [x] Establish packet-wall RED showing `feature_spec_contract` compares the
      rich M4 view to a distinct minimal renderer fixture.
- [x] Record the exact cross-case test-only selector in
      `decision/20260727-p3b-feature-spec-contract-selector-repair.md`.
- [x] Obtain fresh built-in selector review; reviewer
      `/root/hcm_2_4_p3b_feature_spec_contract_selector_review` returned CLEAN
      with one P4 proof-source-attribution wording nit, corrected locally.
- [x] Replace only the stale cross-fixture equality with exact happy/skip
      canonical-input and generated-view equality.
- [x] Run the full CLI surface, 273-test P3/P3B packet wall, strict clippy,
      archive boundary/self-test, diff hygiene, and complete workspace wall.
- [x] Obtain complete-subject discovery review; reviewer
      `/root/hcm_2_4_p3b_final_implementation_review` returned one P2 semantic
      objective finding and one P4 proof-count nit.
- [x] Remediate `p3b-final-implementation-review-P2-1` in both canonical model
      outputs, dependent generated views/plan, the selector record, and one
      focused semantic assertion; correct `p3b-final-implementation-review-P4-1`.
- [x] Rerun focused M4/M5/semantic proof, the 273-test P3/P3B wall, strict
      clippy, archive/diff checks, and the complete workspace wall after
      remediation.
- [x] Obtain different-fresh closure review; reviewer
      `/root/hcm_2_4_p3b_final_implementation_closure` returned CLEAN with no
      new finding or advisory and closed the discovery P2/P4.
- [x] Obtain different-fresh implementation review and record
      `proof/implementation/P3B-cli-surface-proof.md`.

## P4 — Decision Record

- [x] Select `example.profile.hcm-2-4-decision-record@1.0.0` from the exact
      decision fixture profile and add only `decision_record` at
      `.handbook/records/decision.yaml` with the frozen null-role/intake/renderer
      closure.
- [x] Obtain fresh independent review of
      `decision/20260727-p4-p5-coverage-token-derivation-selector.md`.
- [x] Convert only the named Decision blocker refusal into exact committed
      token-list proof and activate only the named positive mutation test.
- [x] Capture RED, make the one-expression `intake_commit_plan` edit, and prove
      Decision intake/candidate/promotion/read/validate plus stale-basis and all
      unrelated negative cases GREEN.
- [x] Obtain fresh independent implementation review, remediate both closure
      findings, obtain different-fresh supplemental causal review CLEAN, and
      commit the reviewed P4/P5 token packet at `00dde01`.
- [ ] Prove no root default, generated command, inferred filename, Projection,
       or persistent view.
      This broader P4 gate is outside the approved token selector and remains
      outstanding for separate authority; token prerequisite closure does not
      satisfy it.
- [x] Record bounded-stop evidence:
      `proof/implementation/P4-decision-record.md` and
      `decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`.

## P5 — Risk Record

- [x] Select `example.profile.hcm-2-4-risk-record@1.0.0` from the exact risk
      fixture profile and add only `risk_record` at
      `.handbook/records/risk.yaml` with the frozen null-role/intake/renderer
      closure.
- [x] Convert only the named Risk blocker refusal into exact committed
      token-list proof and activate only the named positive mutation test.
- [x] Share only the reviewed P4 production expression, capture RED/GREEN, and
      prove Risk intake/candidate/promotion/read/validate plus every unrelated
      negative case.
- [x] Prove no root default, generated command, inferred filename, Projection,
      or persistent view.
- [x] Keep evidence independent from Decision Record or record an accepted
      pre-edit atomicity rationale.
- [x] Record bounded-stop evidence:
      `proof/implementation/P5-risk-record.md` and
      `decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`.

## P6 — Aggregate cleanup

- [ ] BLOCKED: P2 is unresolved/not GREEN and P4's separate
      negative-surface/path proof remains open. The P4/P5 token prerequisite
      is review-clean and committed.
- [ ] Replace fixed/bridge flow input collection with admitted selected
      descriptors and retained canonical bytes.
- [ ] Preserve Charter authority, Project Context behavior, registry brief,
      packet ordering, budgets, fixtures, blockers, and refusals.
- [ ] Delete `BR-HCM-2-PILOT-FLOW-01`.
- [ ] Delete `BR-HCM-2-CHARTER-FLOW-01`.
- [ ] Delete both bridge types, fixed renderer path selector, fixed-family/path
      authority, fixed-sibling loaders, legacy exceptions, and bridge-only tests.
- [ ] Prove forbidden selector/legacy influence scans are empty.
- [ ] Record status/evidence: pending.

## P7 — Proof, review, and closeout

- [ ] BLOCKED: P7 cannot begin until P6 is accepted; Phase 2 exit cannot be
      claimed from Option C containment.
- [ ] Run every per-family proof and exact Phase 2 exit mapping.
- [ ] Replay exact 1.1 acceptance, exact 1.2 compatibility, every
      Project-Authority-field negative, real-path 1.2 selection, byte-identical
      HCM-2.2 records, and all unchanged HCM-2.2 negative/atomic/recovery
      matrices.
- [ ] Run formatting, strict lint, focused crates, full workspace, feature tree,
      package/archive, native Windows, documents, handoff modes, diff check, and
      GitNexus change detection.
- [ ] Update only earned gates and bridge rows.
- [ ] Freeze the complete-subject fingerprint and immutable review dispatch.
- [ ] Complete discovery review/burst, consolidate all P1/P2, and perform one
      remediation.
- [ ] Complete one different-fresh delta closure; use supplemental cycles only
      for directly caused/unmasked P1/P2 within the declared allowance.
- [ ] Resolve all P1/P2; fix or register/deduplicate P3/P4.
- [ ] Commit the reviewed subject.
- [ ] Create the parent-owned completed handoff and rebuild the ledger.
- [ ] Commit the mechanical closeout separately, push without force, and verify
      clean origin 0/0.
- [ ] Stop before HCM-3.x or any automatically inferred implementation.
- [ ] State in the completed handoff that implementation requires a new explicit
      top-level selection of that exact handoff and enters read-only P0 first.
