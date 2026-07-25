# HCM-2.3 implementation Review 5 remediation

## Gate and authority

Fresh complete-subject Review 5 admitted immutable dispatch
`20260723T001130Z--HCM-2-3--fresh-complete-implementation-review-5` with 57
entries and aggregate subject fingerprint
`sha256:b07eef50d4b30966ce90459280f4a274656b7ef7c92c327adc493b0d79d317e1`.
Its verdict was `CHANGES_REQUIRED`. No finding was waived.

The user authorized only the bounded HCM-2.3 implementation and the additional
schema-root source-kind separation. Every lineage helper in this repair was
treated as CRITICAL even when the entry-state GitNexus index returned UNKNOWN.
The production delta below remains confined to
`crates/engine/src/artifact_lineage_store.rs`; it does not touch
`SchemaRegistry::load_admitted_deferred_fingerprints`, widen schema authority,
or enter HCM-2.4, Phase 3+, or Phase 4 SDK/transport work.

Review 5 required three repairs:

1. revalidate the canonical basis after the deterministic final publication
   hook, immediately before scratch publication, on fresh and recovery paths;
2. prove every ledger, result, semantic record, and subordinate closure is
   reachable from exactly one valid journal chain before replay/expiry and
   before recovery mutation; and
3. forbid fresh `CreateNew` from adopting equal preexisting closure or semantic
   bytes while retaining exact installed-subset recovery for an already
   established intent.

## Exact impact evidence

Commands used `--direction upstream --depth 4` against
`C:\hcm22ar-doc-repair`. The entry-state index returned target-not-found for
the new private HCM-2.3 lineage symbols; each result reported UNKNOWN risk,
zero direct callers, zero affected symbols, zero processes, and zero modules.
Those results were treated as CRITICAL, not as low risk.

| Exact target request | Exact command |
|---|---|
| `GenericArtifactLineageStoreV1.lookup_existing` | `npx gitnexus impact "GenericArtifactLineageStoreV1.lookup_existing" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.verify_committed` | `npx gitnexus impact "GenericArtifactLineageStoreV1.verify_committed" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.validate_inventory` | `npx gitnexus impact "GenericArtifactLineageStoreV1.validate_inventory" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.install_output` | `npx gitnexus impact "GenericArtifactLineageStoreV1.install_output" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `require_equal_file` | `npx gitnexus impact "require_equal_file" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `publish_replacement` | `npx gitnexus impact "publish_replacement" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.execute_inner` | `npx gitnexus impact "GenericArtifactLineageStoreV1.execute_inner" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.recover_locked` | `npx gitnexus impact "GenericArtifactLineageStoreV1.recover_locked" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.install_descriptor` | `npx gitnexus impact "GenericArtifactLineageStoreV1.install_descriptor" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `replace_durable` | `npx gitnexus impact "replace_durable" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `validate_result_record` | `npx gitnexus impact "validate_result_record" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `require_fresh_create_new_outputs_absent` | `npx gitnexus impact "require_fresh_create_new_outputs_absent" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.validate_committed_reachability` | `npx gitnexus impact "GenericArtifactLineageStoreV1.validate_committed_reachability" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |
| `GenericArtifactLineageStoreV1.require_exact_committed_chain` | `npx gitnexus impact "GenericArtifactLineageStoreV1.require_exact_committed_chain" --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` |

No exact impact analysis made a high-level schema resolver edit unavoidable,
so the user's mandatory stop condition for such an edit did not trigger.

## RED evidence

- At the final deterministic publication hook, a concurrent writer could
  change, delete, or same-byte/restored-mtime replace the canonical basis after
  the earlier compare-and-write check. Publication then overwrote that newer
  state.
- A syntactically valid, self-fingerprinted orphan ledger could reach
  `lookup_existing` and manufacture retained-result replay or tombstone expiry;
  orphan results and valid-name artifact records had no reverse citation proof.
- Fresh `CreateNew` accepted a preexisting equal immutable destination through
  `require_equal_file`, allowing uncited closure or semantic bytes to be
  adopted by a newly established transaction.

## Remediation

- `publish_replacement` now receives the exact expected canonical fingerprint
  and native version. After the final deterministic hook it re-observes the
  canonical basis, then rechecks the retained scratch, releases the required
  Windows basis handle, and publishes. Fresh and recovery callers use the same
  last-check boundary. Canonical mutation, deletion, and same-byte/restored-
  mtime ABA all refuse before marker, result, ledger, or promotion authority.
- `recover_locked` now begins with a reverse-reachability pass after structural
  inventory admission and before establishing or pending recovery. It parses
  every ledger and result, validates exact pending suffix records, verifies
  committed chains, rejects duplicate citations, and permits installed
  immutable records only when one pending or committed journal cites them.
  Exact empty pending shells remain structurally admissible for the established
  intent move and are still rejected later unless that move supplies the exact
  intent.
- Recovery ends with a committed-only equality pass: on-disk ledgers, results,
  semantic records, and closures must equal the exact one-chain projection.
  `lookup_existing` independently requires the addressed ledger to resolve to
  and verify one exact committed journal before conflict, replay, or expiry.
- Fresh planning checks every `CreateNew` final destination with no-follow
  metadata before intent establishment, and installation uses exclusive
  create-new only. Equal and unequal closure/semantic residue refuses without
  an intent, result, ledger, or authority delta. Recovery retains its separate
  exact descriptor/bytes/semantic validation and may resume only an installed
  subset cited by an already-established intent.

## GREEN evidence

- all six fresh/recovery final-hook combinations (changed, deleted, and
  same-byte/restored-mtime ABA): PASS;
- equal and unequal fresh closure and semantic residue before intent: PASS;
- manufactured retained-result replay, tombstone expiry, result-only,
  ledger-only, malformed self-fingerprinted ledger, ambiguous second-ledger,
  and valid-name artifact orphan cases: PASS with zero post-setup delta;
- every commit staging/install/suffix fault, every installed subset, and every
  established refusal prefix recovery matrix: PASS;
- complete HCM-2.3 focused wall: lineage 49/49, registration 16/16, selection
  1/1, and direct schema registry 19/19;
- exact HCM-2.2 regression wall: 16 engine targets / 89 tests and two CLI
  targets / 10 tests;
- complete engine package: PASS in 518.4 seconds, including unit 152/152 and
  lineage 49/49; complete CLI package: PASS in 228.2 seconds, including all
  eight actual-binary HCM-2.3 cases; complete workspace: PASS in 722.7 seconds;
- Windows engine/CLI all-target Clippy with `-D warnings`, engine all-target
  check, format check, diff check, and WSL engine all-target Clippy: PASS;
- WSL retained-basis substitution, retained-scratch substitution, all final-
  hook canonical mutations, and manufactured-control reachability: PASS;
- dependency tree PASS with no manifest, lock, dependency, or feature change;
  engine package remains 191 files / 3.2 MiB / 540.9 KiB compressed; CLI
  packaging reproduces only the unchanged missing-version baseline for
  `handbook-compiler`;
- archive normal/self-test PASS; handoff normal validation PASS with 272
  current dispatches; both handoff self-tests PASS;
- six JSON artifacts are duplicate-safe; all three schemas pass Draft 2020-12
  meta-validation; 24 affected/slice Markdown files, 48 local links, and all
  fences pass;
- preservation hashes remain exact for `profile_builtins.rs`
  (`e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`),
  the HCM-1.1 schema entry
  (`ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`),
  and HCM-1.1 root schema
  (`f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`).

GitNexus change detection and a fresh immutable complete-subject Review 6
remain mandatory. This remediation record does not claim `CLEAN` or commit
authority.
