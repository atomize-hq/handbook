# HCM-2.1 Implementation Plan

## Status

Planning-only. This plan becomes future implementation authority only with the
review-clean [`../SPEC.md`](../SPEC.md) packet and a separately selected
top-level HCM-2.1 implementation session. No task is executed in the planning
session that creates or closes this packet.

## Delivery strategy

Use one vertical, test-first Project Context cutover. Each step keeps the
workspace buildable and preserves HCM-1 profile/registry/setup/doctor truth.
Editing agents run sequentially because engine, compiler, flow, and CLI changes
share public types. Review agents are always fresh, isolated, and read-only.

## Dependency graph

```text
P0 entry + impact refresh
  -> P1 RED canonical-record/renderer proof
  -> P2 engine canonical owner
  -> P3 authoring + canonical input template
  -> P4 setup/doctor integration
  -> P5 flow bridge + selected source
  -> P6 end-to-end cutover + regression wall
  -> P7 control-pack truth
  -> P8 fresh review/remediation loop
  -> P9 primary commit + parent closeout
```

## P0 — Revalidate entry and freeze the implementation subject

1. Resolve repo root, branch, HEAD, worktree status, selected HCM-1.4 handoff,
   exact HCM-1.2 definition bytes, and the current packet fingerprint.
2. Reread the required skill chain and exact `00`-`08` authority sections.
3. Refresh GitNexus and run upstream impact on every existing symbol proposed
   for edit. Warn before HIGH/CRITICAL edits and stop on out-of-packet blast
   radius.
4. Prove the exact selected Project Context instance/kind/schema/path closure
   and the literal 29-member definition manifest before code changes.

**Checkpoint:** clean attributed entry, exact dependency proof, no code edit,
and a frozen allowed-path list.

## P1 — Write failing owner-contract tests first

1. Add engine RED tests for the exact canonical record, duplicate-safe parse,
   selected-schema validation, typed roundtrip, the packet's closed-emitter
   boundary YAML bytes, fixed Markdown boundary bytes, exhaustive plain-text
   transform table, and independent fingerprints.
2. Add negative tests for malformed/duplicate/multi-document/non-object input,
   constants, missing/unknown fields, bounds, stable refs, control syntax,
   descriptor binding, size, unsafe paths, symlinks, non-regular files, retained
   observation, typed/render reasons, different-byte races, and identical-byte
   inode ABA.
3. Add compiler/CLI RED tests for validate-only, atomic authoring, existing-
   truth policy, result fingerprints, and zero legacy-Markdown access.
4. Add setup/doctor RED tests for the exact selected path, null/non-null doctor
   row, `1.1.0` schema, three exact closure/race reasons, one retained
   observation, and unchanged HCM-1.4 decision fields.
5. Add Environment Inventory RED tests for selected-YAML reference success and
   missing/invalid/unsafe/null/retired/mismatched reference refusal.
6. Add flow RED tests for the exact owned-path/nullable-fingerprint/Rendered-
   mode DTO, source-versus-rendered budget domains and thresholds, manifest/
   freshness/log bytes, required selected truth, and legacy-input irrelevance.
7. Add native non-Unix RED coverage for validate-only and fail-before-mutation
   author/doctor/flow behavior.

**Checkpoint:** failures are contract failures, not fixture/setup noise.

## P2 — Land the engine-owned canonical Project Context boundary

1. Replace the old rich Project Context author input/timestamp renderer with
   `CanonicalProjectContext` and the exact parser/closed-emitter/plain-text-
   transform/renderer contract.
2. Reuse the selected registry for structural validation and existing bounded
   no-follow reads; retain one file observation through decode/render/hash and
   final identity/byte stability; do not duplicate schema or path policy.
3. Convert the exact engine identity/ingest-issue/baseline-validation path
   fields to owned `String`; adapt freshness sorting/encoding by borrow only and
   prove byte-identical `reduced-v1-m8` preimages for fixed siblings.
4. Bind only `project_context` / exact kind / exact schema / descriptor path.
5. Derive exact observed/written source and rendered-output fingerprints.
6. Add exact typed-decode/render/observation-change inspection reasons and
   delete obsolete rich-input normalization, fake-marker, timestamp, and old
   Markdown validator exports after all selected callers move.

