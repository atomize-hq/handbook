# HCM-2.3 implementation proof wall

## Authority and bounded context

- selected handoff: `20260722T042100Z--HCM-2-3--orchestration--planning-completed`
- entry branch: `codex/hcm-2-3-planning`
- entry HEAD: `3c49fa2c6d653f4b1a703d1d5c5196147b003533`
- active packet: none; the user's fresh top-level selection supplies the exact implementation authority required by the selected review-clean planning handoff
- allowed horizon: HCM-2.3 only; HCM-2.4, Phase 3+, Phase 4 SDK/transport, package-default expansion, and preservation worktrees/archives are excluded

## Entry proof

- worktree was clean at entry and branch/HEAD matched literally
- selected handoff, planning primary `4155e5cc`, planning closeout `3c49fa2c`, HCM-2.2 primary `6766d3ed`, and HCM-2.2 closeout `5c31eee` are ancestors of entry HEAD
- handoff validator: PASS (52 records, 265 dispatches, 8 legacy records, 52 ledger rows)
- v1 admission self-test: PASS
- orchestration self-test: PASS
- archive-boundary self-test and normal archive-boundary validation: PASS
- Rust/Cargo: `1.89.0`; default Python: `3.11.9`; WSL validation Python: `3.12.3` with `jsonschema 4.10.3`
- package-owned `crates/engine/definitions`: no `registry-brief` or `registry_brief` member

## Frozen dependency hashes

| Path | SHA-256 |
|---|---|
| `crates/engine/tests/fixtures/hcm_1_1_custom_kind/schema-entry.yaml` | `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536` |
| `crates/engine/tests/fixtures/hcm_1_1_custom_kind/root.schema.json` | `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510` |
| `crates/engine/tests/hcm_1_3_artifact_registry.rs` | `9781ac49d4ae41eed59b8ccf39ebc026e14bd0fdebb0c50accd93de3b0748c36` |
| `crates/engine/src/profile_builtins.rs` | `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1` |
| `crates/cli/src/main.rs` | `70c324c75c441f97bc81a940833cd88196700f9a48ffa6e6a729307b8c986af3` |
| `crates/engine/src/artifact_instance.rs` | `128eab60a3a16f08cd0966f076a62fa6bbbd4df20c9d0f5f6832ec53983bfa1d` |

## Live impact gate

GitNexus was refreshed to entry HEAD. Its FTS query index is unavailable, so concept queries returned degraded empty results; symbol context and upstream impact remain available and were used. `ArtifactInstanceRegistry::resolve` is CRITICAL: 8 direct callers, 427 total upstream symbols, 37 execution processes, and 20 modules. Direct callers include both profile resolution sites, replacement resolution, and focused test helpers. The only authorized existing-symbol edit is the packet's one-field relaxation for non-Charter `intake_definition_ref`; any broader dependency relaxation or need to edit another CRITICAL resolver stops this run.

## Execution ledger

This log is append-only during the run. Each increment records its RED cause, GREEN proof, regression scope, formatting/diff checks, graph/scope result, and any immutable internal dispatch/run identity before the next increment begins.

### Increment 0 — frozen baseline

- immutable implementation dispatch `20260722T123845Z--HCM-2-3--registration-kernel-implementation` started as fresh default built-in subagent `/root/hcm23_registration_kernel`; status after dispatch: `running`
- immutable implementation dispatch `20260722T123846Z--HCM-2-3--generic-lineage-implementation` started as fresh default built-in subagent `/root/hcm23_generic_lineage`; status after dispatch: `running`
- both dispatches share the review-clean HCM-2.3 packet manifest `sha256:de116ba4567332ff9a3e9b5da11320987f04b7b87241f5d9e3b6fb2e19042ba1`
- post-dispatch handoff/dispatch validation: PASS (52 records, 267 current dispatches, 8 admitted legacy dispatches, 52 ledger entries)
- no behavior or existing production symbol changed in Increment 0

### Increment 3 — descriptor-selected optional intake

- fresh GitNexus impact immediately before edit: `ArtifactInstanceRegistry::resolve` CRITICAL, 8 direct callers, 427 total upstream symbols, 37 affected processes, 20 modules; direct production callers are `resolve_profile_selection` and `replace_instances`, with broad setup/doctor/pipeline/Charter transitive reach
- RED: `cargo test -p handbook-engine --test artifact_instances non_charter_descriptor_accepts_only_a_typed_generic_intake_reference -- --exact` failed only at `artifact_instances/1/intake_definition_ref` with `UnsupportedDependency`
- GREEN: the resolver's non-Charter later-owned rejection list no longer rejects only `intake_definition_ref`; parsing remains typed through `ExactDefinitionRef`, while lifecycle, renderer, projection, overlay, extensions, and the frozen Charter 1.1 closure are unchanged
- focused GREEN: exact new test PASS; complete `artifact_instances` integration target PASS (10/10)
- operation-context closure remains mandatory before content use; unresolved, missing, wrong-kind, and wrong-schema intake refs are not treated as admitted by this change

### Increment 11 — CLI grammar slice

