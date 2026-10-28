#!/usr/bin/env pwsh
# scripts/check-app-submodules.ps1 — apps/ 子模块健康闸门（PLAN-876 phase：OS876-SUBMODULE 守卫）
#
# 背景（2026-10-28 实测）：`apps/` 下 24 个子模块工作树曾被清空——只剩 `.git`
# 指针、索引里是成批 `D `（暂存删除）。表征：桌面注册表 30 条缩到 4 条、
# 小组件 `faces=0`，宿主只表现为"图标不对/没有小组件"。
#
# 本闸门把该签名变成确定性检查（启动桌面或合入前可跑）：
#   ①子模块未初始化（`git submodule status` 行首 `-`）或指针不匹配（`+`/`U`）；
#   ②已初始化模块出现"批量暂存删除"（`^D ` 行 > 阈值）= 工作区被清空；
#   ③`apps/` 有 `pac.at` 的目录数 < `apps.manifest` 的 active repo 条目数。
#
# 用法：
#   pwsh -File scripts/check-app-submodules.ps1              # 体检本仓
#   pwsh -File scripts/check-app-submodules.ps1 -SelfTest    # 验证检测器本身
#   pwsh -File scripts/check-app-submodules.ps1 -OsRoot D:/autostack/auto-os
# 退出码：0 = healthy；1 = 发现缺口；2 = 环境/用法错误。
[CmdletBinding()]
param(
    [string]$OsRoot,
    [int]$Threshold = 5,
    [switch]$SelfTest
)

$ErrorActionPreference = 'Stop'

function Get-StagedDeletions([string]$dir) {
    $out = & git -C $dir status --porcelain 2>$null
    if ($LASTEXITCODE -ne 0) { return -1 }
    @($out | Where-Object { $_ -match '^D ' }).Count
}

if ($SelfTest) {
    $tmp = Join-Path ([IO.Path]::GetTempPath()) ("p876-submod-selftest-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force -Path $tmp | Out-Null
    try {
        & git -C $tmp init -q | Out-Null
        & git -C $tmp config user.email t@example.com
        & git -C $tmp config user.name t
        1..10 | ForEach-Object { Set-Content -Path (Join-Path $tmp "f$_.txt") -Value 'x' -NoNewline }
        & git -C $tmp add -A | Out-Null
        & git -C $tmp commit -qm fixture
        $clean = Get-StagedDeletions $tmp
        & git -C $tmp rm -rq . | Out-Null
        $wiped = Get-StagedDeletions $tmp
        Write-Host "[self-test] clean=$clean wiped=$wiped threshold=$Threshold"
        if ($clean -ne 0 -or $wiped -le $Threshold) {
            Write-Error "[app-submodules] FAIL: detector cannot separate clean vs wiped states" -ErrorAction Continue
            exit 1
        }
        Write-Host '[self-test] detector OK'
        exit 0
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

$root = if ($OsRoot) { (Resolve-Path -LiteralPath $OsRoot).Path }
        else { (Resolve-Path (Join-Path $PSScriptRoot '..')).Path }
Push-Location $root
try {
    $fail = $false
    $status = & git submodule status --recursive 2>$null
    if (-not $status) {
        Write-Host '[app-submodules] FAIL: no submodules registered (apps/ empty?)'
        exit 1
    }
    foreach ($line in $status) {
        if ($line -match '^-') { Write-Host "[app-submodules] UNINITIALIZED: $line"; $fail = $true }
        elseif ($line -match '^[+U]') { Write-Host "[app-submodules] MISMATCH: $line"; $fail = $true }
    }
    foreach ($line in $status) {
        if ($line -notmatch '^\s*[0-9a-f]+\s+(\S+)') { continue }
        $path = $Matches[1]
        if (-not (Test-Path -LiteralPath $path -PathType Container)) { continue }
        $n = Get-StagedDeletions $path
        if ($n -gt $Threshold) {
            Write-Host "[app-submodules] WIPED? $path staged_deletions=$n"
            $fail = $true
        }
    }
    $manifest = Join-Path $root 'apps.manifest'
    if (Test-Path -LiteralPath $manifest -PathType Leaf) {
        # 逐条按 manifest 的 repo 路径解析（覆盖 nested `apps/x/app` 与仓外 `../auto-musk`），
        # 不按 apps/ 一级目录计数——深度与外置根都会让计数失真。
        $entries = @((Get-Content -LiteralPath $manifest -Raw | ConvertFrom-Json).apps |
            Where-Object { $_.status -eq 'active' -and $_.kind -eq 'repo' })
        $missing = @($entries | Where-Object {
                -not (Test-Path -LiteralPath (Join-Path $root (Join-Path $_.repo 'pac.at')) -PathType Leaf)
            })
        Write-Host "[app-submodules] manifest_repo_entries=$(@($entries).Count) missing_pac_at=$(@($missing).Count)"
        if (@($missing).Count -gt 0) {
            Write-Host "[app-submodules] COVERAGE: missing pac.at for [$((@($missing.id) -join ','))]"
            $fail = $true
        }
    }
    if ($fail) {
        Write-Host '[app-submodules] 修复建议：git submodule update --init --recursive'
        Write-Host '  （若某模块索引已成批删除而文件不在：先在模块内 git reset --hard HEAD，再 recheck）'
        exit 1
    }
    Write-Host '[app-submodules] healthy'
    exit 0
} finally {
    Pop-Location
}
