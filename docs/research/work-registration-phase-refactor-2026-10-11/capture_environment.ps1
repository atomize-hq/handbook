$phaseEnvironment = [ordered]@{
    captured_at_utc = [DateTime]::UtcNow.ToString('o')
    os = Get-CimInstance Win32_OperatingSystem | Select-Object Caption,Version,BuildNumber,TotalVisibleMemorySize
    cpu = Get-CimInstance Win32_Processor | Select-Object Name,NumberOfCores,NumberOfLogicalProcessors
    volume = Get-Volume -DriveLetter C | Select-Object FileSystem,Size,SizeRemaining
    rustc = (& rustc -Vv | Out-String).Trim()
    cargo = (& cargo -V | Out-String).Trim()
    python = (& python --version | Out-String).Trim()
    power_scheme = (& powercfg /getactivescheme | Out-String).Trim()
    allowlisted_build_environment = [ordered]@{
        RUSTFLAGS = $env:RUSTFLAGS
        CARGO_ENCODED_RUSTFLAGS = $env:CARGO_ENCODED_RUSTFLAGS
        RUSTUP_TOOLCHAIN = $env:RUSTUP_TOOLCHAIN
        CARGO_BUILD_TARGET = $env:CARGO_BUILD_TARGET
        CARGO_PROFILE_RELEASE_OPT_LEVEL = $env:CARGO_PROFILE_RELEASE_OPT_LEVEL
    }
    build = 'locked/offline release defaults, two jobs, MSVC; same standalone driver and dependency lock'
    stacks = '32 MiB main worker and controlled-race child'
    cache = 'warm Windows native stores after untimed public-owner population; no eviction'
    timed_workloads = 'one process and one workflow at a time; all population/builds/tests paused or complete'
}
$phaseEnvironment | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath "$PSScriptRoot/environment.json" -Encoding utf8
