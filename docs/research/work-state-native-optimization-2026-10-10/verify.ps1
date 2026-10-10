param([string]$Experiment,[string]$Target,[string]$Evidence)
$ErrorActionPreference='Stop'
$env:RUST_MIN_STACK='33554432'
$env:HCM_RESEARCH_VARIANT='combined'
Push-Location -LiteralPath $Experiment
try {
    & cargo test --locked --offline --release --jobs 2 --target-dir $Target -p handbook-engine --features test-support --lib research_ -- --test-threads=1 *> (Join-Path $Evidence 'tests-research.txt')
    if ($LASTEXITCODE -ne 0) { throw 'research guard tests failed' }
    & cargo test --locked --offline --release --jobs 2 --target-dir $Target -p handbook-engine --features test-support --test repository_durability --test work_state --test work_kind -- --test-threads=1 *> (Join-Path $Evidence 'tests-engine.txt')
    if ($LASTEXITCODE -ne 0) { throw 'engine tests failed' }
    & cargo test --locked --offline --release --jobs 2 --target-dir $Target -p handbook-sdk --features handbook-engine/test-support --test work_state --test work_state_retained --test work_state_integrity --test work_state_durability --test work_kind --test work_kind_integrity --test work_kind_durability -- --test-threads=1 *> (Join-Path $Evidence 'tests-sdk.txt')
    if ($LASTEXITCODE -ne 0) { throw 'sdk tests failed' }
} finally { Pop-Location }