**Checkpoint:** engine positive/negative/golden/security tests pass; HCM-1.1-
HCM-1.4 engine tests and package-definition manifest remain exact.

## P3 — Cut authoring and inputs directly to canonical YAML

1. Rewire compiler authoring to resolve selected decisions, parse/render before
   mutation, recheck under the retained lock, and atomically write only the
   descriptor-owned YAML path.
2. Preserve file/stdin and `--validate` command grammar; replace the input
   schema rather than translating it.
3. Return path, byte length, source fingerprint, rendered-output fingerprint,
   and `text/markdown` media type from the typed result.
4. Make the author result path an owned `String` cloned/moved only from the
   selected decision; remove the legacy fixed Project Context path constant and
   every re-export without a replacement alias.
5. Cut the core Project Context input template and live-skill fixture to the
   exact canonical record.
6. Delete old Project Context timestamp/test hooks and prove the legacy Markdown
   path is never read, written, renamed, or used for overwrite eligibility.
7. On non-Unix, preserve validate-only but refuse non-validate authoring before
   lock, directory, temp, or write; never invoke the fallback writer.
8. Cut only Environment Inventory's Project Context preflight/input/rendered
   reference to the exact selected YAML path, with focused refusal tests and no
   other Environment Inventory contract change.

**Checkpoint:** engine/compiler/CLI Project Context author API/result/path suites
and live-skill author smoke pass with exact bytes, no legacy path export, and no
sibling authoring changes.

## P4 — Prove setup and extend doctor

1. Run setup tests first. Keep production setup unchanged when its current
   selected structural behavior satisfies the packet.
2. Extend the doctor report to schema `1.1.0` with the exact nullable
   `project_context` fingerprint row and the packet's three exact inspection
   reasons.
3. Project the engine-owned retained/stability-checked observation; do not
   reopen, reparse, rerender, or rehash in compiler/CLI code.
4. Preserve all HCM-1.4 condition, role, capability, applicability, inspection,
   readiness, error, JSON framing, human-output, and exit mappings.
5. Prove different-byte substitution and identical-byte inode ABA cannot emit
   fingerprints and force the exact invalid row/status.
6. Add only the three exact new reason-name arms to CLI setup rendering and
   prove every other setup byte/outcome remains unchanged.

**Checkpoint:** complete setup/doctor/compiler/CLI regressions pass and setup
writes no artifact.

## P5 — Install the bounded Project Context flow bridge

1. Make packet path carriers owned strings where the selected descriptor path
   requires it; preserve fixed sibling semantics beyond the enumerated
   Environment Inventory reference.
2. Add `BR-HCM-2-PILOT-FLOW-01`: load Project Context from selected decisions,
   block when required truth is absent/invalid, render in memory, and project
   the exact owned-path DTO, source/render fingerprints, and `Rendered` mode.
3. Ensure old Project Context Markdown never enters manifest, freshness,
   blockers, summaries, sections, notes, fixture lineage, or decision logs.
4. Preserve Charter, Environment Inventory content authority, and Feature Spec;
   make only the enumerated Project Context DTO, source-versus-rendered budget,
   manifest/decision-log, fixture-lineage, refusal, and rendering changes plus
   Environment Inventory's reference-only path cutover; preserve freshness
   schema `reduced-v1-m8`, generation `1`, and its existing source-only inputs
   without source length or rendered values.
5. Keep pipeline/stage/external pipeline consumer code unchanged and record the
   bridge deletion gate for HCM-2.4.
6. Carry owned paths through packet, budget, all path-bearing flow resolver
   subject/next-action variants, and compiler subject/next-action variants by
   move/clone only; prohibit leaking, interning, fixed adapters, or lifetime
   coercion.
7. Advance flow and compiler C04 result version together to
   `reduced-v1-m8.2`; prove compiler acceptance of `.2` and exact rejection of
   `.1` while C03 remains `reduced-v1-m8` generation `1`.

**Checkpoint:** flow/compiler/CLI exact DTO, full owned-path carrier, C04
version, and threshold goldens plus selected legacy-irrelevance probes pass;
frozen pipeline tests remain green.

## P6 — Run the full implementation proof wall

