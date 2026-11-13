$ErrorActionPreference = 'Stop'
$root = 'C:\fnbench'; $public = "$root\public"; $phase = 'account_preflight'; $invoked = $false; $created = $false
function Write-Launch($result, $phase, $kind) {
    @{result=$result; phase=$phase; error_type=$kind} | ConvertTo-Json -Compress | Set-Content "$public\launch.json"
}
Write-Launch 'preflight_in_progress' $phase ''
try {
    if (Get-LocalUser -Name fnbench -ErrorAction SilentlyContinue) { throw 'benchmark account already exists' }
    $password = ConvertTo-SecureString (([guid]::NewGuid().ToString('N')) + ([guid]::NewGuid().ToString('N')) + 'aA1!') -AsPlainText -Force
    New-LocalUser -Name fnbench -Password $password -PasswordNeverExpires | Out-Null; $created = $true
    & icacls $root /grant 'fnbench:(OI)(CI)RX' /T /C | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'read ACL failed' }
    foreach ($path in @("$root\work", "$root\public", "$root\tmp")) {
        & icacls $path /grant 'fnbench:(OI)(CI)M' /T /C | Out-Null
        if ($LASTEXITCODE -ne 0) { throw 'write ACL failed' }
    }
    $credential = [pscredential]::new("$env:COMPUTERNAME\fnbench", $password)
    $args = '"C:\fnbench\src\driver.py" "C:\fnbench\work\fixtures" "C:\fnbench\public"'
    $phase = 'launch_invoked'; Write-Launch 'launch_invoking_outcome_unknown' $phase ''
    $invoked = $true
    $process = Start-Process -FilePath $env:FN_PYTHON -ArgumentList $args -Credential $credential -WorkingDirectory "$root\src" -RedirectStandardOutput "$root\stdout.txt" -RedirectStandardError "$root\stderr.txt" -PassThru -Wait
    $phase = 'process_returned'
    if ($process.ExitCode -ne 0) { Write-Launch 'workload_failed' $phase "exit_$($process.ExitCode)"; throw 'workload failed; inspect events' }
    Write-Launch 'workload_completed' $phase ''
} catch {
    if (-not $invoked) { Write-Launch 'failed_before_launch' $phase $_.Exception.GetType().Name }
    elseif ($phase -ne 'process_returned') { Write-Launch 'launch_outcome_unknown' $phase $_.Exception.GetType().Name }
    throw
} finally {
    if ($created) { Remove-LocalUser -Name fnbench -ErrorAction SilentlyContinue }
}