- GitNexus impact before the existing CLI registration edit: top-level `Command` enum LOW, 0 upstream symbols/processes; `Command::run` LOW, 0 upstream symbols/processes
- RED: the exact artifact help test failed with `unrecognized subcommand 'artifact'`
- GREEN: the fixed ten-command grammar, explicit required `--repository-root`, exact kind/instance selectors, bounded `path|-` data inputs, committed intake pair, expected-current option, and `--json` are registered
- CLI mutation help has no idempotency-key argv option; generic help has no example-kind or approval command
- focused grammar target PASS (4/4); product adapter behavior remains RED/pending until the engine registration/kernel integration is present

### Increment 1 — typed intake source class

- fresh GitNexus impact: `ProfileSelectionRequest` CRITICAL, 5 direct callers, 441 upstream symbols, 37 processes, 20 modules; the most exposed existing constructor is `shipped_profile_request` at CRITICAL, 1 direct/368 upstream/36 processes/20 modules
- the expansion is packet-reviewed and literal: one additive `intake_definition_sources: Vec<DefinitionSourceBinding>` field; existing shipped constructors receive an empty array and the existing profile resolver/fingerprint path does not consume it
- RED: focused compile failed only because `ProfileSelectionRequest` lacked `intake_definition_sources` (`E0560`/`E0609`)
- GREEN: typed repository-path intake binding round trip PASS; existing `profile_selection` integration target PASS (19/19); shipped profile-decision proof PASS
- existing constructor edits were mechanical empty-array preservation; GitNexus returned MEDIUM/LOW for indexed test constructors and UNKNOWN only for cfg-gated test helpers absent from the active index

### Increment 2 — explicitly authorized schema-root domain separation

- first implementation attempt stopped before editing when the complete fixture proved that the existing shared allowed-root check applied the HCM-2.3 repository root to compile-time built-in schema documents; no weakened request or fixture was accepted
- fresh user authorization permits exactly one bounded HCM-2.3 separation: built-in documents remain beneath fixed package root `definitions/schemas` and come only from `profile_builtins::schema_document`; repository documents remain beneath the request-authorized roots, with the fixture request retaining only `.handbook/definitions/schemas`
- exact GitNexus command for each permitted private symbol: `npx gitnexus impact <exact-uid> --direction upstream --depth 4 -r C:\hcm22ar-doc-repair`
- `Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1`: GitNexus LOW but treated as CRITICAL by authorization; 2 direct/total affected symbols, 1 resolver process, 2 modules
- `Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1`: GitNexus LOW but treated as CRITICAL by authorization; 1 direct caller, 3 total affected symbols through depth 2, 1 resolver process, 2 modules
- RED: the new built-in source-domain test failed with `LocalReferenceOutsideRoot` for the allowlisted `definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json`; the complete fixture likewise failed at that built-in document while its request contained only `.handbook/definitions/schemas`
- GREEN: only the private source-kind containment decision changed; `ClosureLoader::load_document`, `SchemaRegistry::load_admitted_deferred_fingerprints`, and every high-level resolver remain otherwise unchanged
- focused GREEN: source-domain unit tests PASS (5/5); exact mixed fixture proof PASS (1/1); direct repository schema-registry integration target PASS (19/19)
- containment remains before local-cache lookup, shared-cache lookup, document limits, built-in allowlist lookup, and repository access; prefix collisions and outside-root paths retain `LocalReferenceOutsideRoot`; a contained absent built-in retains `LocalReferenceMissing`
- the test-only adapter that appended `definitions/schemas` was removed, and the positive fixture now asserts the sole request root literally

### Increment 4 — registration closure and exact real-path fixture

- the fresh registration-kernel implementation agent returned only its bounded source/test changes and proof report; the parent inspected and integrated those bytes without widening the dispatch
- RED: the new complete repository-profile selection and registration tests initially failed because the intake source class and operation context did not exist and because non-Charter descriptors rejected their typed intake ref
- GREEN: the fixed `.handbook/profile-selection.json` record closes every required typed source class, retains exactly `.handbook/definitions/schemas` as its sole request-authorized schema root, and resolves the shipped-plus-custom profile, schema, kind, descriptor, and intake closure
- exact registration proof PASS (9/9); exact selection-request real-path proof PASS (1/1); existing profile-selection proof PASS (19/19); direct schema-registry proof PASS (19/19)
- the package-owned definitions tree still contains no registry-brief definition and the HCM-1.3 fixture inputs remain the repository-owned custom source

### Increment 5 — generic read, validation, intake, and canonical YAML

- the fresh generic-lineage implementation agent returned its bounded new-module/test changes; all service behavior remained selected by exact kind ref and instance ID rather than a kind enum, filename, or command-specific semantic branch
- RED: list/read/validate/intake-definition-read and all three intake evaluation modes failed at the missing generic operation service; candidate previews and deterministic canonical YAML likewise failed before their owners existed
- GREEN: the operation context closes the selected profile, schema, kind, descriptor, and optional intake before canonical content use; safe reads retain exact path, size, mutation, YAML, and structural-validation protections
- guided-adaptive, express, and agent-assisted pure evaluations converge on identical normalized candidate content while retaining explicit mode metadata and producing no state, journal, key, evidence, or receipt writes
- deterministic YAML emits the JSON data model with stable UTF-8 key order and LF, rejects non-JSON YAML features, obeys the output bound, and passes parse/emit/parse equality and permutation/Unicode/ambiguous-scalar proofs

