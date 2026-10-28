#!/usr/bin/env pwsh
# Desktop launcher: source group (lang/ui/man) or explicit prebuilt candidates.
[CmdletBinding()]
param(
    [ValidateSet('vue', 'iced')] [string]$Track = 'vue',
    [switch]$Fullscreen,
    [switch]$DryRun,
    [string]$LangRoot,
    [string]$UiRoot,
    [string]$ManRoot,
    [string]$BuildRoot,
    [string]$CandidateDir
)

function Throw-DesktopFailure([string]$Message, [int]$Code = 1) {
    $failure = [System.Exception]::new($Message)
    $failure.Data['DesktopExitCode'] = $Code
    throw $failure
}

function Resolve-DesktopSource([string]$Explicit, [string]$EnvName, [string]$Repo, [string]$Parent, [string]$Probe) {
    $override = if ($Explicit) { $Explicit } else { [Environment]::GetEnvironmentVariable($EnvName, 'Process') }
    $choices = if ($override) { @($override) } else { @((Join-Path $Parent $Repo), "D:/autostack/$Repo") }
    foreach ($choice in $choices) {
        if (Test-Path -LiteralPath (Join-Path $choice $Probe) -PathType Leaf) {
            return (Resolve-Path -LiteralPath $choice -ErrorAction Stop).Path
        }
    }
    Throw-DesktopFailure "$Repo root invalid (parameter/$EnvName -> OS sibling -> main checkout): $($choices -join ', ')"
}

function Invoke-DesktopCargo([string]$Root, [string]$Target, [string[]]$ArgumentList) {
    $ErrorActionPreference = 'Stop'
    $PSNativeCommandUseErrorActionPreference = $false
    $oldTarget = [Environment]::GetEnvironmentVariable('CARGO_TARGET_DIR', 'Process')
    Push-Location -LiteralPath $Root
    try {
        $env:CARGO_TARGET_DIR = $Target
        & cargo @ArgumentList
        $code = $LASTEXITCODE
        if ($code -ne 0) { Throw-DesktopFailure "cargo failed in $Root (exit $code); desktop will not launch" $code }
    }
    finally {
        if ($null -eq $oldTarget) { Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue }
        else { [Environment]::SetEnvironmentVariable('CARGO_TARGET_DIR', $oldTarget, 'Process') }
        Pop-Location
    }
}

function Invoke-DesktopProgram([string]$Executable, [string[]]$ArgumentList) {
    $ErrorActionPreference = 'Stop'
    $PSNativeCommandUseErrorActionPreference = $false
    & $Executable @ArgumentList
    $code = $LASTEXITCODE
    if ($code -ne 0) { Throw-DesktopFailure "desktop command failed (exit $code): $Executable" $code }
}

