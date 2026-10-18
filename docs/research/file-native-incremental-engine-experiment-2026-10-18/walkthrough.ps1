param([Parameter(Mandatory=$true)][string]$Root,[Parameter(Mandatory=$true)][string]$Destination,[Parameter(Mandatory=$true)][string]$Python)
$ErrorActionPreference='Stop'
if(Test-Path -LiteralPath $Destination){throw 'Fresh destination required'}
Get-Content -LiteralPath (Join-Path $Root 'README.md')
$binding=@(& $Python (Join-Path $PSScriptRoot 'ordinary.py') bind $Root)
if($LASTEXITCODE -ne 0 -or $binding.Count -ne 2){throw 'Binding failed'}
$retained=$binding[0];$manifest=$binding[1]
Get-ChildItem -LiteralPath $retained -Recurse | Select-Object -First 25 FullName
Get-Content -LiteralPath (Join-Path $retained 'NAVIGATION.md')
Get-ChildItem -LiteralPath $retained -Recurse -Filter '*.json' | Select-String -Pattern '1.00|sha256|digest' | Select-Object -First 5
if($env:WALK_READY){
    [IO.File]::WriteAllText($env:WALK_READY,$retained)
    $deadline=[DateTime]::UtcNow.AddSeconds(60)
    while(!(Test-Path -LiteralPath $env:WALK_RELEASE)){
        if([DateTime]::UtcNow -gt $deadline){throw 'Copy barrier timeout'}
        Start-Sleep -Milliseconds 10
    }
}
New-Item -ItemType Directory -Path (Split-Path -Parent $Destination) -Force | Out-Null
Copy-Item -LiteralPath $retained -Destination $Destination -Recurse
if((Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $Destination 'MANIFEST.json')).Hash.ToLower() -ne $manifest){throw 'Copied manifest binding failed'}
& $Python (Join-Path $PSScriptRoot 'ordinary.py') verify $Destination $manifest
if($LASTEXITCODE -ne 0){throw 'File verification failed'}
