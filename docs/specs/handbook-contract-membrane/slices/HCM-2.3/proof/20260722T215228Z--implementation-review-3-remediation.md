# HCM-2.3 Implementation Review 3 Remediation

> **Status (2026-07-22): all four Review 3 findings accepted and remediated;
> complete proof wall and a fresh different-agent complete-subject review remain
> pending.** This record does not claim `CLEAN`, a primary commit, closeout, or
> authority for HCM-2.4 or any later phase.

## Review admission and verdict

Fresh isolated Review 3 admitted the immutable subject from
[`20260722T202942Z--HCM-2-3--fresh-complete-implementation-review-3.json`](../../../handoffs/dispatches/20260722T202942Z--HCM-2-3--fresh-complete-implementation-review-3.json),
subject fingerprint
`sha256:0cc34c001c046f0d8d0d5c9b350285b9ca699c07fccf70a3e80f2dd9483bd36d`.
It returned `CHANGES_REQUIRED`. No finding is waived:

1. a canonical namespace-substitution window remained between scratch
   preparation/publication and final installed verification;
2. same-bytes canonical-basis ABA with restored ordinary metadata was not
   durably distinguishable, including during recovery;
3. the required engine recovery, post-marker prefix/non-prefix, inventory, and
   same-key/different-request concurrency matrices were incomplete; and
4. the required real CLI exit/currentness/restart/tombstone matrix was
   incomplete.

## User authorization and exact impact evidence

The user explicitly authorized the bounded HCM-2.3 schema-root separation and
required it to be treated as CRITICAL regardless of GitNexus's reported risk.
That repair remains confined in production to `schema_registry.rs`: BuiltIn
schema paths are fixed beneath `definitions/schemas` and obtain bytes only from
the unchanged package allowlist; Repository paths are request-root-only; the
HCM-2.3 request contains only `.handbook/definitions/schemas`.
`SchemaRegistry::load_admitted_deferred_fingerprints` remains untouched.

Exact schema impact commands and reported reach:

