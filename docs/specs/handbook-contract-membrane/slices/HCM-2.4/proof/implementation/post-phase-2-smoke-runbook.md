# Post-Phase-2 product smoke runbook

This HCM-2.4 runbook exercises current supported paths from disposable
directories. It does not replace the immutable P7/Phase 2 exit record and does
not authorize HCM-3.x. Run it in PowerShell from the repository root.

## Disposable binary and roots

```powershell
$SourceRoot = (git rev-parse --show-toplevel).Trim()
$Scratch = Join-Path ([IO.Path]::GetTempPath()) ("handbook-hcm24-" + [guid]::NewGuid().ToString("N"))
$env:CARGO_TARGET_DIR = Join-Path $Scratch "target"
New-Item -ItemType Directory -Path $Scratch | Out-Null
cargo build -p handbook-cli
$Handbook = Join-Path $env:CARGO_TARGET_DIR "debug/handbook.exe"
```

All later mutations remain below `$Scratch`; only tracked fixtures and
contracts are read from `$SourceRoot`.

## Setup, doctor, and incomplete packet

```powershell
$EmptyRepo = Join-Path $Scratch "empty-repo"
New-Item -ItemType Directory -Path $EmptyRepo | Out-Null
git -C $EmptyRepo init -q
Push-Location $EmptyRepo
& $Handbook setup
& $Handbook doctor --json
& $Handbook inspect
& $Handbook generate
Pop-Location
```

Setup exits `1` with `ACTION_REQUIRED`, creates repository identity/runtime
state, and does not create Charter, Project Context, Environment Context, or
`.handbook/profile-selection.json`. Doctor and inspect report both missing
required artifacts with author actions. Generate reports the primary missing
Charter refusal. Inspect and generate must never return `SystemRootMissing` or
direct the operator back to setup.

An explicitly selected non-Git directory retains the same non-authoring
`ACTION_REQUIRED` behavior:

```powershell
$NonGit = Join-Path $Scratch "selected-non-git"
New-Item -ItemType Directory -Path $NonGit | Out-Null
Push-Location $NonGit
& $Handbook setup
Pop-Location
```

## Project Context platform boundary

```powershell
$ProjectContextInput = Join-Path $SourceRoot "tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml"
Push-Location $EmptyRepo
& $Handbook author project-context --from-inputs $ProjectContextInput --validate
& $Handbook author project-context --from-inputs $ProjectContextInput
Pop-Location
```

Validation succeeds without mutation. On Windows, mutation refuses with
`UnsupportedPlatformStrictMutation` and points to a supported Unix host. This
is the accepted platform boundary.

## Charter intake and authenticator boundary

The exact current closed-envelope intake proof is disposable and generates its
input from the released canonical Charter contract:

```powershell
cargo test -p handbook-cli --test hcm_2_2_product_cutover author_intent_is_evaluated_but_fails_closed_without_lineage_persistence -- --exact
Push-Location $EmptyRepo
& $Handbook approvers bootstrap --initial-charter-quorum "Project owner approval=Project owner" --json
Pop-Location
```

The proof passes. Bootstrap reaches the engine and refuses with
`AUTHENTICATOR_UNAVAILABLE`, `retryable: true`, and no current-operation
content delta. The smoke does not bypass native authority.

## Generic artifact listing

```powershell
$GenericRepo = Join-Path $Scratch "generic"
New-Item -ItemType Directory -Path $GenericRepo | Out-Null
Copy-Item -LiteralPath (Join-Path $SourceRoot "crates/engine/tests/fixtures/hcm_2_3_generic_custom_kind/.handbook") -Destination $GenericRepo -Recurse
git -C $GenericRepo init -q
Push-Location $GenericRepo
& $Handbook setup
& $Handbook artifact list-kinds --repository-root $GenericRepo --json
& $Handbook artifact list-instances --repository-root $GenericRepo --json
Pop-Location
```

Both list commands emit JSON after explicit profile selection and setup-created
identity. In `$EmptyRepo`, the same `--json` commands still refuse missing
selection with empty stdout and plain stderr. HCM-2.3 defines no exact
pre-establishment JSON error envelope; that presentation gap is deferred to an
authority that can define versioned protocol semantics.

## Prepared packet

The committed Charter authority is deliberately not synthesized by setup.
These exact integration proofs prepare authority-complete disposable copies of
`tests/fixtures/planning_ready_repo` through
`crates/engine/tests/support/hcm_2_2_committed_charter.rs`:

```powershell
cargo test -p handbook-cli --test cli_surface inspect_reports_ready_when_required_artifacts_present -- --exact
cargo test -p handbook-cli --test cli_surface generate_emits_real_packet_body_when_ready -- --exact
```

Both pass and complement the incomplete installed-binary path above.

## Supported pipeline route

```powershell
$PipelineRepo = Join-Path $Scratch "pipeline"
Copy-Item -LiteralPath (Join-Path $SourceRoot "tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo") -Destination $PipelineRepo -Recurse
git -C $PipelineRepo init -q
Push-Location $PipelineRepo
& $Handbook pipeline list
& $Handbook pipeline show --id pipeline.foundation_inputs
& $Handbook pipeline resolve --id pipeline.foundation_inputs
Pop-Location
```

The route resolves through the next stage requiring
`charter_gaps_detected`/`needs_project_context`. Compile and capture additionally
require the persisted route basis, selected stage, declared inputs, and
completed external model output described by their command help; this smoke
does not manufacture those prerequisites.
