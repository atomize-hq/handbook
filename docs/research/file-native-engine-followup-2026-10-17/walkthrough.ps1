param([Parameter(Mandatory=$true)][string]$Store,[Parameter(Mandatory=$true)][string]$Destination)
$ErrorActionPreference='Stop'
# Start with files and their included directions; no engine executable or API.
Get-ChildItem -LiteralPath $Store | Select-Object Name
Get-Content -LiteralPath (Join-Path $Store 'README.md')
$selected=Get-Content -Raw -LiteralPath (Join-Path $Store 'SELECTED.json') | ConvertFrom-Json
$publicationPath=Join-Path $Store ('publications/'+$selected.publication+'.json')
if ((Get-FileHash -LiteralPath $publicationPath -Algorithm SHA256).Hash.ToLower() -ne $selected.digest) {throw 'Publication digest mismatch'}
$publication=Get-Content -Raw -LiteralPath $publicationPath | ConvertFrom-Json
$packet=Join-Path $Store $publication.candidate.closure
$readable=if($publication.candidate.mode -eq 'B'){$packet}else{Join-Path $Store 'view'}
Get-Content -LiteralPath (Join-Path $readable 'NAVIGATION.md')
Get-ChildItem -LiteralPath (Join-Path $readable 'docs') -Recurse -File | Select-Object FullName,Length
$overview=Get-ChildItem -LiteralPath (Join-Path $readable 'docs') -File -Filter '*d000000*'
Get-Content -LiteralPath $overview.FullName -TotalCount 6
Get-ChildItem -LiteralPath (Join-Path $readable 'docs') -File -Filter '*.md' | Select-Object -First 2 | ForEach-Object {Get-Content -LiteralPath $_.FullName -TotalCount 5}
if(Test-Path -LiteralPath $Destination){throw 'Refuse existing extraction destination'}
Copy-Item -LiteralPath $packet -Destination $Destination -Recurse
$manifest=Get-Content -Raw -LiteralPath (Join-Path $Destination 'MANIFEST.json') | ConvertFrom-Json
if ((Get-FileHash -LiteralPath (Join-Path $Destination 'MANIFEST.json') -Algorithm SHA256).Hash.ToLower() -ne $publication.candidate.manifest_digest){throw 'Manifest mismatch'}
foreach($entry in $manifest.files.PSObject.Properties){if((Get-FileHash -LiteralPath (Join-Path $Destination $entry.Name) -Algorithm SHA256).Hash.ToLower() -ne $entry.Value){throw "Copied bytes mismatch: $($entry.Name)"}}
Get-Content -LiteralPath (Join-Path $Destination 'packet/provenance.json')
'EXTRACTION VERIFIED: '+$Destination
