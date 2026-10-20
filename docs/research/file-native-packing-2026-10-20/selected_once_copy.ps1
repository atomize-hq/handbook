param([Parameter(Mandatory=$true)][string]$Root,[Parameter(Mandatory=$true)][string]$Destination,[string]$CapturedSelection,[string]$SaveSelection)
$ErrorActionPreference='Stop'
$SelectionFile=if($CapturedSelection){$CapturedSelection}else{Join-Path $Root 'SELECTED.json'}
$SelectionRaw=[System.IO.File]::ReadAllBytes($SelectionFile)
if($SaveSelection){[System.IO.File]::WriteAllBytes($SaveSelection,$SelectionRaw)}
$Selection=[System.Text.Encoding]::UTF8.GetString($SelectionRaw) | ConvertFrom-Json
$PublicationPath=Join-Path $Root ('publications/'+$Selection.publication+'.json')
$Hasher=[System.Security.Cryptography.SHA256]::Create()
$PublicationDigest=[BitConverter]::ToString($Hasher.ComputeHash([System.IO.File]::ReadAllBytes($PublicationPath))).Replace('-','').ToLowerInvariant()
if($PublicationDigest -ne $Selection.digest){throw 'selected-publication-digest'}
$Publication=Get-Content -LiteralPath $PublicationPath -Raw | ConvertFrom-Json
$Generation=Join-Path $Root ('generations/'+$Publication.attempt)
if(-not (Test-Path -LiteralPath $Generation -PathType Container)){throw 'selected-generation-missing'}
$GenerationManifest=Join-Path $Generation 'MANIFEST.json'
$ManifestDigest=[BitConverter]::ToString($Hasher.ComputeHash([System.IO.File]::ReadAllBytes($GenerationManifest))).Replace('-','').ToLowerInvariant()
if($ManifestDigest -ne $Publication.generation_manifest){throw 'selected-generation-manifest-digest'}
if(Test-Path -LiteralPath $Destination){throw 'fresh-copy-destination-required'}
Copy-Item -LiteralPath $Generation -Destination $Destination -Recurse
$Rows=@(Get-ChildItem -LiteralPath $Destination -Recurse -File | Sort-Object FullName | ForEach-Object {
    [PSCustomObject]@{path=$_.FullName.Substring($Destination.Length+1).Replace('\','/');bytes=$_.Length;sha256=[BitConverter]::ToString($Hasher.ComputeHash([System.IO.File]::ReadAllBytes($_.FullName))).Replace('-','').ToLowerInvariant()}
})
[PSCustomObject]@{selected=$Selection;attempt=$Publication.attempt;generation=$Generation;files=$Rows;discovery='ordinary PowerShell; SELECTED captured exactly once; no engine or pack decoder'} | ConvertTo-Json -Depth 8
