#!/usr/bin/env pwsh
# scripts/desktop.ps1 — Stage B §3-a 桌面薄包装（PLAN-009 T1，auto-os Design 01 §3 选项 a）
#
# 解析序定位 auto-lang，注入 env 调既有两条桌面入口——框架仓零改动：
#   - vue 轨（缺省）：CWD=<lang>/examples/desktop-host，`auto run --desktop`
#     （注入 AUTO_OS_ROOT + AUTO_DESKTOP_APPS_EXTRA=<os>/apps——desktop-host
#     项目内兄弟探测够不到本仓，经 env 显式聚合；注册表=框架 demo+本仓 apps/+kanban 三源）
#   - iced 轨：CWD=<本仓根>，cargo run ui_desktop（`../auto-os/apps` 兄弟探测
#     自命中——CWD 在本仓根时 `..`/auto-os 解析回本仓，P-3 容器探测无需 env）
#
# 解析序（沿本仓 AGENTS §2）：$AUTO_LANG_ROOT → 兄弟 ../auto-lang → 主检出
# D:/autostack/auto-lang。用法：
#   ./scripts/desktop.ps1                    # vue 轨
#   ./scripts/desktop.ps1 -Track iced        # iced/VM 轨（验收宿主）
#   ./scripts/desktop.ps1 -Track iced -Fullscreen
#   ./scripts/desktop.ps1 -DryRun            # 只打印解析与将执行的命令
[CmdletBinding()]
param(
    [ValidateSet('vue', 'iced')] [string]$Track = 'vue',
    [switch]$Fullscreen,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'

# 本仓根（脚本位于 <os>/scripts/；DESKTOP_OS_ROOT env 可显式覆盖——指向
# 另一伞形检出/主检出用，缺省=脚本所在仓根）。
$OsRoot = if ($env:DESKTOP_OS_ROOT) { (Resolve-Path $env:DESKTOP_OS_ROOT).Path }
          else { (Resolve-Path (Join-Path $PSScriptRoot '..')).Path }
$OsParent = Split-Path $OsRoot -Parent

function Test-LangRoot([string]$p) {
    (Test-Path (Join-Path $p 'crates\auto-lang')) -and (Test-Path (Join-Path $p 'examples\desktop-host'))
}

# 解析序：env → 兄弟 → 主检出
$LangRoot = $null
foreach ($c in @($env:AUTO_LANG_ROOT, (Join-Path $OsParent 'auto-lang'), 'D:/autostack/auto-lang')) {
    if ($c -and (Test-LangRoot $c)) { $LangRoot = (Resolve-Path $c).Path; break }
}
if (-not $LangRoot) {
    Write-Error "auto-lang 未解析（解析序：AUTO_LANG_ROOT env → 兄弟 auto-lang → D:/autostack/auto-lang，探针=crates/auto-lang+examples/desktop-host）"
}

# auto CLI 解析（vue 轨）：PATH → lang target/release → target/debug
function Resolve-AutoCli {
    foreach ($c in @(
        (Get-Command auto -ErrorAction SilentlyContinue | Select-Object -First 1 -ExpandProperty Source),
        (Join-Path $LangRoot 'target\release\auto.exe'),
        (Join-Path $LangRoot 'target\debug\auto.exe')
    )) { if ($c -and (Test-Path $c)) { return $c } }
    return $null
}

$env:AUTO_OS_ROOT = $OsRoot   # manifest 聚合 env 臂（P-3：设置即权威）

if ($Track -eq 'vue') {
    $autoCli = Resolve-AutoCli
    if (-not $autoCli) {
        Write-Error "auto CLI 未找到：请先在 auto-lang 构建（cargo build -p auto）或将其加入 PATH"
    }
    # 复审补二：AUTO_DESKTOP_APPS_EXTRA 是「单 app 根路径表」全替换语义
    # （vue.rs desktop_extra_app_roots env 臂——无容器展开/不并 manifest）——
    # 容器须在脚本侧展开为子目录列表（';' 分隔）。追加 os-config 单根；
    # kanban(repo 形态)留缺省臂，vue 宿主 v1 front-only 本就跳过需后端 app。
    $extra = @(Get-ChildItem -Directory (Join-Path $OsRoot 'apps') -ErrorAction SilentlyContinue |
        Where-Object { Test-Path (Join-Path $_.FullName 'pac.at') } |
        ForEach-Object { $_.FullName })
    $osConfig = Join-Path $OsParent 'auto-os-config\auto'
    if (Test-Path $osConfig) { $extra += $osConfig }
    # PLAN-008：顶层画廊两件随 EXTRA 显式注入（env 全替换语义下脚本化
    # vue 轨的画廊供给；widgets-gallery render=vm 由注册表 vue 过滤自然
    # 排除——设计行为）。
    foreach ($g in @('ui-gallery', 'widgets-gallery')) {
        $gdir = Join-Path $OsRoot $g
        if (Test-Path $gdir) { $extra += $gdir }
    }
    $env:AUTO_DESKTOP_APPS_EXTRA = ($extra -join ';')
    # 复审补（T1 漏注）：vue 轨主注册表目录缺省解析到 <project>/examples/ui
    # （desktop-host 下不存在，vue.rs desktop_apps_dir 必败）——须显式注入
    # 框架 demo 主注册表（§3-a 原设计：AUTO_DESKTOP_APPS + EXTRA 两件齐注）。
    $env:AUTO_DESKTOP_APPS = Join-Path $LangRoot 'examples\ui'
    Write-Host "[desktop.ps1] track=vue  lang=$LangRoot  os=$OsRoot  auto=$autoCli"
    Write-Host "[desktop.ps1] AUTO_OS_ROOT=$($env:AUTO_OS_ROOT)  AUTO_DESKTOP_APPS=$($env:AUTO_DESKTOP_APPS)"
    Write-Host "[desktop.ps1] AUTO_DESKTOP_APPS_EXTRA=$($env:AUTO_DESKTOP_APPS_EXTRA)"
    if ($DryRun) { Write-Host "[dry-run] cd <lang>/examples/desktop-host; & auto run --desktop"; return }
    Push-Location (Join-Path $LangRoot 'examples\desktop-host')
    try { & $autoCli run --desktop }
    finally { Pop-Location }
}
else {
    $exeArgs = @()
    if ($Fullscreen) { $exeArgs += '--fullscreen' }
    Write-Host "[desktop.ps1] track=iced  lang=$LangRoot  os=$OsRoot  args=$exeArgs"
    Write-Host "[desktop.ps1] AUTO_OS_ROOT=$($env:AUTO_OS_ROOT)（apps 容器经 CWD=本仓根兄弟探测自命中）"
    # T1 实证教训：cargo 按调用方 CWD 发现 .cargo/config.toml（/STACK:32MB
    # 在 lang 仓 config）——从本仓 cargo run 丢旗标致起动栈溢出。故 lang 侧
    # build（config 生效）+ 本仓 CWD 直接运行 exe。
    if ($DryRun) {
        Write-Host "[dry-run] cd <lang>; cargo build -p auto-lang --features ui-iced --example ui_desktop"
        Write-Host "[dry-run] cd <os-root>; & <lang>/target/debug/examples/ui_desktop.exe $exeArgs"
        return
    }
    Push-Location $LangRoot
    try { cargo build -p auto-lang --features ui-iced --example ui_desktop }
    finally { Pop-Location }
    $exe = Join-Path $LangRoot 'target\debug\examples\ui_desktop.exe'
    Push-Location $OsRoot   # ../auto-os/apps 兄弟探测自命中（590 boot 实证同口径）
    try { & $exe @exeArgs }
    finally { Pop-Location }
}
