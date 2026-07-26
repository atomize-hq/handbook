# HCM-2.4 implementation checklist

Status: planning only. These boxes remain unchecked until HCM-2.4
implementation is separately selected.

## Entry

- [ ] Verify the completed HCM-2.4 planning handoff, reviewed planning commit,
      final subject fingerprint, clean worktree, and origin 0/0.
- [ ] Confirm implementation authority repeats all SPEC non-goals and stops.
- [ ] Treat selection as read-only P0 authority only; do not edit any
      implementation surface until P0 freezes and passes its exact live gate.
- [ ] Refresh GitNexus and replace every UNKNOWN/intended symbol with an exact
      UID upstream impact result.
- [ ] Report every HIGH/CRITICAL result before editing.

## P0 — Inventory lock

- [ ] Replay exact fingerprints for all six shipped kinds and schemas, both
      root profiles, existing Charter intake/renderer, and HCM-2.3 registry brief.
- [ ] Freeze the exact three selected root descriptors.
- [ ] Freeze the complete
      `shipped_profile_request` → `resolve_shipped_profile_decisions` →
      `CharterDefinitionRegistry::validate_selected_decisions` path and every
      HCM-2.2 record producer/currentness consumer.
- [ ] Freeze the released HCM-2.2 Charter profile ref/fingerprint pair and the
      full Project Authority descriptor field table.
- [ ] Freeze every fixed family/path/bridge/helper/test selector and packet file
      manifest.
- [ ] Add the exact P2 `crates/cli/src/main.rs`, two named `cli_surface.rs`
      tests, and the one consumed environment-inventory help snapshot to the
      live manifest.
- [ ] Freeze focused HCM-2.1, HCM-2.2, HCM-2.3, flow, pipeline, package, and
      archive proof commands.
- [ ] Resolve every exact P1B/P1C UID, inventory compiler
      `doctor_report_from_inspection` as the sixth existing registry caller,
      and confirm only the planned seventh caller from
      `evaluate_charter_intake`, with no eighth caller or new process/module
      beyond the SPEC impact ceilings.
- [ ] Record status/evidence: pending.

## P1A — Definition support and admission

- [ ] Add immutable exact-version successors for Project Context, Environment
      Context, Work Specification, Decision Record, and Risk Record.
- [ ] Add one schema-backed first-party intake and fixed deterministic renderer
      for each.
- [ ] Add the exact three-instance root profile successor.
- [ ] Prove its `project_authority` row equals the released 1.1 descriptor in
      every field and freeze the literal authored 1.2 profile fingerprint in an
      immutable HCM-2.4 vector.
- [ ] Add exact built-in sources and admit only exact package-owned first-party
      renderer refs while retaining all other later-owned dependency refusals.
- [ ] Admit only the five exact frozen Project Context, Environment Context,
      Work Specification, Decision Record, and Risk Record descriptor rows,
      including kind, intake, singleton renderer, role/path, requiredness, and
      empty later-owned fields; retain lifecycle/Projection/overlay and every
      mismatched dependency refusal.
- [ ] Prove duplicate-safe parsing, coverage, fingerprints, exact refs, null
      Resolution input, and full-byte render goldens.
- [ ] Prove no public API/Cargo/external-dependency/package-boundary change.
- [ ] Record status/evidence: pending.

## P1B — Charter compatibility foundation

- [ ] Keep `shipped_profile_request` selecting 1.1 throughout P1B.
- [ ] Extend only
      `CharterDefinitionRegistry::validate_selected_decisions` to accept the
      exact released 1.1 tuple and the exact P1A-authored 1.2 tuple.
- [ ] Compare every Project Authority id/kind/role/capability/label/path/
      requiredness/condition/dependency/lifecycle/intake/renderer/Projection/
      overlay/extension field and exact subordinate definition.
- [ ] Reject wrong fingerprints, unlisted versions, range/prefix acceptance,
      descriptor mutations, missing definitions, fallback, second resolution,
      migration, and dual reads.
- [ ] Make the existing HCM-2.2 profile identity constants crate-visible
      without changing either value.