### Increment 6 — immutable generic lineage and compare-and-write promotion

- RED: intake append, candidate validate, candidate append, promotion, recovery, replay, key-retention, refusal, race, and restart cases failed before the separate generic store and mutation coordinator existed
- GREEN: finalized intake, validation result, candidate `1.4`, and promotion `1.2` records are immutable JCS+LF records under the generic instance store; one ordered repository lock covers recovery, currentness evaluation, append, and compare-and-write
- candidate validation reads one committed intake under the store lock and proves a zero-write delta; candidate append recomputes the candidate from the committed intake; promotion recomputes the complete candidate/definition closure, revalidates deterministic YAML, and replaces canonical truth only when the expected basis still matches
- pre-establishment failures leave zero durable writes; every established refusal is evidence-backed, retained, restartable, and exactly replayable without re-evaluation; raw idempotency keys are absent from public result records and canonical output
- same-key/same-request races converge, same-key/different-request races conflict, stale and ABA bases refuse, and recovery exposes no subordinate or partial authority
- focused generic lineage proof PASS (12/12), including crash-prefix, malformed-state, inventory, mutation, replay, and process-concurrency cases

### Increment 7 — stable CLI adapter and actual-binary proof

- RED: the fixed grammar originally refused `artifact`; after grammar registration, each product operation failed at its explicit unimplemented adapter stub before its engine owner was connected
- GREEN: the ten fixed commands route typed repository-root, kind-ref, instance-ID, path/stdin values, committed intake identity, expected-current basis, and internal raw key to the engine without direct CLI state writes or per-kind dispatch
- retained established refusals render their closed typed JSON result and return a nonzero process status; exact replay returns the same result after a fresh process; pre-establishment failures return nonzero with no durable state
- actual-binary proof covers list/read/validate/intake-definition-read, all three pure intake modes, intake append, zero-write candidate validation, candidate append, promotion, exact replay, retained refusal, path input, stdin input, and fresh-process reads
- CLI package proof PASS: unit/integration groups include `98/98` surface tests and exact HCM-2.3 actual-binary proof `5/5`

### Increment 8 — final regression and preservation wall before review

- `cargo fmt --all -- --check`: PASS
- `cargo clippy -p handbook-engine -p handbook-cli --all-targets -- -D warnings`: PASS
- `cargo test -p handbook-engine --quiet`: PASS, including the complete HCM-2.2 fault/recovery and schema regressions
- `cargo test -p handbook-cli --quiet`: PASS
- `cargo test --workspace --quiet`: PASS, exit 0 after the complete workspace wall
- `cargo tree -p handbook-engine -e features`: PASS; no dependency or feature addition
- `cargo package -p handbook-engine --allow-dirty --no-verify`: PASS (191 files, 2.9 MiB unpacked, 497.6 KiB compressed)
- `cargo package -p handbook-cli --allow-dirty --no-verify`: unavailable at unchanged baseline Cargo metadata because the existing path dependency `handbook-compiler` has no version; no Cargo manifest or publication surface is authorized or changed by HCM-2.3
- archive-boundary self-test and normal validation: PASS
- v1 handoff-admission self-test and orchestration-contract self-test: PASS
- normal handoff validation: PASS with 3 record schemas, 2 dispatch schemas, 2 templates, 52 records, 267 current dispatches, 8 admitted legacy dispatches, and 52 ledger entries; validation used an ignored `target/hcm23-proof-objects` object overlay for the one historical commit absent from the shared object database, without modifying a preservation worktree, archive, or shared ref
- duplicate-safe JSON/schema/vector semantic replay: PASS (`duplicate-safe=6`, `schemas=3`, `selection=1`, `runtime=4`, `control=39`, `schema_vectors=52`)
- Markdown reference/link/fence replay: PASS (`markdown_files=24`, `local_links=51`, balanced fences, required references present)
- `git diff --check`: PASS
- unauthorized-scope inspection: no package-owned definition, Cargo manifest, SDK/transport, Phase 3+, HCM-2.4, preservation-worktree, or archive change

### Increment 9 — pre-review graph and subject gate

