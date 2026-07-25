# HCM-2.3 implementation Review 4 remediation

## Gate and authority

Fresh complete-subject Review 4 admitted immutable dispatch
`20260722T223824Z--HCM-2-3--fresh-complete-implementation-review-4` with 55
entries and aggregate subject fingerprint
`sha256:07ed199cc69106e2e1829469c75e3a2554ca2b588053fadbab118bf8fe107965`.
Its verdict was `CHANGES_REQUIRED`. No finding was waived. The user separately
authorized the bounded source-kind schema-root separation, required every
private helper touched there to be treated as CRITICAL, and prohibited changes
to `SchemaRegistry::load_admitted_deferred_fingerprints`; that method remains
unchanged. The exact schema impact evidence and commands remain recorded in the
main proof wall, Increment 12.

Review 4 required four repairs:

1. bind native identity/version to retained handles and close the deterministic
   substitution windows around basis observation and scratch publication;
2. make fresh scratch strictly absent/create-new and recovery scratch strictly
   observe-only against persisted publication authority;
3. remove public caller authority over intake `consumer.version`; and
4. enforce one shared engine 1 MiB byte-document limit before parsing for
   evaluate, intake append, candidate append, and promotion.

## Exact pre-edit impact evidence

Every result below was treated as CRITICAL under the user's explicit authority,
including GitNexus LOW/UNKNOWN results. Commands used `--direction upstream`
and the stated depth against `C:\hcm22ar-doc-repair`.

| Exact target / UID | Exact command shape | Result before edit |
|---|---|---|
| `Impl:crates/engine/src/canonical_repo_support.rs:TrustedRepoFile` | `npx gitnexus impact Impl:crates/engine/src/canonical_repo_support.rs:TrustedRepoFile --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` | LOW; 0 direct callers, 0 affected symbols, 0 processes, 0 modules |
| `observe_retained_regular_file`, `native_path_tokens`, `prepare_replacement_scratch`, `publish_replacement`, `require_prepared_replacement_unchanged` | `npx gitnexus impact <target> --direction upstream --depth 4 -r C:\hcm22ar-doc-repair` for each target | entry-state index returned target-not-found/UNKNOWN; 0 reported direct callers, affected symbols, processes, or modules |
| `ArtifactMutationServiceV1.intake_append`, `ArtifactMutationServiceV1.candidate_append`, `ArtifactMutationServiceV1.promote` | same exact command shape at depth 4 for each target | target-not-found/UNKNOWN; 0 reported direct callers, affected symbols, processes, or modules |
| `parse_intake_append`, `parse_candidate_append`, `parse_promotion`, `validate_intake_bindings` | same exact command shape at depth 4 for each target | target-not-found/UNKNOWN; 0 reported direct callers, affected symbols, processes, or modules |
| `ArtifactRepositoryV1.evaluate_intake_document`, `CoverageInputDocumentV1.from_bytes` | same exact command shape at depth 4 for each target | target-not-found/UNKNOWN; 0 reported direct callers, affected symbols, processes, or modules |

The UNKNOWN results arise because these HCM-2.3 symbols are new relative to the
entry-state GitNexus index. They were not interpreted as low risk. No
high-level schema resolver was edited for this remediation, and no new impact
analysis made such an edit unavoidable.

## Mandatory RED evidence

- equal fresh scratch residue was adopted and unequal residue was rewritten;
- same-byte replacement scratch substitution at the final pre-rename hook
  changed canonical bytes before refusal;
- direct engine mutation parsers admitted 1 MiB + 1 byte, and public intake
  persisted an attacker-supplied consumer version instead of repository
  `VERSION`;
- the Unix retained-read/path-identity hook demonstrated the pathname-only
  identity gap; on Windows the same attempted canonical substitution was
  denied by the retained strict handle's share mode;
- the first zero-delta evaluation assertion also exposed that the public
  evaluate entry reached recovery-lock creation before input admission.

## Remediation

