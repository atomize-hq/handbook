$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$taskRepository = 'C:/Users/spmcc/Documents/__Project_Code/handbook'
$taskBaseline = '8e5c6f21077a508d40cd0661bd1946d04a132281'
$taskRedRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('handbook-work-registration-red-' + [guid]::NewGuid().ToString('N'))
$taskRedSource = Join-Path $taskRedRoot 'source'
$taskRedTarget = Join-Path $taskRedRoot 'target'
$taskRedArchive = Join-Path $taskRedRoot 'baseline.tar'
New-Item -ItemType Directory -Path $taskRedSource -Force | Out-Null
& git -C $taskRepository archive --format=tar --output=$taskRedArchive $taskBaseline
if ($LASTEXITCODE -ne 0) { throw 'Baseline archive failed' }
& tar -xf $taskRedArchive -C $taskRedSource
if ($LASTEXITCODE -ne 0) { throw 'Baseline extraction failed' }
$taskRedTest = Join-Path $taskRedSource 'crates/engine/tests/work_state.rs'
Copy-Item -LiteralPath (Join-Path $taskRepository 'crates/engine/tests/work_state.rs') -Destination $taskRedTest
$taskRedLog = Join-Path $taskRedRoot 'red-output.log'
$env:RUST_MIN_STACK = '67108864'
$env:CARGO_BUILD_JOBS = '2'
Push-Location $taskRedSource
try {
    & cargo test --locked --release --target-dir $taskRedTarget -p handbook-engine --features test-support --test work_state retained_explicit_binding_uses_supplied_exact_basis_and_original_bytes_without_native_reads -- --exact --test-threads=1 *> $taskRedLog
    $taskRedExit = $LASTEXITCODE
} finally {
    Pop-Location
}
$taskOutput = Get-Content -LiteralPath $taskRedLog -Raw
$taskExpectedRed = ($taskRedExit -ne 0) -and ($taskOutput -match 'test retained_explicit_binding_uses_supplied_exact_basis_and_original_bytes_without_native_reads \.\.\. FAILED')
[pscustomobject]@{
    baseline = $taskBaseline
    source = $taskRedSource
    target = $taskRedTarget
    test_source_sha256 = (Get-FileHash -LiteralPath $taskRedTest -Algorithm SHA256).Hash.ToLowerInvariant()
    product_transaction_sha256 = (Get-FileHash -LiteralPath (Join-Path $taskRedSource 'crates/engine/src/work_state/transaction.rs') -Algorithm SHA256).Hash.ToLowerInvariant()
    exit_code = $taskRedExit
    expected_red = $taskExpectedRed
    log = $taskRedLog
} | ConvertTo-Json -Compress
Get-Content -LiteralPath $taskRedLog -Tail 28
if (-not $taskExpectedRed) { throw 'Expected baseline regression failure was not observed' }
