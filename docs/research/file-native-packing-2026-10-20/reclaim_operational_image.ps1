param(
 [Parameter(Mandatory=$true)][string]$Root,
 [Parameter(Mandatory=$true)][string]$CapsuleRoot,
 [Parameter(Mandatory=$true)][string]$ExpectedManifest,
 [Parameter(Mandatory=$true)][string]$ExpectedInventory,
 [Parameter(Mandatory=$true)][string]$ExpectedTransport,
 [Parameter(Mandatory=$true)][string]$ParentProof,
 [Parameter(Mandatory=$true)][string]$Receipt,
 [switch]$Execute
)
# Prepared only. Every execution needs a separate parent admission and proof.
$ErrorActionPreference='Stop'
function Assert-ImageBoundary([string]$Candidate,[string]$Boundary){
 $Full=[IO.Path]::GetFullPath($Candidate)
 if(-not ($Full.Equals($Boundary,[StringComparison]::OrdinalIgnoreCase) -or $Full.StartsWith($Boundary+'\',[StringComparison]::OrdinalIgnoreCase))){throw 'outside-exact-image-root'}
 $Cursor=$Full
 while($Cursor){
  if(Test-Path -LiteralPath $Cursor){$Item=Get-Item -LiteralPath $Cursor -Force;if(($Item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw 'reparse-or-junction'}}
  $Next=[IO.Path]::GetDirectoryName($Cursor);if($Next -eq $Cursor){break};$Cursor=$Next
 }
 return $Full
}
function Get-ImageHash([string]$Path){
 $Hash=[Security.Cryptography.SHA256]::Create();$Stream=[IO.File]::OpenRead($Path)
 try{return [BitConverter]::ToString($Hash.ComputeHash($Stream)).Replace('-','').ToLowerInvariant()}finally{$Stream.Dispose();$Hash.Dispose()}
}
$Owned='C:\file-native-packing-20261020';$Root=[IO.Path]::GetFullPath($Root);$CapsuleRoot=[IO.Path]::GetFullPath($CapsuleRoot);$Receipt=[IO.Path]::GetFullPath($Receipt)
$Fixed=@($Owned+'\history-restored\disposable\store',$Owned+'\packed-restored\disposable\store')
$Pattern='^C:\\file-native-packing-20261020\\operational-matrix\\n8000-[AB]\\(baseline|L-old|L-new|P0|PZ)\\disposable\\store$'
if($Root -notin $Fixed -and $Root -notmatch $Pattern){throw 'root-not-in-explicit-owned-image-allowlist'}
$null=Assert-ImageBoundary $Root $Root;$null=Assert-ImageBoundary $CapsuleRoot $Owned
if($CapsuleRoot.StartsWith($Root+'\',[StringComparison]::OrdinalIgnoreCase)){throw 'archive-inside-target'}
if(-not $Receipt.StartsWith($Owned+'\receipts\',[StringComparison]::OrdinalIgnoreCase) -or (Test-Path -LiteralPath $Receipt)){throw 'fresh-external-receipt-required'}
if(-not (Test-Path -LiteralPath $ParentProof -PathType Leaf)){throw 'parent-independent-proof-required'}
foreach($Hash in @($ExpectedManifest,$ExpectedInventory,$ExpectedTransport)){if($Hash -notmatch '^[0-9a-f]{64}$'){throw 'explicit-hash-binding-required'}}
if((Get-ImageHash (Join-Path $CapsuleRoot 'manifest.json')) -ne $ExpectedManifest -or (Get-ImageHash (Join-Path $CapsuleRoot 'files.jsonl')) -ne $ExpectedInventory -or (Get-ImageHash (Join-Path $CapsuleRoot 'operational.bin.gz')) -ne $ExpectedTransport){throw 'verified-archive-changed'}
$Manifest=Get-Content -LiteralPath (Join-Path $CapsuleRoot 'manifest.json') -Raw | ConvertFrom-Json
if($Manifest.version -ne 1 -or $Manifest.profile -ne 'quiescent-full-operational' -or $Manifest.inventory_sha256 -ne $ExpectedInventory -or $Manifest.transport_sha256 -ne $ExpectedTransport -or -not ([IO.Path]::GetFullPath($Manifest.original)).Equals($Root,[StringComparison]::OrdinalIgnoreCase)){throw 'archive-source-binding'}
$Excluded=@('DISPOSABLE.json','pack-maintenance.lock','pack-readers.lock','preparation.lock','writer.lock')
if(@($Manifest.excluded).Count -ne 5 -or @($Manifest.excluded | Where-Object {$_ -notin $Excluded}).Count -ne 0){throw 'unexpected-archive-exclusions'}
$Rows=@(Get-Content -LiteralPath (Join-Path $CapsuleRoot 'files.jsonl') | ForEach-Object {$_ | ConvertFrom-Json});$Expected=@{};$PlannedBytes=[long]0
if($Rows.Count -ne $Manifest.files -or $Rows.Count -gt 100000){throw 'inventory-count'}
foreach($Row in $Rows){
 if($Row.path.Contains('\') -or $Row.path.Contains(':') -or [IO.Path]::IsPathRooted($Row.path) -or @($Row.path -split '/' | Where-Object {$_ -in @('','..','.')}).Count -gt 0 -or $Row.path -in $Excluded -or $Row.sha256 -notmatch '^[0-9a-f]{64}$' -or [long]$Row.bytes -lt 0){throw 'inventory-path-or-digest'}
 $Path=Assert-ImageBoundary (Join-Path $Root $Row.path.Replace('/','\')) $Root
 if($Expected.ContainsKey($Path)){throw 'duplicate-native-path'};$Expected[$Path]=$Row;$PlannedBytes+=[long]$Row.bytes
}
if($PlannedBytes -ne $Manifest.logical_bytes){throw 'inventory-byte-total'}
$Actual=@{};$Kept=@{};$Pending=New-Object 'System.Collections.Generic.Stack[string]';$Pending.Push($Root)
while($Pending.Count -gt 0){
 $Directory=$Pending.Pop()
 foreach($Item in Get-ChildItem -LiteralPath $Directory -Force){
  if(($Item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw 'reparse-or-junction-in-image'}
  if($Item.PSIsContainer){$Pending.Push($Item.FullName)}elseif($Expected.ContainsKey($Item.FullName)){$Actual[$Item.FullName]=$true}else{
   if($Item.DirectoryName -ne $Root -or $Item.Name -notin $Excluded -or $Item.Length -gt 4096){throw 'unarchived-regular-file'};$Kept[$Item.FullName]=[long]$Item.Length
  }
 }
}
if($Actual.Count -ne $Expected.Count){throw 'source-file-set-changed'}
foreach($Path in $Expected.Keys){$Row=$Expected[$Path];$Item=Get-Item -LiteralPath $Path -Force;if($Item.Length -ne $Row.bytes -or (Get-ImageHash $Path) -ne $Row.sha256){throw 'source-bytes-changed'}}
$RemovedFiles=0;$RemovedBytes=[long]0;$Failure=$null
try{
 if($Execute){foreach($Row in $Rows){
  $Path=Assert-ImageBoundary (Join-Path $Root $Row.path.Replace('/','\')) $Root;$Item=Get-Item -LiteralPath $Path -Force
  if($Item.Length -ne $Row.bytes -or (Get-ImageHash $Path) -ne $Row.sha256){throw 'changed-before-individual-delete'}
  Remove-Item -LiteralPath $Path -Force
  if(Test-Path -LiteralPath $Path -PathType Leaf){throw 'delete-not-observed'};$RemovedFiles++;$RemovedBytes+=[long]$Row.bytes
  [IO.File]::AppendAllText($Receipt+'.jsonl',([PSCustomObject]@{path=$Path;bytes=[long]$Row.bytes;sha256=$Row.sha256;removed_files=$RemovedFiles;removed_bytes=$RemovedBytes} | ConvertTo-Json -Compress)+[Environment]::NewLine)
 }}
}catch{$Failure=$_.Exception.ToString();throw}finally{
 $Result=[PSCustomObject]@{status=if($Failure){'partial-failure-prefix-preserved'}elseif($Execute){'complete-inventoried-regular-files-reclaimed'}else{'verified-plan-no-deletion'};execute=[bool]$Execute;root=$Root;capsule=$CapsuleRoot;inventory_sha256=$ExpectedInventory;transport_sha256=$ExpectedTransport;parent_proof=$ParentProof;parent_proof_sha256=(Get-ImageHash $ParentProof);planned_files=$Rows.Count;planned_bytes=$PlannedBytes;removed_files=$RemovedFiles;removed_bytes=$RemovedBytes;excluded_control_files_retained=$Kept;all_empty_directories_retained=$true;archive_untouched=$true;failure=$Failure}
 [IO.File]::WriteAllText($Receipt,($Result | ConvertTo-Json -Depth 6));$Result | ConvertTo-Json -Depth 6
}