- exact command: `npx gitnexus detect-changes --scope all --repo C:\hcm22ar-doc-repair --limit 200`
- result: PASS with an expected HIGH warning; 20 indexed modified files, 42 indexed changed symbols, 12 affected execution flows
- reported flow reach includes the already-authorized `ProfileSelectionRequest`/`shipped_profile_request` construction paths, CLI `Command`/`run`, and schema-location/resolve paths; the new untracked private modules are not represented until reindexing
- no unexpected high-level CRITICAL resolver edit is present; in particular, `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged
- exact changed-path audit: 37 paths before the review dispatch, zero forbidden paths, and `git diff --check` PASS

### Increment 10 — fresh complete-subject Review 1

- fresh isolated default reviewer `/root/hcm23_complete_review_1` admitted the
  immutable 43-path Review 1 subject at
  `sha256:2027b8b283abc807b0ad2cb3c3eeef87e17cb3fad3aaa907fde48203f4cc350c`
- immutable dispatch:
  `20260722T155754Z--HCM-2-3--fresh-complete-implementation-review-1`
- verdict: `CHANGES_REQUIRED`; three Critical and ten Required findings; no
  finding was waived and no `CLEAN` or completion claim was retained
- exact accepted findings and remediation ownership are recorded in
  `20260722T155754Z--implementation-review-1-remediation.md`

### Increment 11 — Review 1 remediation

- repository authority is recovered and identity-locked before generic-store
  recovery, lookup, planning, and establishment; identity is revalidated before
  replay lookup and again immediately before intent publication
- every ordinary generic read now takes the same recovery-first lock path, so a
  markerless installed promotion is completed before canonical output is read
- strict repository reads retain native handles; Windows retains every parent
  handle without delete sharing and denies same-path mutation, while Unix uses
  no-follow descriptor-relative open/create/rename/unlink operations; inventory
  remains bounded and every consumed file is admitted and read through the
  strict retained handle
- compile-time BuiltIn intake bindings are loaded only from
  `profile_builtins::definition`; repository intake bindings remain trusted
  repository reads; generic eligibility stays false for compatibility-only
  BuiltIn definitions
- registry meta-validation proves exact finite schema leaves, exact target
  coverage, type compatibility, and no ancestor/descendant overlap before a
  generic intake becomes usable
- blocked known-unknown/contradicted coverage remains a bounded auditable
  evaluation with no normalized content or field-source authority; completed
  evaluations are structurally validated against the selected instance schema
- intake persistence accepts one through fifteen distinct subordinate values,
  deduplicates equal values by fingerprint, retains every coverage row, and
  proves the semantic record's exact subordinate closure
- candidate mismatch now uses the admitted `StaleBasis` / `Currentness` /
  non-null unequal fingerprint tuple and is proven through the actual CLI with
  retained refusal replay and zero authoritative outputs
- transaction, family, idempotency, semantic, closure, and per-instance
  inventory grammars/count/byte/depth limits are exact; verified descriptors
  equal the exact intent projection; ledger/tombstone fields and retained
  result bindings are closed; time is sampled under lock; establishment fsyncs
  its transaction family before rename
- control-pack wording now describes this as an uncommitted review-pending
  subject; candidate classifications remain conditional on clean review and
  primary commit

### Increment 12 — schema-root authorization and exact impact evidence

The user explicitly authorized one additional bounded CRITICAL separation.
BuiltIn schema documents and local references are contained only beneath the
fixed compile-time root `definitions/schemas` and obtain bytes only from the
unchanged `profile_builtins::schema_document` allowlist. Repository documents
and local references are contained only beneath request-authorized roots. The
HCM-2.3 request contains exactly `.handbook/definitions/schemas`; the removed
test adapter does not append `definitions/schemas`. Existing direct
repository-registry callers may still authorize their own roots.

Exact pre-edit GitNexus evidence, all treated as CRITICAL even where the graph
reported lower risk:

| Exact symbol UID / command target | Command | Result |
|---|---|---|
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | LOW; 2 direct/total affected symbols; 1 resolver process; 2 modules |
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | LOW; 1 direct caller; 3 affected symbols through depth 2; 1 resolver process; 2 modules |
| `Enum:crates/engine/src/schema_registry.rs:ResolvedBindingJsonType` | `npx gitnexus impact ResolvedBindingJsonType --direction upstream --depth 3 --repo C:\hcm22ar-doc-repair --file crates/engine/src/schema_registry.rs` | LOW; 0 direct callers, 0 affected processes, 0 modules; visibility-only test-adapter compatibility |

`SchemaRegistry::load_admitted_deferred_fingerprints` and every other
high-level CRITICAL schema resolver remain unchanged. Containment remains
before request-cache reuse, document limits, BuiltIn allowlist lookup, and
repository access. Root, descendant, prefix collision, outside-root, cached,
missing-allowlist, traversal, symlink, non-regular, mutation, and unsafe-path
proofs retain their exact failure classifications.

### Increment 13 — post-remediation proof wall before Review 2

- `cargo fmt --all -- --check`: PASS
- Windows `cargo clippy -p handbook-engine -p handbook-cli --all-targets -- -D warnings`: PASS
- WSL `cargo clippy -p handbook-engine --all-targets -- -D warnings`: PASS
- source-domain schema tests: PASS 5/5 on Windows and WSL; direct repository
  schema-registry integration: PASS 19/19
- registration/kernel: PASS 11/11; generic lineage: PASS 19/19 on Windows and
  WSL; actual-binary HCM-2.3 CLI: PASS 5/5
- complete focused HCM-2.2 wall: PASS 89 engine plus 10 CLI tests; unchanged
  Charter candidate `1.3`, result `1.0`, intent `1.2`, authority, recovery, and
  installed product paths remain green
- `cargo test -p handbook-engine --quiet`: PASS; `cargo test -p handbook-cli
  --quiet`: PASS; `cargo test --workspace --quiet`: PASS
- `cargo tree -p handbook-engine -e features`: PASS with no dependency or
  feature addition
- `cargo package -p handbook-engine --allow-dirty --no-verify`: PASS, 191
  files, 3.0 MiB unpacked, 509.2 KiB compressed
- `cargo package -p handbook-cli --allow-dirty --no-verify`: unchanged baseline
  limitation reproduced exactly because path dependency `handbook-compiler`
  has no version; no Cargo/package-surface change is authorized or present
- archive-boundary normal and negative self-test: PASS
- handoff normal validation: PASS with 3 record schemas, 2 dispatch schemas, 2
  templates, 52 records, 268 current dispatches, 8 admitted legacy dispatches,
  and 52 ledger entries; v1 admission and orchestration-contract self-tests:
  PASS
- unchanged SHA-256: `profile_builtins.rs`
  `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`;
  HCM-1.1 schema entry
  `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`;
  HCM-1.1 root schema
  `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`
- Windows default Python lacks `jsonschema`; all handoff modes used WSL Python
  3.12 with the real Windows worktree gitdir explicitly bound for normal
  validation, while the orchestration self-test ran without inherited gitdir
  state so its temporary-repository proof remained isolated
- `git diff --check`: PASS; analyzer-only GitNexus onboarding rewrites were
  restored byte-for-byte from entry HEAD and are excluded from the subject
- pre-Review-2 `npx gitnexus detect-changes --scope all --repo
  C:\hcm22ar-doc-repair --limit 500`: CRITICAL, 22 indexed modified files, 62
  changed symbols, and 257 affected flows; the reach is the expected
  registry/profile/CLI graph plus generated fingerprint-symbol noise, while new
  untracked private modules remain incompletely represented; no forbidden
  high-level schema resolver edit is present
- this is executable pre-review evidence, not `CLEAN`, landed proof, a primary
  commit, or authority for HCM-2.4 or later work

### Increment 14 — fresh complete-subject Review 2

- immutable dispatch:
  `20260722T175647Z--HCM-2-3--fresh-complete-implementation-review-2`
- fresh isolated Review 2 admitted all 47 manifest paths and recomputed
  aggregate
  `sha256:322ef3a897cabe756ff44fd6f1cc5b51555d75fae2c8c60edfd27109f1e08abd`
- verdict: `CHANGES_REQUIRED`, four Critical and five Required findings; no
  finding was waived and no prior proof was treated as `CLEAN`
- accepted findings covered public raw-store authority bypass, recovery trust
  in self-fingerprinted journals/runtime bytes, missing immediate context/plan
  revalidation, native file/install races, structural-refusal mapping, public
  read bypasses, exact empty-shell recovery, canonical scratch recovery, and
  proof overclaim
- exact finding-to-repair evidence is recorded in
  `20260722T195055Z--implementation-review-2-remediation.md`

### Increment 15 — Review 2 remediation and complete proof wall

- raw journal/store APIs are crate-private; public repository reads expose only
  owned DTOs through one authority-held, recovery-first path
- intents retain the complete typed request subject; exact plan equality is
  revalidated immediately before establishment; recovery validates the full
  nested journal and every staged/installed runtime record against current
  instance, context, schema closure, intake, definition, and canonical
  authority
- strict repository reads retain native file/directory identity, detect Unix
  and Windows hard-link aliases, reject same-size/time mutation, preserve
  bounded-read error precedence, and support extended-length Windows paths via
  a fail-closed provider fallback when `fsutil` cannot query them
- the exact empty pending shell is admitted; deterministic canonical scratch is
  crash-recoverable and the basis is rechecked immediately before publication;
  structurally invalid intake persists only the closed
  `StructuralValidationFailed` outputless refusal
- Windows focused proof: schema source-domain 5/5, direct schema registry
  19/19, generic lineage 33/33, registration 13/13, selection 1/1, and actual
  CLI 5/5
- full HCM-2.2 preservation proof: 16 engine targets / 89 tests and two CLI
  targets / 10 tests, all PASS
- `cargo fmt --all -- --check`: PASS; Windows all-target engine/CLI Clippy:
  PASS; complete engine and CLI package tests: PASS; complete workspace wall:
  PASS in 543.9 seconds
- WSL: engine all-target Clippy PASS; generic lineage 33/33, direct schema
  registry 19/19, and schema source-domain 5/5 PASS, including Unix hard-link,
  descriptor-relative path substitution, and restored-mtime/ctime mutation
  proof
- dependency feature tree: PASS with no manifest/lock/feature addition; engine
  package PASS, 191 files, 3.1 MiB unpacked / 524.5 KiB compressed; CLI package
  reproduces the unchanged `handbook-compiler` missing-version baseline
  limitation
- archive boundary normal/self-test PASS; handoff normal validation PASS with 3
  record schemas, 2 dispatch schemas, 2 templates, 52 records, 269 current
  dispatches, 8 admitted legacy dispatches, and 52 ledger entries; v1 and
  orchestration self-tests PASS
- duplicate-safe validation PASS for six JSON artifacts and three Draft
  2020-12 schemas; Markdown validation PASS for 26 files, 53 local links, and
  balanced fences
- preservation SHA-256 remains exact: `profile_builtins.rs`
  `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`,
  HCM-1.1 schema entry
  `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`,
  and HCM-1.1 root schema
  `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`
- exact changed-path inventory: 53 paths, zero Cargo/package-definition,
  archive, preservation, HCM-2.4, Phase 3+, or Phase 4 SDK/transport paths;
  `git diff --check` PASS
- this remains pre-Review-3, uncommitted evidence and does not claim `CLEAN`

### Increment 16 — pre-Review-3 graph and subject gate

- exact command: `npx gitnexus detect-changes --scope all --repo
  C:\hcm22ar-doc-repair --limit 500`
- result: expected CRITICAL reach; 26 indexed changed files, 68 indexed changed
  symbols, and 293 affected execution flows
- reported reach includes the already-authorized registry/profile/CLI paths,
  `CanonicalWorkspace`/`TrustedRepoFile`, schema resolution, and generated
  fingerprint-symbol noise; new untracked private HCM-2.3 modules are not fully
  represented until indexing
- `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged; no
  Cargo/package-definition, archive, preservation, HCM-2.4, Phase 3+, or Phase
  4 SDK/transport path is in the 53-path subject
