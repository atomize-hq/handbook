param(
  [string]$Python='C:/Users/spmcc/.pyenv/pyenv-win/versions/3.11.9/python.exe',
  [string]$Executable='C:/file-native-navigation-20261019/bin/candidate.exe',
  [string]$Owned='C:/file-native-navigation-20261019',
  [string]$Worktree='C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook'
)
$ErrorActionPreference='Stop'
$driver=Join-Path $PSScriptRoot 'measure.py'
$census=Join-Path $PSScriptRoot 'census.py'
$deadline=[DateTimeOffset]::Parse('2026-10-20T00:20:00Z').ToUnixTimeSeconds()
$matrix=Join-Path $Owned 'matrix-final'
New-Item -ItemType Directory -Path $matrix -Force|Out-Null
$outcomes=@()
foreach($n in @(128,1000,10000)){
  foreach($profile in @(4,3,2)){
    foreach($mode in @('A','B')){
      $label="n$n-$mode-p$profile"
      $base=Join-Path $matrix $label
      if([DateTimeOffset]::UtcNow.ToUnixTimeSeconds() -ge $deadline){throw 'original task closeout deadline: remaining cells censored'}
      if((Get-PSDrive C).Free -lt 140GB){throw 'Windows free reserve refusal'}
      $before=& $Python $census $Owned $Worktree|ConvertFrom-Json
      $sources=[long]($n/10)*126976+[long]($n%10)*65536
      $forecast=$sources*$(if($mode -eq 'B'){3}else{2})+64MB
      if($before.logical_bytes+$forecast -gt 768MB -or $before.files+4*$n+10000 -gt 100000){throw "capacity admission refusal for $label"}
      $before|ConvertTo-Json -Depth 6|Set-Content -LiteralPath (Join-Path $matrix "$label-admission.json") -Encoding utf8
      & $Python $driver --exe $Executable --base $base --n $n --mode $mode --profile $profile --deadline $deadline 2>&1|Tee-Object -FilePath (Join-Path $matrix "$label-run.txt")
      if($LASTEXITCODE){throw "measurement failure/censor: $label; fixture retained"}
      $summary=Get-Content -LiteralPath (Join-Path $base 'summary.json') -Raw|ConvertFrom-Json
      if($summary.status -ne 'success'){throw 'missing completion proof'}
      # Delete only the successfully verified disposable fixture, preserving raw
      # events, measurements, failures and admission/cleanup receipts in its parent.
      $target=(Resolve-Path -LiteralPath (Join-Path $base 'cell/disposable/store')).Path
      $expected=[IO.Path]::GetFullPath((Join-Path $base 'cell/disposable/store'))
      $allowed=[IO.Path]::GetFullPath($matrix)+[IO.Path]::DirectorySeparatorChar
      if($target -ne $expected -or -not $target.StartsWith($allowed,[StringComparison]::OrdinalIgnoreCase)){throw 'cleanup boundary mismatch'}
      Remove-Item -LiteralPath $target -Recurse -Force
      $after=& $Python $census $Owned $Worktree|ConvertFrom-Json
      if($after.logical_bytes -gt 768MB -or $after.files -gt 100000){throw 'owned closeout ceiling exceeded'}
      $outcomes+=@{cell=$label;status='success';fixture_removed=(-not(Test-Path -LiteralPath $target));source= $summary;owned_after=$after}
      $outcomes|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $matrix 'cells.json') -Encoding utf8
    }
  }
}
