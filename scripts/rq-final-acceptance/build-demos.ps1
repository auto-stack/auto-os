#!/usr/bin/env pwsh
# OS066 T-03: build all examples/ui projects with the group-tree auto.exe,
# record pass/fail per project (V-01 denominator buildability).
param([string]$Out = 'D:\autostack\plan-artifacts\plan-066\auto-os\demo-build.csv')
$ErrorActionPreference = 'Continue'
$env:AUTO_LANG_ROOT = 'D:\autostack\.wt\os-066\auto-lang'
$env:AUTO_UI_ROOT = 'D:\autostack\.wt\os-066\auto-ui'
$env:AUTO_MAN_ROOT = 'D:\autostack\.wt\os-066\auto-man'
$auto = 'D:\autostack\.wt\os-066\auto-os\.auto\desktop-runtime\target\debug\auto.exe'
$demos = Get-ChildItem 'D:\autostack\auto-ui\examples\ui' -Directory | Where-Object { Test-Path (Join-Path $_.FullName 'pac.at') } | Sort-Object Name
$rows = @()
foreach ($d in $demos) {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    Push-Location $d.FullName
    $out2 = & $auto build 2>&1
    $code = $LASTEXITCODE
    Pop-Location
    $secs = [math]::Round($sw.Elapsed.TotalSeconds, 1)
    $rows += [pscustomobject]@{ Demo = $d.Name; Exit = $code; Secs = $secs; Tail = ($out2 | Select-Object -Last 1) -join ' ' }
    Write-Host "[demo-build] $($d.Name) exit=$code ${secs}s"
}
$rows | Export-Csv -NoTypeInformation -Encoding utf8 $Out
Write-Host "total=$($rows.Count) pass=$(($rows | Where-Object Exit -eq 0).Count) fail=$(($rows | Where-Object Exit -ne 0).Count)"