function Invoke-DesktopMain {
    [CmdletBinding()]
    param(
        [ValidateSet('vue', 'iced')] [string]$Track = 'vue',
        [switch]$Fullscreen, [switch]$DryRun,
        [string]$LangRoot, [string]$UiRoot, [string]$ManRoot,
        [string]$BuildRoot, [string]$CandidateDir
    )
    $ErrorActionPreference = 'Stop'
    $os = if ($env:DESKTOP_OS_ROOT) { $env:DESKTOP_OS_ROOT } else { Join-Path $PSScriptRoot '..' }
    $os = (Resolve-Path -LiteralPath $os -ErrorAction Stop).Path
    $parent = Split-Path $os -Parent
    if ($CandidateDir -and $BuildRoot) { Throw-DesktopFailure 'CandidateDir and BuildRoot are mutually exclusive' }
    $suffix = if ([Environment]::OSVersion.Platform -eq 'Win32NT') { '.exe' } else { '' }
    $mode = if ($CandidateDir) { 'candidate' } else { 'source' }
    if ($mode -eq 'source') {
        $LangRoot = Resolve-DesktopSource $LangRoot 'AUTO_LANG_ROOT' 'auto-lang' $parent 'crates/auto-lang/Cargo.toml'
        $UiRoot = Resolve-DesktopSource $UiRoot 'AUTO_UI_ROOT' 'auto-ui' $parent 'crates/auto-ui-tools/Cargo.toml'
        $ManRoot = Resolve-DesktopSource $ManRoot 'AUTO_MAN_ROOT' 'auto-man' $parent 'crates/auto/Cargo.toml'
        # These workspaces declare sibling path dependencies; env cannot rewrite Cargo manifests.
        if ((Split-Path $LangRoot -Leaf) -ne 'auto-lang' -or (Split-Path $UiRoot -Leaf) -ne 'auto-ui' -or (Split-Path $ManRoot -Leaf) -ne 'auto-man') {
            Throw-DesktopFailure 'source roots must keep auto-lang/auto-ui/auto-man sibling directory names'
        }
        $comparison = if ($suffix) { [StringComparison]::OrdinalIgnoreCase } else { [StringComparison]::Ordinal }
        foreach ($root in @($UiRoot, $ManRoot)) {
            if (-not [string]::Equals((Split-Path $root -Parent), (Split-Path $LangRoot -Parent), $comparison)) {
                Throw-DesktopFailure 'source roots must be a sibling lang/ui/man group (Cargo path dependencies)'
            }
        }
        if (-not $BuildRoot) { $BuildRoot = $env:AUTO_DESKTOP_BUILD_ROOT }
        if (-not $BuildRoot) { $BuildRoot = Join-Path $os '.auto/desktop-runtime' }
        $target = Join-Path ([IO.Path]::GetFullPath($BuildRoot)) 'target'
        $bin = Join-Path $target 'debug'
        $desktop = Join-Path $bin "ui-desktop$suffix"
    }
    else {
        $bin = [IO.Path]::GetFullPath($CandidateDir)
        $desktop = Join-Path $bin "ui-desktop$suffix"
        if ($Track -eq 'vue') {
            $LangRoot = Resolve-DesktopSource $LangRoot 'AUTO_LANG_ROOT' 'auto-lang' $parent 'crates/auto-lang/Cargo.toml'
        }
    }
    if ($Track -eq 'vue' -and -not (Test-Path -LiteralPath (Join-Path $LangRoot 'examples/desktop-host') -PathType Container)) {
        Throw-DesktopFailure 'Vue desktop requires the language examples/desktop-host project'
    }
    $cli = Join-Path $bin "auto$suffix"
    $tool = Join-Path $bin "auto-ui-tool$suffix"
    $required = @($cli, $tool)
    if ($Track -eq 'iced') { $required += $desktop }
    $keys = @('AUTO_LANG_ROOT', 'AUTO_UI_ROOT', 'AUTO_MAN_ROOT', 'AUTO_UI_TOOL_BIN', 'AUTO_OS_ROOT', 'AUTO_DESKTOP_APPS', 'AUTO_DESKTOP_APPS_EXTRA', 'AUTO_DESKTOP_RQHOST', 'AUTO_MPV_LIB')
    $previous = @{}
    foreach ($key in $keys) { $previous[$key] = [Environment]::GetEnvironmentVariable($key, 'Process') }
    try {
        if ($mode -eq 'source') {
            $env:AUTO_LANG_ROOT = $LangRoot; $env:AUTO_UI_ROOT = $UiRoot; $env:AUTO_MAN_ROOT = $ManRoot
        }
        $env:AUTO_UI_TOOL_BIN = $tool
        $env:AUTO_OS_ROOT = $os
        if (-not $env:AUTO_DESKTOP_RQHOST) { $env:AUTO_DESKTOP_RQHOST = '1' }
        if (-not $env:AUTO_MPV_LIB) {
            $mpv = Join-Path $parent 'tools/mpv/libmpv-2.dll'
            if (Test-Path -LiteralPath $mpv -PathType Leaf) { $env:AUTO_MPV_LIB = $mpv }
        }
        if ($Track -eq 'vue') {
            Remove-Item Env:AUTO_DESKTOP_APPS_EXTRA -ErrorAction SilentlyContinue
            $env:AUTO_DESKTOP_APPS = Join-Path $os 'apps'
        }
        Write-Host "[desktop] mode=$mode track=$Track os=$os"
        Write-Host "[desktop] lang=$LangRoot ui=$UiRoot man=$ManRoot"
        Write-Host "[desktop] auto=$cli tool=$tool desktop=$desktop"
        if ($mode -eq 'source') {
            $cliArgs = @('build', '--locked', '-p', 'auto', '--bin', 'auto')
            $toolArgs = @('build', '--locked', '-p', 'auto-ui-tools', '--bin', 'auto-ui-tool')
            if ($Track -eq 'iced') {
                $cliArgs += @('--features', 'mpv-widget')
                $toolArgs += @('--features', 'auto-ui-runtime/mpv-widget')
            }
            $jobs = @(
                @{ Root = $ManRoot; Arguments = $cliArgs },
                @{ Root = $UiRoot; Arguments = $toolArgs }
            )
            if ($Track -eq 'iced') {
                $jobs += @{ Root = $os; Arguments = @('build', '--locked', '-p', 'os-desktop', '--bin', 'ui-desktop') }
            }
            foreach ($job in $jobs) {
                Write-Host "[desktop] cwd=$($job.Root) target=$target cargo $($job.Arguments -join ' ')"
                if (-not $DryRun) { Invoke-DesktopCargo $job.Root $target $job.Arguments }
            }
        }
        if (-not $DryRun) {
            foreach ($file in $required) {
                if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { Throw-DesktopFailure "required desktop artifact missing: $file" }
            }
        }
        $cwd = if ($Track -eq 'iced') { $os } else { Join-Path $LangRoot 'examples/desktop-host' }
        $executable = if ($Track -eq 'iced') { $desktop } else { $cli }
        $arguments = if ($Track -eq 'iced') { @('--apps-dir', (Join-Path $os 'apps')) } else { @('run', '--desktop') }
        if ($Track -eq 'iced' -and $Fullscreen) { $arguments += '--fullscreen' }
        Write-Host "[desktop] cwd=$cwd run=$executable $($arguments -join ' ')"
        if ($DryRun) { return }
        Push-Location -LiteralPath $cwd
        try { Invoke-DesktopProgram $executable $arguments }
        finally { Pop-Location }
    }
    finally {
        foreach ($key in $keys) {
            if ($null -eq $previous[$key]) { Remove-Item -LiteralPath "Env:$key" -ErrorAction SilentlyContinue }
            else { [Environment]::SetEnvironmentVariable($key, $previous[$key], 'Process') }
        }
    }
}

# Dot-sourcing exposes the real orchestration for focused process-boundary tests.
if ($MyInvocation.InvocationName -ne '.') {
    try { Invoke-DesktopMain @PSBoundParameters }
    catch {
        Write-Error $_ -ErrorAction Continue
        $code = if ($_.Exception.Data.Contains('DesktopExitCode')) { [int]$_.Exception.Data['DesktopExitCode'] } else { 1 }
        exit $code
    }
}
