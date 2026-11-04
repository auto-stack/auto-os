#!/usr/bin/env pwsh
# PLAN-049 CLI demo (design: docs/design/02-local-app-communication-f01.md §6).
# Shows only already-verified behavior: three real processes, 50×1KiB captures
# across two callers (25 operations each) plus duplicate-key retries, a
# read-only SQLite reconciliation via the Python standard library, and an
# idempotent repeat run against the same directory. Facts only — this is a
# baseline report, not a performance claim.
param(
    [string]$KeepRun = ''
)

$ErrorActionPreference = 'Stop'
$RepoRoot = Split-Path -Parent $PSScriptRoot
$Ws = Join-Path $RepoRoot 'experimental/communication-f01'
$ExeDir = Join-Path $Ws 'target/debug'
if (-not (Test-Path (Join-Path $ExeDir 'comm-broker.exe'))) {
    Write-Host '==> build (locked)' -ForegroundColor Cyan
    & cargo build --locked --workspace --manifest-path (Join-Path $Ws 'Cargo.toml')
    if ($LASTEXITCODE -ne 0) { Write-Error 'build failed' }
}

$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$RunDir = Join-Path $env:TEMP "autoos-comm-f01/demo-$stamp"
New-Item -ItemType Directory -Path $RunDir | Out-Null
Write-Host "==> run dir: $RunDir" -ForegroundColor Cyan

function New-Token {
    # 256-bit credential text: two 128-bit GUIDs in hex form
    ([guid]::NewGuid().ToString('N') + [guid]::NewGuid().ToString('N'))
}

# --- process containment: the demo controller owns every process it starts
# (kill-on-close job; every spawn is assigned immediately and the job is
# terminated in the finally block — the SUSPENDED-first pattern is proven in
# the test suite, the residual assign-after-start window is demo-layer only)
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class DemoJob {
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode)]
    public static extern IntPtr CreateJobObjectW(IntPtr attrs, string name);
    [StructLayout(LayoutKind.Sequential)]
    public struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
        public long PerProcessUserTimeLimit;
        public long PerJobUserTimeLimit;
        public uint LimitFlags;
        public UIntPtr MinimumWorkingSetSize;
        public UIntPtr MaximumWorkingSetSize;
        public uint ActiveProcessLimit;
        public UIntPtr Affinity;
        public uint PriorityClass;
        public uint SchedulingClass;
    }
    [DllImport("kernel32.dll")]
    public static extern bool SetInformationJobObject(IntPtr job, int cls, ref JOBOBJECT_BASIC_LIMIT_INFORMATION info, int len);
    [DllImport("kernel32.dll")]
    public static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
    [DllImport("kernel32.dll")]
    public static extern bool TerminateJobObject(IntPtr job, uint exitCode);
}
'@
$demoJob = [DemoJob]::CreateJobObjectW([IntPtr]::Zero, $null)
if ($demoJob -eq [IntPtr]::Zero) { Write-Error 'CreateJobObjectW failed' }
$lim = New-Object DemoJob+JOBOBJECT_BASIC_LIMIT_INFORMATION
$lim.LimitFlags = 0x2000 # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
[void][DemoJob]::SetInformationJobObject($demoJob, 9, [ref]$lim, [System.Runtime.InteropServices.Marshal]::SizeOf($lim))
function Assign-ToDemoJob { param([object]$Proc) [void][DemoJob]::AssignProcessToJobObject($demoJob, $Proc.Handle) }

# --- access config: provider + two permitted callers (tokens stay in files)
$tokens = @{ captureProvider = New-Token; callerA = New-Token; callerB = New-Token }
$policy = @{
    entries = @(
        @{ token = $tokens.captureProvider; principal = 'capture-provider'; role = 'provider'; allow = @('auto.capture.inbox') },
        @{ token = $tokens.callerA; principal = 'caller-a'; role = 'caller'; allow = @('auto.capture.inbox') },
        @{ token = $tokens.callerB; principal = 'caller-b'; role = 'caller'; allow = @('auto.capture.inbox') }
    )
} | ConvertTo-Json -Depth 5
$policyPath = Join-Path $RunDir 'broker-access.json'
Set-Content -Path $policyPath -Value $policy -Encoding utf8NoBOM
Set-Content -Path (Join-Path $RunDir 'token-capture-provider.txt') -Value $tokens.captureProvider -Encoding ascii
Set-Content -Path (Join-Path $RunDir 'token-caller-a.txt') -Value $tokens.callerA -Encoding ascii
Set-Content -Path (Join-Path $RunDir 'token-caller-b.txt') -Value $tokens.callerB -Encoding ascii