1. Run focused engine, compiler author/setup/doctor/resolver, flow, CLI author/
   doctor/setup, live-skill, install-smoke, and direct-cutover tests.
2. Run negative no-follow, retained-observation/ABA, duplicate YAML, bounds,
   deterministic golden, fingerprint-domain, non-Unix refusal, Environment
   Inventory selected-reference, no-persistent-Markdown, and legacy-
   irrelevance probes.
3. Run all applicable HCM-1.1-HCM-1.4 tests, full workspace tests, formatting,
   Clippy, docs, Windows target, package-definition, package archive,
   handoff-validator, archive, scope, secret, whitespace, and diff gates.
4. Record raw command/results in one immutable HCM-2.1 proof wall and bind an
   exact sorted path/SHA-256 subject manifest.

**Checkpoint:** complete proof wall green with every unavailable platform proof
named honestly; no promotion yet.

## P7 — Update only affected control-pack truth

1. Update `00`, affected `03` rows, `04` HCM-2.1/Phase-2 status, and `06` gates,
   bridge row, and regression evidence to exact landed truth.
2. Close `PG-YAML-01` only if real product-path proof supports it. Keep
   `PG-ARTIFACT-01`, `PG-YAML-02`, and every sibling gate open at the exact
   remaining boundary.
3. Mark implementation tasks complete only after their evidence exists.
4. Rerun the full proof wall after documentation changes.

**Checkpoint:** code, tests, proof, and control-pack classifications agree.

## P8 — Fresh review and remediation loop

1. Create an immutable schema-valid final review dispatch bound to the complete
   final subject and proof wall.
2. Spawn one fresh isolated read-only built-in `default` reviewer and require
   findings first by Critical, Required, Optional, Nit.
3. Validate every finding against live truth. Repair valid findings in the
   parent, rerun the full wall, create a new dispatch/fingerprint, and use a
   different fresh reviewer.
4. Repeat until one complete-subject reviewer returns `CLEAN` or a genuine stop
   condition is met. Do not change subject bytes after CLEAN.

**Checkpoint:** final clean review binds the exact staged implementation
subject fingerprint.

## P9 — Commit and close the parent orchestration

1. Run staged GitNexus change detection, exact staged-path equality, manifest
   replay, `git diff --cached --check`, secret scan, and whitespace scan.
2. Commit the reviewed HCM-2.1 implementation/control-pack subject with one
   scoped Conventional Commit.
3. Create one parent-owned v1.2 completed handoff referencing the primary
   commit and proof-relevant delegated runs; rebuild and validate the ledger in
   all three modes.
4. Commit only the handoff/ledger mechanical closeout in a second commit.
5. Stop. Do not start HCM-2.2.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Frozen schema cannot express the retired rich input | direct canonical-record input; no schema edit or lossy mapper; stop if product requirements demand more fields |
| Persistent Markdown becomes a peer authority | render in memory only and prove no Markdown write |
| Environment Inventory keeps a dangling Markdown dependency | exact selected-YAML reference-only cutover with positive/refusal and all-three smoke proof |
| Doctor combines two observations or hides closure failure | one retained engine observation, exact reason precedence, and identity/byte ABA proof |
| YAML/Markdown fingerprints vary by library choices | closed scalar/framing/normalization algorithms plus literal boundary goldens |
| Flow accidentally continues reading old Markdown | selected-path bridge plus adversarial conflicting-legacy fixtures and decision-log scans |
| Flow DTO/budget semantics drift | exact field/domain table, effective-byte domains, and threshold/golden proof |
| Non-Unix writer weakens atomicity | validate-only stays portable; mutation refuses before filesystem access; native Windows proof required |
| Mixed-family cutover becomes permanent | named `BR-HCM-2-PILOT-FLOW-01` with HCM-2.4 deletion proof |
| Doctor reports stale fingerprints after a race | bind load/render to one observed byte read and fail invalid on disagreement |
| Setup starts materializing content | no planned setup edit; explicit no-write and rewrite-refusal proof |
| Public types drift silently | exact version/fingerprint fields, impact analysis, compiler/CLI/flow consumer tests |
| Package definitions drift | literal 29-member set/size/SHA-256/byte replay before and after |