| Exact symbol UID | Command | Direct / affected / processes / modules | Reported / disposition |
|---|---|---:|---|
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.load_document#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | 2 / 2 / 1 / 2 | LOW / treated CRITICAL |
| `Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1` | `npx gitnexus impact Function:crates/engine/src/schema_registry.rs:ClosureLoader.require_allowed_root#1 --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | 1 / 3 / 1 / 2 | LOW / treated CRITICAL |

The new lineage store and its tests remain untracked relative to the entry-HEAD
GitNexus index. Every exact pre-edit lookup below therefore returned target not
found, `risk: UNKNOWN`, `impactedCount: 0`, and no indexed direct callers,
processes, or modules. Each result was reported and treated as CRITICAL rather
than as evidence of safety:

```text
npx gitnexus impact <target> --repo C:\hcm22ar-doc-repair --direction upstream --include-tests --depth 3
```

Targets: `prepare_replacement_scratch`, `publish_replacement`,
`verify_compare_and_write_path`, `ensure_marker`, `recover_establishing`,
`build_intent`, `validate_intent`, `build_marker`, `validate_marker`,
`install_output`, `recovery_installed_output_set`, `install_descriptor`,
`commit_new_locked`, `recover_pending`, `observe_installed_output`,
`verify_installed_outputs`, `native_path_tokens`,
`substitute_with_same_bytes`,
`canonical_scratch_identity_substitution_after_basis_verification_refuses`,
`canonical_identity_substitution_after_installed_verification_refuses`,
`commit_install_and_committed_rename_faults_recover_to_exact_replay`, and
`same_key_same_request_race_commits_once_and_replays_once`.

No additional high-level CRITICAL symbol was edited. The Review 3 remediation
stays in the private lineage-store verification path, its closed control
schema/vectors, HCM-2.3 tests, CLI tests, and proof/contract text.

## Implemented remediation

### Native canonical publication and ABA

- Promotion intent binds the exact native version of the nullable canonical
  basis. A same-bytes rewrite with restored mtime still changes the native
  version and refuses before publication and during recovery.
- `native-publication.json` binds transaction/intent, basis version, canonical
  ref, and the deterministic scratch's native identity/version;
  promotion `verified.json` binds that observation fingerprint.
- Root/basis checks precede stage reuse or publication. Scratch and installed
  canonical handles remain retained across their in-process decision windows.
- Recovery accepts only exact scratch bytes plus exact native identity/version;
  absent, partial, regular-file-substituted, symlinked, hard-linked, or
  non-regular residue refuses without rewrite or adoption.
- Native identity is Windows file ID + creation time or Unix device + inode +
  filesystem birth time. Birth/creation survives rename but changes on
  delete/recreate, closing immediate inode/file-ID reuse. Native version adds
  Windows USN/metadata or Unix link/length/mtime/ctime metadata. Unsupported
  observations fail closed.
- Installed verification retains every exact output and removes a newly
  written marker if the retained guards change before the marker fault
  boundary.

RED/GREEN evidence: the Unix post-install regular same-byte substitution test
initially recovered successfully because its inode was immediately reused.
After birth time entered native identity, the Windows regular/hard-link rows
and WSL regular/symlink/hard-link rows all refuse. Scratch substitution,
same-bytes restored-mtime basis ABA, incomplete scratch, and normal promotion
remain green.

### Engine matrix

- Commit recovery now has executable cases for exact staging, every install
  ordinal, installed verification, marker, evidence `E`, result `D`, ledger
  `L`, and committed rename `Q` fault boundaries.
- Early established/pending commit intent prefixes perform only the allowed
  byte-preserving intent move and then refuse incomplete staging without
  artifact/result/ledger output.
- Exact post-marker prefixes `{}`, `{E}`, `{E,D}`, `{E,D,L}`, and
  `{E,D,L,Q}` recover to byte-identical replay. Missing-middle `D/L` without
  `E` and `L` without `D` refuse with zero state delta.
- Same-key/different-request concurrency produces exactly one commit, one
  `IdempotencyConflict`, and one evaluation.
- Direct boundary tests admit the exact maximum and reject maximum + 1 for 16
  transaction files, 16 MiB per transaction, 1,024 pending and 4,096 committed
  transactions per family, 64/256 MiB aggregate pending/committed bytes, 1 MiB
  records, 4,096 semantic records, 8,192 closure records, and 256 MiB per
  instance. Exact grammar rejects deeper/unknown directories before the looser
  depth ceiling can be reached.

### Actual CLI matrix

The built `handbook` binary now proves:

- exit 0 success, exit 1 domain/control refusal, and exit 2 Clap grammar
  refusal;
- every operation uses the explicit repository root while cwd is unrelated;
- missing selection refuses, and only exact lowercase `absent` is the absent
  currentness token;
- human and JSON success/refusal surfaces;
- fresh-process recovery for every `{}`/`{E}`/`{E,D}`/`{E,D,L}`/
  `{E,D,L,Q}` post-marker prefix;
- fresh-process zero-delta refusal for missing-middle suffix states; and
- exact request under a tombstone returns expired while a different request
  under the same key conflicts.

## Focused GREEN evidence to date

- frozen control identity chain: PASS 1/1;
- Windows generic lineage integration: PASS 36/36 before the added matrix;
  every newly added recovery/concurrency/substitution test passes focused;
- engine inventory boundary unit tests: PASS 3/3, including the 17,000+ exact
  entry wall;
- actual-binary CLI additions: PASS 3/3 focused (the file now contains 8
  tests total);
- Windows and WSL post-install native substitution: PASS; WSL scratch
  regular/symlink/hard-link substitution: PASS;
- formatting and engine all-target compile checks: PASS.

The complete HCM-2.2 and workspace proof walls must be rerun after these final
changes. GitNexus change detection and a fresh different-agent complete-subject
review are mandatory before any commit.