- fresh Review 3 must independently admit the complete immutable subject; this
  graph result is not a `CLEAN` or success conclusion

### Increment 17 — fresh complete-subject Review 3

- immutable dispatch:
  `20260722T202942Z--HCM-2-3--fresh-complete-implementation-review-3`
- immutable subject fingerprint:
  `sha256:0cc34c001c046f0d8d0d5c9b350285b9ca699c07fccf70a3e80f2dd9483bd36d`
- verdict: `CHANGES_REQUIRED`; no finding was waived
- accepted findings: canonical scratch/installed namespace substitution,
  same-bytes restored-metadata basis ABA including recovery, incomplete engine
  recovery/inventory/concurrency matrices, and incomplete actual-CLI
  exit/restart/tombstone proof
- exact finding-to-repair evidence is recorded in
  `20260722T215228Z--implementation-review-3-remediation.md`

### Increment 18 — Review 3 remediation, focused evidence

- promotion intent and verified staging now bind a closed native publication
  observation over basis version plus scratch identity/version; native identity
  includes Windows file ID/creation time or Unix device/inode/birth time
- deterministic scratch and installed canonical substitution refuse for
  regular files, symlinks, and hard links; invalid scratch residue is preserved
  without rewrite or adoption; restored-mtime basis ABA refuses