- Fresh promotion now checks deterministic scratch absence before intent
  establishment and uses exclusive create-new. It binds the identity created
  by that write, then requires the final read-only retained guard to reopen the
  same post-close identity. Equal/unequal residue refuses with exact zero
  domain delta and is never adopted, rewritten, removed, or cleaned up.
- Recovery only opens the existing scratch read-only and compares retained
  bytes plus bound native identity/version with `native-publication.json`.
  Invalid or partial residue is preserved unchanged.
- Native tokens bind retained-handle metadata to no-follow pathname identity
  and version. Unix uses device/inode/birth time plus link/length/change times;
  Windows retains file ID/creation and USN/length/time/attributes in the bound
  path observation. The canonical Windows basis handle denies delete/write
  substitution while retained. Scratch is reobserved from its retained handle
  both before and after the deterministic pre-rename hook, and that same handle
  remains retained through publication and commit-marker verification.
- The fresh-write close transition is explicit: Windows may advance USN when
  the write handle closes, so persisted version authority is sampled through
  the same-identity post-write read guard, never from the unstable pre-close
  value.
- `ArtifactMutationServiceV1::intake_append` no longer accepts a consumer
  version. Intake construction and persisted validation require the engine's
  compile-time repository `VERSION`; no request field or request-subject widen
  was introduced.
- `MAX_ARTIFACT_INPUT_DOCUMENT_BYTES` is the shared engine/CLI 1 MiB bound.
  Exact-limit documents reach duplicate-safe parsing; +1 refuses before
  parsing. Public intake evaluation checks the bound before recovery-lock
  mutation, and every mutation parser checks before repository authority or
  journal work.

## GREEN evidence

- focused Windows fresh residue, pre-publication scratch substitution,
  retained-basis sharing denial, recovery scratch, all installed complements,
  engine-owned version, and +1 zero-delta tests: PASS;
- focused WSL Unix retained-read identity substitution and final scratch
  substitution: PASS;
- registration kernel: 16/16; generic lineage: 45/45 in 315.27 seconds;
- complete engine package: PASS in 551.4 seconds, including 152 unit tests,
  lineage 45/45, registration 16/16, and schema registry 19/19;
- complete CLI package: PASS in 200.4 seconds, including 98 unit tests and all
  8 actual-binary HCM-2.3 cases;
- complete workspace: PASS in 784.2 seconds;
- full HCM-2.2 focused regression: 16 engine targets / 89 tests and 2 CLI
  targets / 10 tests PASS;
- Windows engine and CLI all-target Clippy with `-D warnings`, engine all-target
  check, format check, diff check, and WSL engine all-target Clippy: PASS;
- `cargo tree -p handbook-engine -e features`: PASS with no Cargo or lock
  change; engine package remains 191 files / 3.2 MiB / 537.0 KiB compressed;
  CLI package reproduces only the unchanged missing-version baseline for the
  path dependency `handbook-compiler`.
- archive normal and negative self-test PASS; handoff normal validation PASS
  with 3 record schemas, 2 internal-dispatch schemas, 2 templates, 52 records,
  271 current internal dispatches, 8 admitted legacy dispatches, and 52 ledger
  entries; both handoff self-tests PASS;
- duplicate-safe parsing PASS for all six HCM-2.3 JSON artifacts and Draft
  2020-12 meta-validation PASS for all three schemas; Markdown validation PASS
  for 23 affected/slice files, 48 local links, and balanced fences;
- preservation hashes remain exact for `profile_builtins.rs`
  (`e53f709651945bc1ed6064dc7362ef0196bf04f48d0fc21474017ccf0a901eb1`),
  HCM-1.1 schema entry
  (`ede69fb62b0b5a0b023bee65f0adf1c980155d535e47b37108a4097eeaade536`),
  and HCM-1.1 root schema
  (`f52ffaec29bc1edbd51addc1391dd844ab5e58d07e85d69970fcaeeac0159510`).

GitNexus change detection and a fresh immutable complete-subject Review 5
remain mandatory. This remediation record does not claim `CLEAN` or commit
authority.
