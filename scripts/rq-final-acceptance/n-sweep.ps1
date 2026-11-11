#!/usr/bin/env pwsh
# OS066 T-05: N-sweep — start desktop + N product clients, collect
# [P725-FRAME]/[plan060] frame logs + CPU/MEM cost samples.
# N=1 calc | N=4 calc/clock/todo/charts | N=8 +reader/monitor/photos/video
param(
    [ValidateSet(1, 4, 8)] [int]$N = 1,
    [int]$Seconds = 60,
    [string]$Out = 'D:\autostack\plan-artifacts\plan-068\auto-os\sweep',
    [string]$Root = 'D:\autostack\.wt\os-068\auto-os',
    [string]$ProbeRoot = 'D:\autostack\plan-artifacts\plan-068\auto-os\probe-root',
    [string]$MpvLib = 'D:\autostack\tools\mpv\libmpv-2.dll',
    [ValidateSet('debug', 'release')] [string]$Config = 'debug'
)
$ErrorActionPreference = 'Stop'
$target = Join-Path $Root ".auto\desktop-runtime\target\$Config"
$desktop = Join-Path $target 'ui-desktop.exe'
$cli = Join-Path $target 'auto.exe'
$probe = $ProbeRoot
$ts = Get-Date -Format 'yyyyMMdd-HHmmss'
$dir = Join-Path $Out "N$N-$ts"
New-Item -ItemType Directory -Force -Path $dir | Out-Null

$apps = switch ($N) {
    1 { @(@{ Id = '011-calculator'; Dir = 'D:\autostack\auto-calc' }) }
    4 { @(
        @{ Id = '011-calculator'; Dir = 'D:\autostack\auto-calc' },
        @{ Id = '012-clock'; Dir = 'D:\autostack\auto-clock' },
        @{ Id = '013-todo'; Dir = 'D:\autostack\auto-todo' },
        @{ Id = '024-charts'; Dir = 'D:\autostack\auto-charts' }
    ) }
    8 { @(
        @{ Id = '011-calculator'; Dir = 'D:\autostack\auto-calc' },
        @{ Id = '012-clock'; Dir = 'D:\autostack\auto-clock' },
        @{ Id = '013-todo'; Dir = 'D:\autostack\auto-todo' },
        @{ Id = '024-charts'; Dir = 'D:\autostack\auto-charts' },
        @{ Id = '018-book-reader'; Dir = 'D:\autostack\auto-reader' },
        @{ Id = '025-sys-monitor'; Dir = 'D:\autostack\auto-monitor' },
        @{ Id = '029-photo-gallery'; Dir = 'D:\autostack\auto-photos' },
        @{ Id = '030-video-player'; Dir = 'D:\autostack\auto-video'; Remote = $true }
    ) }
}

$pipe = "autodesk-broker-os068-n$N-ts"
$env:AUTO_OS_ROOT = $probe
$env:AUTOOS_BROKER_PIPE = $pipe
$env:AUTO_FRAME_BENCH = '1'
$env:AUTO_DESKTOP_RQHOST = '1'
$env:AUTO_386_APP_ROOT = Join-Path $dir '386root'
$env:AUTO_MPV_LIB = $MpvLib
New-Item -ItemType Directory -Force -Path $env:AUTO_386_APP_ROOT | Out-Null

$dp = Start-Process -FilePath $desktop -ArgumentList '--apps-dir', (Join-Path $probe 'apps') -WorkingDirectory $root -RedirectStandardError (Join-Path $dir 'desktop.log') -PassThru -WindowStyle Hidden
Start-Sleep -Seconds 8
$procs = @()
foreach ($a in $apps) {
    $args = "run --dir `"$($a.Dir)`" --render-mode=desktop --desktop-endpoint=$pipe --autodesk-incubate --app386=$($a.Id) --autodesk-broker=$pipe"
    if ($a.Remote) { $args += ' --autodesk-render=remote' }
    $c = Start-Process -FilePath $cli -ArgumentList $args -WorkingDirectory $a.Dir -RedirectStandardError (Join-Path $dir "client-$($a.Id).log") -PassThru -WindowStyle Hidden
    $procs += $c
    Write-Host "[sweep] client $($a.Id) pid=$($c.Id)"
}
# cost sampling
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$samples = @()
while ($sw.Elapsed.TotalSeconds -lt $Seconds) {
    $procsNow = Get-Process | Where-Object { $_.ProcessName -match 'ui-desktop|^auto$|mpv|reader|monitor|photos|clock|charts|todo|calc' }
    $cpu = [math]::Round((($procsNow | Measure-Object CPU -Sum).Sum), 1)
    $mem = [math]::Round((($procsNow | Measure-Object WorkingSet64 -Sum).Sum / 1MB), 1)
    $samples += [pscustomobject]@{ T = [math]::Round($sw.Elapsed.TotalSeconds, 1); Proc = $procsNow.Count; CpuS = $cpu; MemMB = $mem }
    Start-Sleep -Seconds 5
}
$samples | Export-Csv -NoTypeInformation -Encoding utf8 (Join-Path $dir 'cost.csv')
foreach ($p in $procs) { Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue }
Stop-Process -Id $dp.Id -Force -ErrorAction SilentlyContinue
Write-Host "[sweep] done N=$N dir=$dir samples=$($samples.Count)"