- RED on WSL proved immediate Unix inode reuse could defeat device/inode-only
  identity; GREEN after filesystem birth time entered the native identity token
- exact engine post-marker prefixes/non-prefixes, every commit install/suffix
  fault, early commit intent-only refusal, same-key/different-request race, and
  all frozen inventory maxima/maxima+1 have executable tests
- actual binary proof now includes exact exit 0/1/2, unrelated cwd, missing
  selection, lowercase-only `absent`, every post-marker prefix, illegal
  non-prefixes with zero delta, and tombstone exact/different requests
- focused Windows and WSL native-substitution tests, frozen control-vector
  identity, all new CLI cases, formatting, and engine all-target compile pass
- complete post-remediation HCM-2.2/workspace walls, GitNexus change detection,
  and a fresh different-agent review remain pending; no `CLEAN` or commit is
  claimed

### Increment 19 — Review 3 remediation, complete proof wall

- the complete engine package wall passes: the 150-test unit target and every
  integration target, including generic lineage 42/42, registration 13/13,
  selection 1/1, and direct schema registry 19/19; elapsed wall time 537.6
  seconds
- the complete CLI package wall passes: the 98-test CLI unit target and every
  integration target, including actual-binary HCM-2.3 proof 8/8; elapsed wall
  time 196.5 seconds
- `cargo test --workspace --quiet` passes every package and target, including
  the repeated engine inventory/recovery and actual-process matrices; elapsed
  wall time 749.5 seconds
- the full HCM-2.2 regression suite remains green: 16 engine targets / 89
  tests and two CLI targets / 10 tests
- Windows `cargo fmt --all -- --check`, engine/CLI all-target Clippy with
  `-D warnings`, and engine all-target check pass; WSL engine all-target Clippy
  passes, and the focused regular/symlink/hard-link scratch and installed-path
  substitution cases pass with Unix native identity enforcement
- `cargo tree -p handbook-engine -e features` passes without manifest, lock,
  dependency, or feature changes; engine packaging passes with 191 files,
  3.2 MiB unpacked / 532.7 KiB compressed; CLI packaging reproduces only the
  unchanged `handbook-compiler` missing-version baseline limitation
- archive-boundary normal and negative self-test pass; handoff normal
  validation passes with 3 record schemas, 2 internal-dispatch schemas, 2
  templates, 52 records, 270 current internal dispatches, 8 admitted legacy
  dispatches, and 52 ledger entries; v1-admission and orchestration-contract
  self-tests pass
