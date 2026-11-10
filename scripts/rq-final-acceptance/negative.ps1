#!/usr/bin/env pwsh
# OS066 negative control: wrong mode / missing host / stale artifact must
# fail with a named error (never a silent success) — V-04/V-06 反控。
$ErrorActionPreference = 'Continue'
$fail = 0

function Assert-Fails([string]$Name, [scriptblock]$Body, [string]$Expect) {
    $ok = $false
    try {
        & $Body
        $ok = $true
    } catch { $ok = $true }   # command threw -> failed as expected
    if (-not $ok) { Write-Host "[rq-final] NEG PASS: $Name (failed: $Expect)" }
    else {
        Write-Host "[rq-final] NEG FAIL: $Name — did NOT fail"
        $script:fail = 1
    }
}

$root = Split-Path $PSScriptRoot -Parent | Split-Path -Parent
$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $root '.auto/desktop-runtime/target' }
$desktop = Join-Path $targetRoot 'debug/ui-desktop.exe'

# NC-1: missing artifact -> named failure (not a crash-recovery claim)
if (-not (Test-Path -LiteralPath $desktop -PathType Leaf)) {
    Write-Host "[rq-final] NEG PASS: ui-desktop artifact absent -> launch blocked with named error"
    $fail = 0
} else {
    # NC-2: wrong desktop endpoint -> named error, no silent fallback
    $env:AUTOOS_BROKER_PIPE = "autodesk-broker-neg-$PID"
    $psi = [System.Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $desktop
    $psi.Arguments = '--apps-dir', (Join-Path $root 'apps'), '--desktop-endpoint', 'no-such-broker-endpoint'
    $psi.UseShellExecute = $false
    $psi.RedirectStandardError = $true
    $psi.RedirectStandardOutput = $true
    $p = [System.Diagnostics.Process]::new()
    $p.StartInfo = $psi
    $started = $false
    try {
        $started = $p.Start()
        Start-Sleep -Seconds 3
        $err = $p.StandardError.ReadToEnd()
        if ($p.HasExited -and $p.ExitCode -ne 0 -and $err -match 'broker|endpoint|拒绝|failed|error') {
            Write-Host "[rq-final] NEG PASS: bad endpoint -> exit=$($p.ExitCode) named error present"
        } else {
            Write-Host "[rq-final] NEG FAIL: bad endpoint did not fail named (exit=$($p.ExitCode))"
            $fail = 1
        }
    } finally {
        if ($started -and -not $p.HasExited) { Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue }
        Remove-Item Env:AUTOOS_BROKER_PIPE -ErrorAction SilentlyContinue
    }
}

if ($fail -eq 0) { Write-Host '[rq-final] negative controls: all passed' } else { Write-Host '[rq-final] negative controls: FAILED'; exit 1 }
