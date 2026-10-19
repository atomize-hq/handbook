param([string]$Targets='C:/file-native-navigation-20261019/genesis-final-4bde/shell-targets.json')
$ErrorActionPreference='Stop'
$out='C:/file-native-navigation-20261019/ordinary-proof'
New-Item -ItemType Directory -Path $out -Force|Out-Null
$rows=@()
foreach($item in (Get-Content -LiteralPath $Targets -Raw|ConvertFrom-Json)){
  # B's generation path is selected once in the target receipt. All following
  # discovery/list/search/copy/hash operations use ordinary files at that path.
  $readable=[IO.Path]::GetFullPath($item.readable)
  $files=@(Get-ChildItem -LiteralPath (Join-Path $readable 'docs') -File -Recurse)
  $sourcePath=$null
  foreach($part in Get-ChildItem -LiteralPath (Join-Path $readable 'navigation/groups') -File -Filter '*.yaml'){
    $inside=$false
    foreach($line in Get-Content -LiteralPath $part.FullName -Encoding utf8){
      if($line -match '^  - id: (.+)$'){$inside=(($Matches[1]|ConvertFrom-Json) -eq $item.id)}
      if($inside -and $line -match '^    source: (.+)$'){$sourcePath=$Matches[1]|ConvertFrom-Json}
    }
  }
  if(-not $sourcePath -or $sourcePath -ne $item.source){throw 'ordinary source discovery mismatch'}
  $source=[IO.Path]::GetFullPath((Join-Path $readable $sourcePath))
  if(-not $source.StartsWith($readable+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'source boundary mismatch'}
  $copy=Join-Path $out ($item.mode+'-'+$item.id+'.copy')
  Copy-Item -LiteralPath $source -Destination $copy
  $hash=(Get-FileHash -LiteralPath $copy -Algorithm SHA256).Hash.ToLower()
  if($hash -ne $item.expected_sha256){throw 'exact extraction mismatch'}
  $rows+=@{mode=$item.mode;selected_once=$item.selected;readable=$readable;source_id=$item.id;listed_source_files=$files.Count;discovered_source=$sourcePath;copy=$copy;bytes=(Get-Item -LiteralPath $copy).Length;sha256=$hash;exact_expected_match=$true;tools=@('Get-ChildItem','Get-Content','ConvertFrom-Json JSON scalar','Copy-Item','Get-FileHash');engine_during_extraction=$false}
}
$rows|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $out 'ordinary-proof.json') -Encoding utf8
$rows|Select-Object mode,source_id,listed_source_files,bytes,sha256|Format-Table -Wrap
