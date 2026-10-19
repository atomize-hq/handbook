$ErrorActionPreference='Stop'
$Python='C:/Users/spmcc/.pyenv/pyenv-win/versions/3.11.9/python.exe'
$Owned='C:/file-native-navigation-20261019'
$Worktree='C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook'
$matrix=Join-Path $Owned 'matrix-final'
$deadline=[DateTimeOffset]::Parse('2026-10-20T00:20:00Z').ToUnixTimeSeconds()
foreach($profile in @(3,2)){
  $label="n10000-A-p$profile-full";$base=Join-Path $matrix $label
  if(Test-Path -LiteralPath (Join-Path $base 'summary.json')){
    if(-not(Test-Path -LiteralPath (Join-Path $base 'cell/disposable/store'))){throw 'completed original fixture was removed by prior launcher; do not silently rerun/relabel it'}
    continue
  }
  $before=& $Python (Join-Path $PSScriptRoot 'census.py') $Owned $Worktree|ConvertFrom-Json
  $forecast=2*126976000L+64MB
  if($before.logical_bytes+$forecast -gt 1536MB -or $before.files+32000 -gt 150000 -or (Get-PSDrive C).Free -lt 140GB -or [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() -ge $deadline){throw 'full-control admission refusal'}
  @{scope='complete A comparator from authentic selected operational-prefix restore';owned_before=$before;additional_forecast=$forecast;ceiling=1536MB;file_ceiling=150000;authorization='coordinator prospective 1.5GiB/150000 reallocation within human global40GiB; original deadline/caps unchanged'}|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-admission.json") -Encoding utf8
  & $Python (Join-Path $PSScriptRoot 'measure.py') --exe (Join-Path $Owned 'bin/candidate.exe') --base $base --n 10000 --mode A --profile $profile --deadline $deadline --owned-root $Owned --worktree $Worktree --owned-cap-mib 1536 --owned-file-cap 150000 --restore-archive (Join-Path $Owned 'initialization-censor-capture') 2>&1|Tee-Object -FilePath (Join-Path $matrix "$label-run.txt")
  if($LASTEXITCODE){throw "full-control failure/censor: $label; fixture retained"}
  $s=Get-Content -LiteralPath (Join-Path $base 'summary.json') -Raw|ConvertFrom-Json
  if($s.status -ne 'success' -or $s.scope -ne 'complete ordinary view and immutable projection'){throw 'missing full proof'}
  $target=(Resolve-Path -LiteralPath (Join-Path $base 'cell/disposable/store')).Path
  if($target -ne [IO.Path]::GetFullPath((Join-Path $base 'cell/disposable/store')) -or -not $target.StartsWith([IO.Path]::GetFullPath($matrix)+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'cleanup boundary mismatch'}
  $after=& $Python (Join-Path $PSScriptRoot 'census.py') $Owned $Worktree|ConvertFrom-Json
  if($after.logical_bytes -gt 1536MB -or $after.files -gt 150000){throw 'retained owned ceiling exceeded'}
  @{status='complete-retained';scope=$s.scope;fixture_retained=(Test-Path -LiteralPath $target);owned_after=$after}|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $matrix "$label-retained.json") -Encoding utf8
}
