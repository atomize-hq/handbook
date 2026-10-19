$ErrorActionPreference='Stop'
$Python='C:/Users/spmcc/.pyenv/pyenv-win/versions/3.11.9/python.exe'
$Owned='C:/file-native-navigation-20261019'
$Worktree='C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook'
$matrix=Join-Path $Owned 'matrix-final'
$deadline=[DateTimeOffset]::Parse('2026-10-20T00:20:00Z').ToUnixTimeSeconds()
foreach($profile in @(3,2)){
  $label="n10000-A-p$profile";$base=Join-Path $matrix $label
  $before=& $Python (Join-Path $PSScriptRoot 'census.py') $Owned $Worktree|ConvertFrom-Json
  $forecast=126976000+40MB
  if($before.logical_bytes+$forecast -gt 768MB -or [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() -ge $deadline){throw 'deferred-view control admission refusal'}
  @{scope='immutable projection only; ordinary view/external edit unperformed';owned_before=$before;additional_forecast=$forecast;ceiling=768MB}|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-deferred-admission.json") -Encoding utf8
  & $Python (Join-Path $PSScriptRoot 'measure.py') --exe (Join-Path $Owned 'bin/candidate.exe') --base $base --n 10000 --mode A --profile $profile --deadline $deadline --owned-root $Owned --worktree $Worktree --seed-archive (Join-Path $Owned 'n10k-source-corpus.tar.gz') --defer-view 2>&1|Tee-Object -FilePath (Join-Path $matrix "$label-deferred-run.txt")
  if($LASTEXITCODE){throw "control failure/censor: $label; fixture retained"}
  $s=Get-Content -LiteralPath (Join-Path $base 'summary.json') -Raw|ConvertFrom-Json
  if($s.status -ne 'success' -or $s.scope -ne 'immutable-projection-only; readable view deferred'){throw 'missing scoped proof'}
  $target=(Resolve-Path -LiteralPath (Join-Path $base 'cell/disposable/store')).Path
  if($target -ne [IO.Path]::GetFullPath((Join-Path $base 'cell/disposable/store')) -or -not $target.StartsWith([IO.Path]::GetFullPath($matrix)+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'cleanup boundary mismatch'}
  Remove-Item -LiteralPath $target -Recurse -Force
  @{status='scoped-control-complete';scope=$s.scope;unperformed_cases=$s.unperformed_cases;fixture_removed=(-not(Test-Path -LiteralPath $target))}|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-deferred-cleanup.json") -Encoding utf8
}