- duplicate-safe parsing passes for all six HCM-2.3 contract JSON artifacts;
  all three schemas pass Draft 2020-12 meta-validation; the current affected
  top-level plus complete HCM-2.3 Markdown set passes for 22 files, 48 local
  links, 3 anchors, and balanced fences
- preservation SHA-256 remains exact: `profile_builtins.rs`
  `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`,
  HCM-1.1 schema entry
  `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`,
  and HCM-1.1 root schema
  `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`
- exact subject inventory is 55 paths with zero Cargo/package-definition,
  archive, preservation, HCM-2.4, Phase 3+, or Phase 4 SDK/transport paths;
  `git diff --check` passes
- `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged;
  the HCM-2.3 request retains exactly `.handbook/definitions/schemas`, does not
  append `definitions/schemas`, and built-in bytes remain exclusively behind
  the unchanged compile-time allowlist
- GitNexus change detection and fresh complete-subject Review 4 remain pending;
  this evidence does not claim `CLEAN` or commit authority

### Increment 20 — pre-Review-4 graph and subject gate

- exact command: `npx gitnexus detect-changes --scope all --repo
  C:\hcm22ar-doc-repair --limit 500`
- result: expected CRITICAL reach; 26 indexed changed files, 117 indexed
  changed symbols, and 293 affected execution flows
- immediate replay after recording this gate remained CRITICAL at 26 indexed
  files and 293 affected flows while the generated changed-symbol projection
  normalized to 57; the index's generated-symbol count is not used as a
  success assertion
- reported reach remains the authorized registry/profile/CLI graph,
  `CanonicalWorkspace`/`TrustedRepoFile`, schema resolution, and generated
  fingerprint-symbol noise; new untracked private HCM-2.3 modules remain only
  partially represented by the entry-state index
- `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged and
  the 55-path subject contains no Cargo/package-definition, archive,
  preservation, HCM-2.4, Phase 3+, or Phase 4 SDK/transport path
- fresh Review 4 must independently admit the complete immutable subject; this
  graph result is not a `CLEAN` or success conclusion

### Increment 21 — fresh complete-subject Review 4

- immutable dispatch:
  `20260722T223824Z--HCM-2-3--fresh-complete-implementation-review-4`
- immutable subject fingerprint:
  `sha256:07ed199cc69106e2e1829469c75e3a2554ca2b588053fadbab118bf8fe107965`
- verdict: `CHANGES_REQUIRED`; no finding was waived
- accepted findings: pathname-only native observation/publication window,
  fresh scratch residue adoption/rewrite, caller-controlled intake consumer
  version, and missing shared engine owner byte-document bounds
- exact finding, impact, RED, repair, and GREEN evidence is recorded in
  `20260723T000600Z--implementation-review-4-remediation.md`

### Increment 22 — Review 4 remediation and complete proof wall

- fresh scratch is absent/create-new only; recovery scratch is read-only and
  must equal persisted retained-handle-bound bytes, identity, and version;
  equal/unequal residue refuses before intent establishment with zero delta
- canonical basis and scratch observations bind retained handle metadata to
  platform path identity/version; Windows canonical substitution is denied by
  the retained handle, Unix substitution is detected, and scratch is checked
  again after the deterministic final pre-rename hook
- the public intake API no longer accepts consumer version, engine `VERSION`
  is the only persisted value, and persisted validation requires it exactly
- a shared 1 MiB engine limit applies before parsing to evaluation and all three
  mutation owner documents; exact limit passes parsing and +1 refuses before
  repository/journal mutation; CLI imports the same limit
- registration kernel PASS 16/16; generic lineage PASS 45/45; complete engine
  package PASS in 551.4 seconds; complete CLI package PASS in 200.4 seconds;
  complete workspace PASS in 784.2 seconds
- HCM-2.2 remains green at 16 engine targets / 89 tests and two CLI targets /
  10 tests; Windows and WSL all-target Clippy, Windows format/check/diff, and
  dependency-tree proof pass
- engine package remains 191 files / 3.2 MiB / 537.0 KiB compressed; CLI
  packaging reproduces only the unchanged `handbook-compiler` version baseline
- archive normal/self-test and all three handoff modes pass (271 current
  internal dispatches); six JSON artifacts are duplicate-safe, three schemas
  meta-validate, and 23 Markdown files / 48 local links / balanced fences pass
- all three preservation hashes remain exact; the 57-path pre-Review-5 subject
  contains no Cargo/package-definition, archive, preservation, HCM-2.4,
  Phase 3+, or Phase 4 SDK/transport path
- GitNexus change detection and fresh complete-subject Review 5 remain pending;
  no `CLEAN` or commit is claimed

### Increment 23 — pre-Review-5 graph gate

- exact command: `npx gitnexus detect-changes --scope all --repo
  C:\hcm22ar-doc-repair --limit 500`
- result: expected CRITICAL reach; 26 indexed changed files, 92 indexed
  changed symbols, and 293 affected execution flows
