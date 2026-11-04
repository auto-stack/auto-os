#!/usr/bin/env pwsh
# PLAN-049 verification entry (design: docs/design/02-local-app-communication-f01.md §8).
# Runs the frozen test families of experimental/communication-f01 by scope.
# Usage: pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope <admission|routing|receipt|lifecycle|resources|acceptance>
param(
    [ValidateSet('admission', 'routing', 'receipt', 'lifecycle', 'resources', 'acceptance')]
    [string]$Scope = 'acceptance'
)

$ErrorActionPreference = 'Stop'
$RepoRoot = Split-Path -Parent $PSScriptRoot
$Ws = Join-Path $RepoRoot 'experimental/communication-f01'
if (-not (Test-Path (Join-Path $Ws 'Cargo.toml'))) {
    Write-Error "communication-f01 workspace not found at $Ws"
}

function Invoke-Family {
    param([string]$Name, [string]$Package, [string]$Target)
    Write-Host "==> [$Scope] $Name" -ForegroundColor Cyan
    & cargo test --locked -p $Package --test $Target --manifest-path (Join-Path $Ws 'Cargo.toml')
    if ($LASTEXITCODE -ne 0) {
        Write-Error "family $Name failed (exit $LASTEXITCODE)"
    }
}

# build once so the bin-spawning tests reuse the same artifacts
Write-Host "==> build (locked)" -ForegroundColor Cyan
& cargo build --locked --workspace --manifest-path (Join-Path $Ws 'Cargo.toml')
if ($LASTEXITCODE -ne 0) { Write-Error "build failed" }

switch ($Scope) {
    'admission' {
        Invoke-Family 'f0 wire contract (V-02/03/04 admission legs)' 'autoos-comm-core' 'f0_contract'
        Write-Host '==> [admission] bounded frame IO (V-03 framing legs)' -ForegroundColor Cyan
        & cargo test --locked -p autoos-comm-local --lib --manifest-path (Join-Path $Ws 'Cargo.toml')
        if ($LASTEXITCODE -ne 0) { Write-Error 'frame IO family failed' }
    }
    'routing' {
        Invoke-Family 'routing contract (V-04/06/07/08/12)' 'autoos-comm-demo' 'f1_route'
    }
    'receipt' {
        Invoke-Family 'capture domain (V-09/10/11)' 'autoos-comm-demo' 'capture_receipt'
    }
    'lifecycle' {
        Invoke-Family 'lifecycle + T-01 full-path gate (V-01/06/13)' 'autoos-comm-demo' 'lifecycle'
    }
    'resources' {
        Invoke-Family 'resource boundaries (V-05/12)' 'autoos-comm-demo' 'resources'
    }
    'acceptance' {
        Invoke-Family 'f0 wire contract' 'autoos-comm-core' 'f0_contract'
        Write-Host '==> [acceptance] bounded frame IO' -ForegroundColor Cyan
        & cargo test --locked -p autoos-comm-local --lib --manifest-path (Join-Path $Ws 'Cargo.toml')
        if ($LASTEXITCODE -ne 0) { Write-Error 'frame IO family failed' }
        Invoke-Family 'routing' 'autoos-comm-demo' 'f1_route'
        Invoke-Family 'capture domain' 'autoos-comm-demo' 'capture_receipt'
        Invoke-Family 'lifecycle + full-path gate' 'autoos-comm-demo' 'lifecycle'
        Invoke-Family 'resource boundaries' 'autoos-comm-demo' 'resources'
        Invoke-Family 'acceptance workload (V-14)' 'autoos-comm-demo' 'acceptance'
    }
}

Write-Host "==> [$Scope] all families passed" -ForegroundColor Green
