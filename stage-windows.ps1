$ErrorActionPreference = 'Stop'
$root = 'C:\fnbench'
if (-not (Test-Path "$root\src")) { throw 'owned staged source missing' }
$public = "$root\public"
$phase = 'stage'
function Write-Setup($result, $phase, $kind, $reason) {
    @{result=$result; phase=$phase; error_type=$kind; reason=$reason} | ConvertTo-Json -Compress | Set-Content "$public\setup.json"
}
Write-Setup 'in_progress' $phase ''
try {
    $python = (Get-Command python -ErrorAction Stop).Source
    $rustup = (Get-Command rustup -ErrorAction Stop).Source
    if (([IO.Path]::GetPathRoot($python) -ne 'C:\') -or ([IO.Path]::GetPathRoot($rustup) -ne 'C:\')) { throw 'python_or_rustup_non_c' }
    $env:TEMP = "$root\tmp"; $env:TMP = "$root\tmp"; $env:TMPDIR = "$root\tmp"
    $env:CARGO_HOME = "$root\cargo"; $env:RUSTUP_HOME = "$root\rustup"
    # Scope process search to C:. No machine security configuration is changed.
    $env:PATH = (($env:PATH -split ';') | Where-Object { $_ -and [IO.Path]::IsPathRooted($_) -and [IO.Path]::GetPathRoot($_) -eq 'C:\' -and (Test-Path $_) }) -join ';'
    Set-Location "$root\src"
    & $python -c "import driver; driver.verify_source()"
    if ($LASTEXITCODE -ne 0) { throw 'source_inventory_mismatch' }
    $phase = 'toolchain_install'
    & $rustup toolchain install 1.89.0 --profile minimal
    if ($LASTEXITCODE -ne 0) { throw 'toolchain_install_failed' }
    $rustc = (& $rustup which --toolchain 1.89.0 rustc).Trim()
    $cargo = (& $rustup which --toolchain 1.89.0 cargo).Trim()
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path $vswhere)) { throw 'vswhere_unavailable' }
    $vs = (& $vswhere -latest -products '*' -property installationPath).Trim()
    $kits = (Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots').KitsRoot10
    New-Item -ItemType Directory -Force -Path $env:CARGO_HOME, $env:RUSTUP_HOME | Out-Null
    $env:FN_TOOL_PATHS = @{python=$python; rustup=$rustup; rustc=$rustc; cargo=$cargo; vswhere=$vswhere; visual_studio=$vs; windows_sdk=$kits; root=$root; temp=$env:TEMP; tmp=$env:TMP; cargo_home=$env:CARGO_HOME; rustup_home=$env:RUSTUP_HOME; system_root=$env:SystemRoot} | ConvertTo-Json -Compress
    & $python check-windows-tools.py
    if ($LASTEXITCODE -ne 0) { throw 'tool_dependency_unresolved_or_non_c' }
    $phase = 'environment'
    & $python host.py
    if ($LASTEXITCODE -ne 0) { throw 'host_capture_failed' }
    & $rustup run 1.89.0 rustc -vV | Set-Content "$public\rust-version.txt"
    if ($LASTEXITCODE -ne 0) { throw 'rust_version_failed' }
    $phase = 'build'
    & $rustup run 1.89.0 cargo fetch --locked --manifest-path source/Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw 'cargo_fetch_failed' }
    & $rustup run 1.89.0 cargo build --release --locked --offline --manifest-path source/Cargo.toml --target-dir "$root\src\target" -p handbook-sdk --example s06_work_compare
    if ($LASTEXITCODE -ne 0) { throw 'cargo_build_failed' }
    & $python -c "import driver; driver.verify_source()"
    if ($LASTEXITCODE -ne 0) { throw 'post_build_source_mismatch' }
    foreach ($name in @('TEMP','TMP','TMPDIR','CARGO_HOME','RUSTUP_HOME','PATH')) { "$name=$([Environment]::GetEnvironmentVariable($name))" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append }
    "FN_PYTHON=$python" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
    Write-Setup 'complete_before_launch' $phase ''
} catch {
    $allowed = @('python_or_rustup_non_c','workspace_copy_failed','source_inventory_mismatch','toolchain_install_failed','vswhere_unavailable','tool_dependency_unresolved_or_non_c','host_capture_failed','rust_version_failed','cargo_fetch_failed','cargo_build_failed','post_build_source_mismatch')
    $reason = if ($_.Exception.Message -in $allowed) { $_.Exception.Message } else { 'other' }
    Write-Setup 'failed_before_launch' $phase $_.Exception.GetType().Name $reason
    throw
}