- the generated reach includes the authorized registry/profile/CLI graph,
  `CanonicalWorkspace`/`TrustedRepoFile`, schema resolution, and entry-state
  fingerprint-symbol noise; new HCM-2.3 modules remain only partially visible
  to the entry-state index and their UNKNOWN impact was treated as CRITICAL
- `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged; no
  Cargo/package, archive, preservation, HCM-2.4, Phase 3+, or Phase 4
  SDK/transport path is present
- this graph result authorizes only immutable Review 5 dispatch and is not a
  `CLEAN`, commit, or closeout conclusion

### Increment 24 — fresh complete-subject Review 5

- immutable dispatch:
  `20260723T001130Z--HCM-2-3--fresh-complete-implementation-review-5`
- immutable subject fingerprint:
  `sha256:b07eef50d4b30966ce90459280f4a274656b7ef7c92c327adc493b0d79d317e1`
- verdict: `CHANGES_REQUIRED`; no finding was waived
- accepted findings: a final canonical-basis publication gap, orphan
  ledger/result state able to manufacture replay or expiry without reverse
  journal reachability, and fresh `CreateNew` adoption of equal uncited
  closure/semantic bytes
- exact finding, impact, RED, repair, and focused GREEN evidence is recorded in
  `20260723T013000Z--implementation-review-5-remediation.md`

### Increment 25 — Review 5 remediation, focused evidence

- fresh and recovery replacement publication now re-observe the exact
  fingerprint/native-version basis after the final deterministic hook and
  recheck the retained scratch immediately before publication
- changed, deleted, and same-byte/restored-mtime final-hook basis attacks all
  refuse without marker, result, ledger, or promotion authority
- pre-recovery reachability parses every ledger/result and admits controls or
  immutable artifact records only when exactly one valid pending or committed
  chain cites them; pending suffix records are checked against their exact
  deterministic journal projection before recovery mutation
- committed-only reachability after recovery requires exact set equality, and
  replay/expiry lookup independently verifies the addressed committed chain
- fresh `CreateNew` destinations must be absent before intent and are installed
  exclusively; recovery remains a distinct exact installed-subset path
- focused adversarial cases, all commit suffix faults, every installed subset,
  and all refusal prefixes pass; the engine unit target passes 152/152 and
  Windows engine all-target Clippy passes with `-D warnings`
- complete walls, GitNexus change detection, and fresh complete-subject Review
  6 remain pending; no `CLEAN` or commit is claimed

### Increment 26 — Review 5 remediation, complete proof wall

- final HCM-2.3 focused wall passes: generic lineage 49/49 in 279.41 seconds,
  registration kernel 16/16, selection request 1/1, and direct schema registry
  19/19
- complete engine package passes in 518.4 seconds, including unit 152/152 and
  lineage 49/49; complete CLI package passes in 228.2 seconds, including all
  eight actual-binary HCM-2.3 cases; complete workspace passes in 722.7 seconds
- exact HCM-2.2 regression remains green at 16 engine targets / 89 tests and
  two CLI targets / 10 tests
- Windows format/diff checks, engine all-target check, engine/CLI all-target
  Clippy with `-D warnings`, dependency tree, and WSL engine all-target Clippy
  pass; WSL retained-basis, retained-scratch, final canonical-hook, and reverse-
  reachability adversarial tests pass
- engine package remains 191 files / 3.2 MiB / 540.9 KiB compressed; CLI
  package reproduces only the unchanged `handbook-compiler` missing-version
  baseline; no Cargo manifest, lock, dependency, or feature changed
- archive normal/self-test and all three handoff modes pass; normal handoff
  validation reports 3 record schemas, 2 internal-dispatch schemas, 2
  templates, 52 records, 272 current internal dispatches, 8 admitted legacy
  dispatches, and 52 ledger entries
- all six HCM-2.3 JSON artifacts parse duplicate-safe and all three schemas
  meta-validate; 24 affected/slice Markdown files, 48 local links, and balanced
  fences pass
- preservation SHA-256 remains exact for `profile_builtins.rs`
  `e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`,
  HCM-1.1 schema entry
  `ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`,
  and HCM-1.1 root schema
  `f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`
- GitNexus change detection and fresh complete-subject Review 6 remain pending;
  no `CLEAN` or commit is claimed

### Increment 27 — pre-Review-6 graph and subject gate

- exact command: `npx gitnexus detect-changes --scope all --repo
  C:\hcm22ar-doc-repair --limit 500`
- result: expected CRITICAL reach; 26 indexed changed files, 72 indexed changed
  symbols, and 257 affected execution flows
- the generated reach remains within the authorized registry/profile/CLI,
  canonical filesystem, schema resolution, and entry-state fingerprint-symbol
  graph; new private HCM-2.3 lineage symbols remain absent from the entry-state
  index and every UNKNOWN impact was treated as CRITICAL
- `SchemaRegistry::load_admitted_deferred_fingerprints` remains unchanged; the
  subject contains no Cargo/package-definition, archive, preservation,
  HCM-2.4, Phase 3+, or Phase 4 SDK/transport path
- `git diff --check` passes; this graph result authorizes only immutable Review
  6 dispatch and is not a `CLEAN`, commit, or closeout conclusion