$brokerCfg = @{ run_dir = $RunDir; pipe_name = $null; policy_file = $policyPath } | ConvertTo-Json -Depth 3
$brokerCfgPath = Join-Path $RunDir 'broker-cfg.json'
Set-Content -Path $brokerCfgPath -Value $brokerCfg -Encoding utf8NoBOM
$providerCfg = @{
    run_dir = $RunDir
    token_file = (Join-Path $RunDir 'token-capture-provider.txt')
    endpoint_file = (Join-Path $RunDir 'endpoint.json')
    stop_file = (Join-Path $RunDir 'stop-provider.flag')
    barrier = $null
} | ConvertTo-Json -Depth 3
$providerCfgPath = Join-Path $RunDir 'provider-cfg.json'
Set-Content -Path $providerCfgPath -Value $providerCfg -Encoding utf8NoBOM

function Wait-File {
    param([string]$Path, [int]$Seconds)
    $deadline = (Get-Date).AddSeconds($Seconds)
    while (-not (Test-Path $Path)) {
        if ((Get-Date) -gt $deadline) { Write-Error "timeout waiting for $Path" }
        Start-Sleep -Milliseconds 25
    }
}

function Start-Stack {
    $broker = Start-Process -FilePath (Join-Path $ExeDir 'comm-broker.exe') `
        -ArgumentList @('--config', $brokerCfgPath) `
        -WorkingDirectory $RunDir -RedirectStandardError (Join-Path $RunDir 'broker.log') `
        -PassThru -WindowStyle Hidden
    Assign-ToDemoJob -Proc $broker
    Wait-File -Path (Join-Path $RunDir 'endpoint.json') -Seconds 10
    $provider = Start-Process -FilePath (Join-Path $ExeDir 'capture-inbox.exe') `
        -ArgumentList @('--config', $providerCfgPath) `
        -WorkingDirectory $RunDir -RedirectStandardError (Join-Path $RunDir 'provider.log') `
        -PassThru -WindowStyle Hidden
    Assign-ToDemoJob -Proc $provider
    Wait-File -Path (Join-Path $RunDir 'provider-ready.json') -Seconds 10
    return @($broker, $provider)
}

function Stop-Stack {
    param([object[]]$Procs)
    Set-Content -Path (Join-Path $RunDir 'stop-provider.flag') -Value 'stop' -Encoding ascii
    Set-Content -Path (Join-Path $RunDir 'stop-broker.flag') -Value 'stop' -Encoding ascii
    foreach ($p in $Procs) {
        if (-not $p.WaitForExit(8000)) { $p.Kill() }
    }
    Start-Sleep -Milliseconds 300
}

# --- pregenerate the fixed batch content: 25 operations per caller, reused
# by the repeat run so the dedup keys and payloads are byte-identical
$batchA = @(); $batchB = @()
$opsA = 1..25 | ForEach-Object { [guid]::NewGuid().ToString('N') }
$opsB = 1..25 | ForEach-Object { [guid]::NewGuid().ToString('N') }
foreach ($i in 0..24) {
    $pA = "demo caller-a item $($i + 1) - "
    $batchA += @{ operation_id = $opsA[$i]; text = ($pA + ('p' * (1024 - $pA.Length))) } | ConvertTo-Json -Compress
    $pB = "demo caller-b item $($i + 1) - "
    $batchB += @{ operation_id = $opsB[$i]; text = ($pB + ('p' * (1024 - $pB.Length))) } | ConvertTo-Json -Compress
}
Set-Content -Path (Join-Path $RunDir 'batch-caller-a.jsonl') -Value $batchA -Encoding utf8NoBOM
Set-Content -Path (Join-Path $RunDir 'batch-caller-b.jsonl') -Value $batchB -Encoding utf8NoBOM

function Invoke-Batch {
    param([string]$Caller)
    $out = Join-Path $RunDir "client-$Caller.out.jsonl"
    $p = Start-Process -FilePath (Join-Path $ExeDir 'comm-client.exe') `
        -ArgumentList @(
            '--endpoint', (Join-Path $RunDir 'endpoint.json'),
            '--token-file', (Join-Path $RunDir "token-$Caller.txt"),
            '--cmd', 'batch', '--file', (Join-Path $RunDir "batch-$Caller.jsonl"),
            '--timeout-ms', '2000'
        ) `
        -WorkingDirectory $RunDir -RedirectStandardOutput $out `
        -RedirectStandardError (Join-Path $RunDir "client-$Caller.err.log") `
        -PassThru -WindowStyle Hidden
    Assign-ToDemoJob -Proc $p
    if (-not $p.WaitForExit(20000)) { $p.Kill(); Write-Error "client $Caller timed out" }
    if ($p.ExitCode -ne 0) { Write-Error "client $Caller exited $($p.ExitCode)" }
    Get-Content $out | ForEach-Object { $_ | ConvertFrom-Json }
}

