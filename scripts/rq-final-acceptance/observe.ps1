#!/usr/bin/env pwsh
# OS066 observe: start iced desktop (isolated base) + one product client,
# capture [P725-FRAME] / [plan060] lines with monotonic ms timestamps.
param(
    [string]$Out,
    [int]$Seconds = 45,
    [switch]$Minimal,
    [string]$ManifestRoot,
    [string]$AppDir,
    [string]$AppId,
    [string]$Pipe,
    [string]$OsRoot,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent | Split-Path -Parent   # .wt/os-066/auto-os
$ts = Get-Date -Format 'yyyyMMdd-HHmmss'
if (-not $Out) { $Out = Join-Path $PSScriptRoot 'out' }
$dir = Join-Path $Out "run-$ts"
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$log = Join-Path $dir 'desktop.log'
$clientLog = Join-Path $dir 'client.log'

$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $root '.auto/desktop-runtime/target' }
$desktop = Join-Path $targetRoot 'debug/ui-desktop.exe'
$cli = Join-Path $targetRoot 'debug/auto.exe'
if (-not (Test-Path -LiteralPath $desktop -PathType Leaf)) { throw "ui-desktop artifact missing: $desktop" }

if ($Minimal) {
    if (-not $ManifestRoot) { $ManifestRoot = 'D:\autostack\plan-artifacts\plan-068\auto-os\probe-root' }
    $osRoot = $ManifestRoot
    if (-not (Test-Path -LiteralPath (Join-Path $osRoot 'apps.manifest') -PathType Leaf)) { throw "minimal manifest missing: $osRoot/apps.manifest" }
    $appsDir = Join-Path $osRoot 'apps'
} else {
    if (-not $OsRoot) { $OsRoot = 'D:\autostack\.wt\os-068\auto-os' }
    $osRoot = $OsRoot
    $appsDir = Join-Path $osRoot 'apps'
}
if (-not $Pipe) { $Pipe = "autodesk-broker-os068-$PID" }
$env:AUTO_OS_ROOT = $osRoot
$env:AUTOOS_BROKER_PIPE = $Pipe
$env:AUTO_FRAME_BENCH = '1'
$env:AUTO_DESKTOP_RQHOST = '1'
$env:AUTO_386_APP_ROOT = Join-Path $dir '386root'
New-Item -ItemType Directory -Force -Path $env:AUTO_386_APP_ROOT | Out-Null

Write-Host "[rq-final] observe ts=$ts desktop=$desktop osRoot=$osRoot pipe=$Pipe sec=$Seconds app=$AppId"
if ($DryRun) { Write-Host '[rq-final] dry-run ok'; return }

$sw = [System.Diagnostics.Stopwatch]::StartNew()
$desktopProc = $null
$clientProc = $null
$clientSpawned = $false
try {
    $psi = [System.Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $desktop
    $psi.Arguments = '--apps-dir', $appsDir
    $psi.WorkingDirectory = $root
    $psi.UseShellExecute = $false
    $psi.RedirectStandardError = $true
    $psi.RedirectStandardOutput = $true
    foreach ($k in @('AUTOOS_BROKER_PIPE','AUTO_FRAME_BENCH','AUTO_DESKTOP_RQHOST','AUTO_386_APP_ROOT','AUTO_OS_ROOT','AUTO_DESKTOP_APPS','AUTO_DESKTOP_APPS_EXTRA','AUTO_MPV_LIB','AUTO_BROKER_ATTACH_BUDGET_MS')) {
        $v = [Environment]::GetEnvironmentVariable($k, 'Process')
        if ($v) { $psi.Environment[$k] = $v }
    }
    $desktopProc = [System.Diagnostics.Process]::new()
    $desktopProc.StartInfo = $psi
    if (-not $desktopProc.Start()) { throw 'desktop failed to start' }

    $stamp = { param($line) ("{0} {1}" -f $sw.ElapsedMilliseconds, $line) }
    while (-not $desktopProc.HasExited) {
        if ($desktopProc.StandardError.Peek() -ge 0) {
            $line = $desktopProc.StandardError.ReadLine()
            if ($null -ne $line) {
                Add-Content -LiteralPath $log -Value (& $stamp $line)
                if ($line -match '\[plan060\]|\[P725-FRAME\]|adopt surface|registry:') {
                    Write-Host "[$($sw.ElapsedMilliseconds)ms] $line"
                }
            }
        }
        if ($AppId -and -not $clientSpawned -and $sw.Elapsed.TotalSeconds -ge 8) {
            $clientSpawned = $true
            $cpsi = [System.Diagnostics.ProcessStartInfo]::new()
            $cpsi.FileName = $cli
            $cpsi.Arguments = 'run --dir "' + $AppDir + '" --render-mode=desktop --desktop-endpoint=' + $Pipe + ' --autodesk-incubate --app386=' + $AppId + ' --autodesk-broker=' + $Pipe
            $cpsi.WorkingDirectory = $AppDir
            $cpsi.UseShellExecute = $false
            $cpsi.RedirectStandardError = $true
            $cpsi.RedirectStandardOutput = $true
            $cpsi.Environment['AUTOOS_BROKER_PIPE'] = $Pipe
            $cpsi.Environment['AUTO_386_APP_ROOT'] = Join-Path $dir '386root'
            $cpsi.Environment['AUTO_OS_ROOT'] = $osRoot
            $clientProc = [System.Diagnostics.Process]::new()
            $clientProc.StartInfo = $cpsi
            if (-not $clientProc.Start()) { Write-Host '[rq-final] WARN client failed to start' }
            else { Write-Host "[rq-final] client spawned pid=$($clientProc.Id) app=$AppId" }
        }
        if ($Seconds -gt 0 -and $sw.Elapsed.TotalSeconds -ge $Seconds) { break }
        Start-Sleep -Milliseconds 10
    }
}
finally {
    if ($clientProc -and -not $clientProc.HasExited) { Stop-Process -Id $clientProc.Id -Force -ErrorAction SilentlyContinue }
    if ($desktopProc -and -not $desktopProc.HasExited) {
        Write-Host "[rq-final] timeout/$Seconds s — terminating desktop pid=$($desktopProc.Id)"
        Stop-Process -Id $desktopProc.Id -Force -ErrorAction SilentlyContinue
    }
    if ($clientLog -and (Test-Path -LiteralPath $clientLog)) { }
    # capture client output before desktop teardown completes
    if ($clientProc) {
        try {
            $co = $clientProc.StandardOutput.ReadToEnd()
            if ($co) { Add-Content -LiteralPath (Join-Path $dir 'client.out') -Value $co }
            $ce = $clientProc.StandardError.ReadToEnd()
            if ($ce) { Add-Content -LiteralPath (Join-Path $dir 'client.err') -Value $ce }
        } catch { }
    }
}
Write-Host "[rq-final] log=$log"
$leftover = Get-Process | Where-Object { $_.ProcessName -match 'ui-desktop|rqhost|auto\.exe' }
if ($leftover) { Write-Host "[rq-final] WARN leftover: $($leftover.Id -join ',')" } else { Write-Host '[rq-final] no leftover desktop/rqhost/auto processes' }
