# Process control for this dated experiment only. Never alters store contents.
$experimentCutoff=[datetimeoffset]'2026-10-17T16:48:00Z'
while ($true) {
    $experimentDrivers=@(Get-CimInstance Win32_Process -Filter "name='python.exe'" | Where-Object { $_.CommandLine -like '*file-native-publication-experiment-2026-10-17/run.py growth*' })
    $experimentFree=(Get-PSDrive -Name C).Free
    if ([datetimeoffset]::UtcNow -ge $experimentCutoff -or $experimentFree -lt 171798691840) {
        $experimentChildren=@(Get-CimInstance Win32_Process | Where-Object { $_.ParentProcessId -in @($experimentDrivers.ProcessId) -and $_.Name -eq 'file-native-publication-experiment.exe' })
        $experimentEvent=@{reason='declared cutoff or reserve';time_utc=[datetimeoffset]::UtcNow.ToString('o');drivers=@($experimentDrivers.ProcessId);children=@($experimentChildren.ProcessId);free_bytes=$experimentFree}
        $experimentName=if ($experimentDrivers.Count -gt 0) {'growth-cap-stop.json'}else{'cap-guard-completed.json'}
        $experimentEvent | ConvertTo-Json | Set-Content -LiteralPath (Join-Path 'C:/file-native-experiment-20261017-v3' $experimentName) -Encoding utf8
        $experimentDrivers | ForEach-Object { Stop-Process -Id $_.ProcessId -ErrorAction SilentlyContinue }
        $experimentChildren | ForEach-Object { Stop-Process -Id $_.ProcessId -ErrorAction SilentlyContinue }
        $experimentEvent | ConvertTo-Json
        break
    }
    Start-Sleep -Seconds 2
}
