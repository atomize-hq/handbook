param([switch]$Execute,[Parameter(Mandatory=$true)][string]$Receipt)
$ErrorActionPreference='Stop'
$ArchiveRoot='C:\file-native-packing-20261020\artifact-preservation-xz'
$TargetRoots=@('C:\file-native-packing-20261020\build','C:\Users\spmcc\.codex\worktrees\file-native-packing-20261020\handbook\docs\research\file-native-incremental-engine-experiment-2026-10-18\target')
$ExpectedInventory='63d7df0a00d2ae819d2007674b98ce7a70f7b4376a44af3648f39d6ed6fce360'
function Assert-OwnedPath([string]$CandidatePath,[string]$BoundRoot){
    if(-not [IO.Path]::IsPathRooted($CandidatePath)){throw 'absolute-path-required'}
    $Canonical=[IO.Path]::GetFullPath($CandidatePath)
    if(-not ($Canonical.Equals($BoundRoot,[StringComparison]::OrdinalIgnoreCase) -or $Canonical.StartsWith($BoundRoot+'\',[StringComparison]::OrdinalIgnoreCase))){throw 'outside-exact-owned-target'}
    $Cursor=$Canonical
    while($Cursor){
        if(Test-Path -LiteralPath $Cursor){$Item=Get-Item -LiteralPath $Cursor -Force;if(($Item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw 'reparse-or-junction-path'}}
        $Parent=[IO.Path]::GetDirectoryName($Cursor)
        if($Parent -eq $Cursor){break}
        $Cursor=$Parent
    }
    return $Canonical
}
function Get-ExactHash([string]$FilePath){
    $Digest=[Security.Cryptography.SHA256]::Create();$Stream=[IO.File]::OpenRead($FilePath)
    try{return [BitConverter]::ToString($Digest.ComputeHash($Stream)).Replace('-','').ToLowerInvariant()}finally{$Stream.Dispose();$Digest.Dispose()}
}
$Receipt=[IO.Path]::GetFullPath($Receipt)
if(-not $Receipt.StartsWith('C:\file-native-packing-20261020\receipts\',[StringComparison]::OrdinalIgnoreCase)){throw 'receipt-must-be-outside-targets'}
if(Test-Path -LiteralPath $Receipt){throw 'fresh-receipt-required'}
$InventoryPath=Join-Path $ArchiveRoot 'files.jsonl'
if((Get-ExactHash $InventoryPath) -ne $ExpectedInventory){throw 'inventory-hash'}
$Details=Get-Content -LiteralPath (Join-Path $ArchiveRoot 'manifest.json') -Raw | ConvertFrom-Json
if($Details.status -ne 'complete-round-trip-verified-no-deletion' -or $Details.inventory_sha256 -ne $ExpectedInventory -or $Details.transport_sha256 -ne '849e159dd1c9130e5fa27a5d69947f717ad5256f01efe2ae6851c5fbb0ac7fb8'){throw 'complete-archive-binding'}
if((Get-ExactHash (Join-Path $ArchiveRoot 'artifacts.bin.xz')) -ne $Details.transport_sha256){throw 'archive-transport-hash'}
$Rows=@(Get-Content -LiteralPath $InventoryPath | ForEach-Object {$_ | ConvertFrom-Json})
$Expected=@{};$Keep=@{};$PlannedFiles=0;$PlannedBytes=[long]0
foreach($Row in $Rows){
    $Parts=$Row.path -split '/',2;if($Parts.Length -ne 2 -or $Parts[0] -notin @('0','1') -or ($Parts[1] -split '/') -contains '..' -or $Parts[1].Contains(':') -or $Parts[1].Contains('\')){throw 'inventory-relative-path'}
    $Bound=$TargetRoots[[int]$Parts[0]];$Canonical=Assert-OwnedPath $Row.absolute $Bound
    $Joined=[IO.Path]::GetFullPath((Join-Path $Bound $Parts[1].Replace('/','\')))
    if(-not $Joined.Equals($Canonical,[StringComparison]::OrdinalIgnoreCase) -or $Expected.ContainsKey($Canonical)){throw 'inventory-path-binding'}
    $Expected[$Canonical]=$Row
    if($Row.retain_in_place){$Keep[$Canonical]=$true}else{$PlannedFiles++;$PlannedBytes+=[long]$Row.bytes}
}
$RequiredKeep=@('C:\file-native-packing-20261020\build\release\qualified_lookup.exe','C:\file-native-packing-20261020\build\release\packing-sample.exe')
if($Keep.Count -ne 2 -or @($RequiredKeep | Where-Object {-not $Keep.ContainsKey($_)}).Count -ne 0){throw 'retained-binary-set'}
$Actual=@{};$Pending=New-Object 'System.Collections.Generic.Stack[string]'
foreach($Bound in $TargetRoots){$null=Assert-OwnedPath $Bound $Bound;$Pending.Push($Bound)}
while($Pending.Count -gt 0){
    $Directory=$Pending.Pop()
    foreach($Item in Get-ChildItem -LiteralPath $Directory -Force){
        if(($Item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw 'reparse-or-junction-in-target'}
        if($Item.PSIsContainer){$Pending.Push($Item.FullName)}else{$Actual[$Item.FullName]=$true}
    }
}
if($Actual.Count -ne $Expected.Count -or @($Actual.Keys | Where-Object {-not $Expected.ContainsKey($_)}).Count -ne 0){throw 'target-namespace-changed-after-archive'}
foreach($Name in $Expected.Keys){$Row=$Expected[$Name];$Item=Get-Item -LiteralPath $Name -Force;if($Item.Length -ne $Row.bytes -or (Get-ExactHash $Name) -ne $Row.sha256){throw 'target-bytes-changed-after-archive'}}
$RemovedFiles=0;$RemovedBytes=[long]0;$Failure=$null
try{
    if($Execute){
        foreach($Row in $Rows){
            if($Row.retain_in_place){continue}
            $Parts=$Row.path -split '/',2;$Canonical=Assert-OwnedPath $Row.absolute $TargetRoots[[int]$Parts[0]]
            $Item=Get-Item -LiteralPath $Canonical -Force
            if($Item.Length -ne $Row.bytes -or (Get-ExactHash $Canonical) -ne $Row.sha256){throw 'changed-before-individual-delete'}
            Remove-Item -LiteralPath $Canonical -Force
            if(Test-Path -LiteralPath $Canonical -PathType Leaf){throw 'delete-not-observed'}
            $RemovedFiles++;$RemovedBytes+=[long]$Row.bytes
            $Progress=[PSCustomObject]@{path=$Canonical;bytes=[long]$Row.bytes;sha256=$Row.sha256;removed_files=$RemovedFiles;removed_bytes=$RemovedBytes}
            [IO.File]::AppendAllText($Receipt+'.jsonl',($Progress | ConvertTo-Json -Compress)+[Environment]::NewLine)
        }
    }
}catch{$Failure=$_.Exception.ToString();throw}finally{
    $Result=[PSCustomObject]@{execute=[bool]$Execute;status=if($Failure){'partial-failure-prefix-preserved'}elseif($Execute){'complete-regular-files-reclaimed'}else{'verified-plan-no-deletion'};targets=$TargetRoots;inventory_sha256=$ExpectedInventory;verified_files=$Rows.Count;planned_files=$PlannedFiles;planned_bytes=$PlannedBytes;removed_files=$RemovedFiles;removed_bytes=$RemovedBytes;retained_files=@($Keep.Keys);empty_directories_retained=$true;failure=$Failure;old_gzip_and_complete_XZ_untouched=$true}
    [IO.File]::WriteAllText($Receipt,($Result | ConvertTo-Json -Depth 6));$Result | ConvertTo-Json -Depth 6
}