- [ ] Update only the exact registry, intake, approval-currentness,
      promotion, lifecycle-result/authority, and transaction-preflight symbols
      listed in SPEC so Charter records retain the HCM-2.2 pair.
- [ ] Keep compiler `doctor_report_from_inspection` read/proof-only and prove
      its exact Charter definition closure remains resolved for direct 1.1 and
      selected 1.2 decisions.
- [ ] Add exact
      `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs` and prove
      `invalid_compatible_profile_decisions_cannot_produce_charter_intake`
      refuses before any intake/candidate record is produced.
- [ ] Do not edit `validate_candidate_v13`,
      `validate_promotion_intent_v12`, candidate/result schemas or vectors,
      approval/lineage/lifecycle formats, committed authority, or public APIs.
- [ ] Prove direct 1.1 and compatible 1.2 decisions produce byte/fingerprint
      identical intake, candidate, result, approval, promotion, intent, lineage,
      transaction, recovery/replay, and committed authority.
- [ ] Replay every HCM-2.2 negative unchanged and all HIGH/CRITICAL callers.
- [ ] Confirm the shipped resolver still selects 1.1 at P1B exit.
- [ ] Record status/evidence: pending.

## P1C — Shipped-root adoption

- [ ] Select only the new shipped-root profile ref and exact P1A source closure
      in `shipped_profile_request`.
- [ ] Assert non-null exact intake refs and singleton exact renderer refs for
      both `project_context` and `environment_context`.
- [ ] Preserve exactly three root descriptors, Charter authority, condition
      semantics, vocabulary, and Context Resolution sources.
- [ ] Prove the real resolver returns generic shipped-root 1.2 decisions,
      immediately passes the exact Charter membrane, and every Charter record
      still carries the released 1.1 pair.
- [ ] Replay all CRITICAL upstream flows, the complete HCM-2.2 wall, and prove
      Work/Decision/Risk remain unselected.
- [ ] Record status/evidence: pending.

## P2 — Environment Context

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

- [ ] Establish focused RED for authoritative `FEATURE_SPEC.md`.
- [ ] Capture schema-valid fixed-path Work Specification YAML.
- [ ] Add/select the exact
      `example.profile.hcm-2-4-work-specification@1.0.0` fixture profile and
      frozen `work_specification`/`delivery_unit` descriptor before capture.
- [ ] Bind Stage 10 provenance, identity, trust, and handoff decisions to YAML.
- [ ] Generate the fixed Feature Spec Markdown view deterministically and reject
      it as canonical input.
- [ ] Replay capture/cache/rollback/provenance and every affected handoff
      emit/validate process.
- [ ] Prove descriptor selection rather than equality with a hard-coded path.
- [ ] Prove no public bundle/API, command, dynamic path, or package change.
- [ ] Record status/evidence: pending.

## P4 — Decision Record

- [ ] Select `example.profile.hcm-2-4-decision-record@1.0.0` from the exact
      decision fixture profile and add only `decision_record` at
      `.handbook/records/decision.yaml` with the frozen null-role/intake/renderer
      closure.
- [ ] Exercise intake, generic mutation/read/validate, and renderer against real
      canonical bytes.
- [ ] Prove no root default, generated command, inferred filename, Projection,
      or persistent view.
- [ ] Record status/evidence: pending.

## P5 — Risk Record

- [ ] Select `example.profile.hcm-2-4-risk-record@1.0.0` from the exact risk
      fixture profile and add only `risk_record` at
      `.handbook/records/risk.yaml` with the frozen null-role/intake/renderer
      closure.
- [ ] Exercise intake, generic mutation/read/validate, and renderer against real
      canonical bytes.
- [ ] Prove no root default, generated command, inferred filename, Projection,
      or persistent view.
- [ ] Keep evidence independent from Decision Record or record an accepted
      pre-edit atomicity rationale.
- [ ] Record status/evidence: pending.

## P6 — Aggregate cleanup

- [ ] Confirm P1A–P1C and P2–P5 are independently green.
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
