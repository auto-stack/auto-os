#!/usr/bin/env pwsh
# OS068 T-03: build all examples/ui projects with the group-tree auto.exe,
# record pass/fail per project (V-01 denominator buildability).
# R-07: all roots/exe/out are explicit params — no hardcoded os-066 paths,
# no main-checkout generation. Library projects (no src/front/app.at, e.g.
# stylekit) are classified separately: their required form is the dep
# (compile-with-warning), not an app build.
param(
    [string]$UiRoot = 'D:\autostack\.wt\os-068\auto-ui',
    [string]$LangRoot = 'D:\autostack\.wt\os-068\auto-lang',
    [string]$ManRoot = 'D:\autostack\.wt\os-068\auto-man',
    [string]$WebRoot = 'D:\autostack\.wt\os-068\auto-web',
    [string]$DownRoot = 'D:\autostack\.wt\os-068\auto-down',
    [string]$AutoExe = 'D:\autostack\.wt\os-068\auto-man\target\debug\auto.exe',
    [string]$Out = 'D:\autostack\plan-artifacts\plan-068\auto-os\demo-build.csv'
)
$ErrorActionPreference = 'Continue'
$env:AUTO_LANG_ROOT = $LangRoot
$env:AUTO_UI_ROOT = $UiRoot
$env:AUTO_MAN_ROOT = $ManRoot
$env:AUTO_WEB_ROOT = $WebRoot
$env:AUTO_DOWN_ROOT = $DownRoot
$demos = Get-ChildItem (Join-Path $UiRoot 'examples\ui') -Directory | Where-Object { Test-Path (Join-Path $_.FullName 'pac.at') } | Sort-Object Name
$rows = @()
$libs = @()
foreach ($d in $demos) {
    $appEntry = Join-Path $d.FullName 'src\front\app.at'
    if (-not (Test-Path -LiteralPath $appEntry)) {
        $libs += $d.Name
        Write-Host "[demo-build] $($d.Name) LIBRARY (no src/front/app.at; dep form only)"
        continue
    }
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    Push-Location $d.FullName
    $out2 = & $AutoExe build 2>&1
    $code = $LASTEXITCODE
    Pop-Location
    $secs = [math]::Round($sw.Elapsed.TotalSeconds, 1)
    $tail = ($out2 | Select-Object -Last 2) -join ' | '
    $rows += [pscustomobject]@{ Demo = $d.Name; Exit = $code; Secs = $secs; Tail = $tail }
    Write-Host "[demo-build] $($d.Name) exit=$code ${secs}s"
}
$rows | Export-Csv -NoTypeInformation -Encoding utf8 $Out
$pass = ($rows | Where-Object Exit -eq 0).Count
$fail = ($rows | Where-Object Exit -ne 0).Count
Write-Host "total-apps=$($rows.Count) pass=$pass fail=$fail libs=$(($libs -join ',')) out=$Out"
if ($fail -gt 0) { exit 1 }