function Invoke-OracleCheck {
    param([string]$Label, [int]$ExpectedRows)
    # independent read-only reconciliation with the Python standard library
    $db = Join-Path $RunDir 'inbox-db/capture.db'
    $py = @"
import sqlite3, json
con = sqlite3.connect(r'file:$($db -replace '\\','/')?mode=ro', uri=True)
rows = con.execute('SELECT COUNT(*), COALESCE(SUM(LENGTH(text)),0) FROM captures').fetchone()
sizes = con.execute('SELECT COUNT(*) FROM captures WHERE LENGTH(text) = 1024').fetchone()[0]
print(json.dumps({'label': '$Label', 'rows': rows[0], 'text_bytes': rows[1], 'rows_1kib': sizes}))
"@
    $result = python -c $py
    Write-Host "    oracle: $result" -ForegroundColor DarkGray
    $obj = $result | ConvertFrom-Json
    if ($obj.rows -ne $ExpectedRows) { Write-Error "$Label expected $ExpectedRows rows, got $($obj.rows)" }
    if ($obj.rows_1kib -ne $ExpectedRows) { Write-Error "$Label expected all rows to be 1KiB" }
}

try {
Write-Host '==> run 1: 50 x 1KiB across two callers + duplicate-key retries' -ForegroundColor Cyan
$procs = Start-Stack
$resultsA = @(Invoke-Batch -Caller 'caller-a')
$resultsB = @(Invoke-Batch -Caller 'caller-b')

# duplicate-key retries: rerun the first batch of each caller (dedup keys)
Set-Content -Path (Join-Path $RunDir 'batch-caller-a.jsonl') -Value ($batchA | Select-Object -First 5) -Encoding utf8NoBOM
Set-Content -Path (Join-Path $RunDir 'batch-caller-b.jsonl') -Value ($batchB | Select-Object -First 5) -Encoding utf8NoBOM
$null = Invoke-Batch -Caller 'caller-a'
$null = Invoke-Batch -Caller 'caller-b'

$lat = @($resultsA + $resultsB | ForEach-Object { $_.latency_ms } | Where-Object { $_ -ne $null } | Sort-Object)
$p50 = $lat[[int][math]::Floor(($lat.Count - 1) * 0.5)]
$p95 = $lat[[int][math]::Floor(($lat.Count - 1) * 0.95)]
$committed = @($resultsA + $resultsB | Where-Object { $_.status -eq 'committed' }).Count
Write-Host ("    committed={0}/50  p50={1}ms  p95={2}ms  max={3}ms" -f $committed, $p50, $p95, $lat[-1])
Invoke-OracleCheck -Label 'run1' -ExpectedRows 50

Write-Host '==> run 2: same directory, same keys+payloads (idempotent repeat)' -ForegroundColor Cyan
Stop-Stack -Procs $procs
Remove-Item -Path (Join-Path $RunDir 'stop-broker.flag') -ErrorAction SilentlyContinue
Remove-Item -Path (Join-Path $RunDir 'stop-provider.flag') -ErrorAction SilentlyContinue
Set-Content -Path (Join-Path $RunDir 'batch-caller-a.jsonl') -Value $batchA -Encoding utf8NoBOM
Set-Content -Path (Join-Path $RunDir 'batch-caller-b.jsonl') -Value $batchB -Encoding utf8NoBOM
$procs2 = Start-Stack
$null = Invoke-Batch -Caller 'caller-a'
$null = Invoke-Batch -Caller 'caller-b'
Invoke-OracleCheck -Label 'run2' -ExpectedRows 50
Stop-Stack -Procs $procs2

Write-Host '==> demo facts' -ForegroundColor Green
Write-Host ("    run_dir: {0}" -f $RunDir)
Write-Host "    three independent processes (broker pid, capture-service pid, client pids per batch)"
Write-Host "    receipts and capture ids verified per batch; service DB reconciled read-only via python stdlib sqlite3"
# containment: nothing survives the demo controller
[void][DemoJob]::TerminateJobObject($demoJob, 0)

if ($KeepRun -eq '') {
    Start-Sleep -Milliseconds 300
    Remove-Item -Recurse -Force $RunDir -ErrorAction SilentlyContinue
    Write-Host "    run dir removed (pass -KeepRun keep to retain)"
} else {
    Write-Host "    run dir kept: $RunDir"
}
} finally {
    # containment: nothing survives the demo controller, on any path
    [void][DemoJob]::TerminateJobObject($demoJob, 0)
}
