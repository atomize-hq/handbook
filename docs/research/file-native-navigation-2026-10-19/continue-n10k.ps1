param([string]$Python='C:/Users/spmcc/.pyenv/pyenv-win/versions/3.11.9/python.exe')
$ErrorActionPreference='Stop'
$Owned='C:/file-native-navigation-20261019'
$Worktree='C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook'
$matrix=Join-Path $Owned 'matrix-final'
$deadline=[DateTimeOffset]::Parse('2026-10-20T00:20:00Z').ToUnixTimeSeconds()
$driver=Join-Path $PSScriptRoot 'measure.py'
$census=Join-Path $PSScriptRoot 'census.py'
foreach($profile in @(4,3,2)){
  foreach($mode in @('A','B')){
    $label="n10000-$mode-p$profile";$base=Join-Path $matrix $label
    if(Test-Path -LiteralPath (Join-Path $base 'summary.json')){continue}
    if([DateTimeOffset]::UtcNow.ToUnixTimeSeconds() -ge $deadline){throw 'original closeout deadline: remaining cells censored'}
    if((Get-PSDrive C).Free -lt 140GB){throw 'free reserve refusal'}
    $before=& $Python $census $Owned $Worktree|ConvertFrom-Json
    $more=@()
    if($label -eq 'n10000-A-p4'){
      $admission=Get-Item -LiteralPath (Join-Path $matrix "$label-admission.json")
      $epoch=[DateTimeOffset]::new($admission.CreationTimeUtc).ToUnixTimeSeconds()
      $more=@('--reuse-epoch',"$epoch")
      $forecast=126976000+64MB
    }else{
      $forecast=126976000*$(if($mode -eq 'B'){3}else{2})+64MB
      if($mode -eq 'B'){
        $more=@('--seed-archive',(Join-Path $Owned 'n10k-source-corpus.tar.gz'))
        # Refine the earlier rounded 64 MiB metadata guess from the now-complete
        # A cell: two complete metadata sets, 1 MiB cardinality allowance each,
        # plus 12 MiB for two B manifests, batch COW tails, controls and receipts.
        # Supported fixture operations are the fixed <=130-op measured sequence.
        $a=Get-Content -LiteralPath (Join-Path $matrix 'n10000-A-p4/measurements.json') -Raw|ConvertFrom-Json
        $sourceBytes=126972016L
        $metadata=[long]$a[-1].post_collection_census.logical_bytes-2*$sourceBytes
        $metadataBound=2*($metadata+1MB)+12MB
        $forecast=3*126976000L+$metadataBound
        @{method='fixed-workload geometry bound, prospectively applied; no cap change';measured_A_nonpayload_bytes=$metadata;cardinality_allowance_per_metadata_copy=1MB;metadata_copies=2;B_manifest_COW_control_receipt_allowance=12MB;source_copies=3;source_bytes_per_copy=126976000L;additional_bytes=$forecast;owned_before=$before.logical_bytes;prospective_peak=$before.logical_bytes+$forecast;ceiling=768MB}|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-geometry-admission.json") -Encoding utf8
      }
    }
    if($before.logical_bytes+$forecast -gt 768MB){throw "capacity admission refusal: $label"}
    $before|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-continuation-admission.json") -Encoding utf8
    & $Python $driver --exe (Join-Path $Owned 'bin/candidate.exe') --base $base --n 10000 --mode $mode --profile $profile --deadline $deadline --owned-root $Owned --worktree $Worktree @more 2>&1|Tee-Object -FilePath (Join-Path $matrix "$label-continuation-run.txt")
    if($LASTEXITCODE){throw "measurement failure/censor: $label; fixture retained"}
    $summary=Get-Content -LiteralPath (Join-Path $base 'summary.json') -Raw|ConvertFrom-Json
    if($summary.status -ne 'success'){throw 'missing proof'}
    $target=(Resolve-Path -LiteralPath (Join-Path $base 'cell/disposable/store')).Path
    $expected=[IO.Path]::GetFullPath((Join-Path $base 'cell/disposable/store'))
    if($target -ne $expected -or -not $target.StartsWith([IO.Path]::GetFullPath($matrix)+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'cleanup boundary mismatch'}
    Remove-Item -LiteralPath $target -Recurse -Force
    $after=& $Python $census $Owned $Worktree|ConvertFrom-Json
    if($after.logical_bytes -gt 768MB -or $after.files -gt 100000){throw 'owned ceiling exceeded'}
    @{cell=$label;status='measurement-success';initialization_status=$summary.initialization_status;owned_after=$after;fixture_removed=(-not(Test-Path -LiteralPath $target))}|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $matrix "$label-continuation-cleanup.json") -Encoding utf8
  }
}
