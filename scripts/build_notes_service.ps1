#!/usr/bin/env pwsh
# PLAN-067 T-02 P-01：Notes FeaturePack 服务可执行件发布入口（.sh 同源孪生）。
# 经 auto service 生成链构建生成后端，发布为 <notes>/target/release/
# notes-service.exe 稳定命名件（生产 manifest daemon.bin 路径语义消费面；
# 生成件名含路径哈希 id8 不能进 manifest）。只回收自有 PID。
[CmdletBinding()]
param(
    [string]$NotesRoot,
    [string]$Port = '17919'
)

$ErrorActionPreference = 'Stop'
function Fail([string]$Message, [int]$Code = 1) {
    Write-Error "build_notes_service: $Message"
    exit $Code
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$osRoot = Resolve-Path (Join-Path $scriptDir '..')
$parent = Split-Path -Parent $osRoot

# 解析序（AGENTS §2 红线）：显式参数/env → 兄弟检出 → 主检出；永不建链接。
function Resolve-Root([string]$Explicit, [string]$EnvName, [string]$Repo, [string]$Probe, [string]$What) {
    $candidates = @(
        $Explicit,
        [Environment]::GetEnvironmentVariable($EnvName, 'Process'),
        (Join-Path $parent $Repo),
        "D:/autostack/$Repo"
    ) | Where-Object { $_ }
    foreach ($c in $candidates) {
        if (Test-Path -LiteralPath (Join-Path $c $Probe) -PathType Leaf) {
            return (Resolve-Path -LiteralPath $c).Path
        }
    }
    Fail "$What root not found ($EnvName -> sibling -> main checkout)" 2
}

$notes = if ($NotesRoot) {
    if (-not (Test-Path (Join-Path $NotesRoot 'pac.at'))) { Fail "notes root invalid: $NotesRoot" }
    (Resolve-Path $NotesRoot).Path
} else { Resolve-Root $null 'AUTO_NOTES_ROOT' 'auto-notes' 'pac.at' 'notes' }
$autoBin = Join-Path (Resolve-Root $null 'AUTO_MAN_ROOT' 'auto-man' 'target/debug/auto.exe' 'auto-man') 'target/debug/auto.exe'
$langRoot = Resolve-Root $null 'AUTO_LANG_ROOT' 'auto-lang' 'crates/auto-lang/Cargo.toml' 'auto-lang'

$artHome = if ($env:PLAN_ARTIFACTS_HOME) { $env:PLAN_ARTIFACTS_HOME } else { 'D:/autostack/plan-artifacts/plan-067' }
New-Item -ItemType Directory -Force -Path $artHome | Out-Null
$log = Join-Path $artHome ("build-notes-service-{0:yyyyMMdd-HHmmss}.log" -f (Get-Date))

$notesSha = (& git -C $notes rev-parse HEAD).Trim()
Write-Output "[publish] notes=$notes"
Write-Output "[publish] notes HEAD=$notesSha"
Write-Output "[publish] auto=$autoBin"
Write-Output "[publish] build port=$Port"

# 生成 + 构建（生成链 = 唯一经生产验证的产物来源）；等待窗为开发 profile 预算（600s）。
# AUTO_ALLOW_PARTIAL_DB=1：asset_upload（NOTES-002/PLAN-730）无 db.rs 匹配，
# notes 仓自有 battery 同款 env 接受面；fp v1 端点全部有 db.rs 匹配。
$env:AUTO_ALLOW_PARTIAL_DB = '1'
$proc = Start-Process -FilePath $autoBin -ArgumentList @('service', '--server', 'rust', '-B', $Port) `
    -WorkingDirectory $notes -RedirectStandardOutput $log -RedirectStandardError "$log.err" `
    -PassThru -WindowStyle Hidden
$ready = $false
$deadline = (Get-Date).AddSeconds(600)
while ((Get-Date) -lt $deadline) {
    if ((Test-Path $log) -and (Select-String -Path $log -Pattern 'AUTO_SERVICE_READY' -Quiet)) { $ready = $true; break }
    if ($proc.HasExited) { break }
    Start-Sleep -Seconds 2
}
if (-not $proc.HasExited) {
    & taskkill /T /F /PID $proc.Id 2>$null | Out-Null
}
Start-Sleep -Seconds 1
if (-not $ready) {
    Fail "auto service 未在 600s 内就绪（log: $log）" 3
}

$exe = Get-ChildItem -Path (Join-Path $langRoot 'target/generated-consumers/*/release/auto-notes-*-back.exe') |
    Sort-Object LastWriteTime -Descending | Select-Object -First 1
if (-not $exe) { Fail "生成后端 exe 未定位（$langRoot/target/generated-consumers/*/release）" 4 }

$pubDir = Join-Path $notes 'target/release'
New-Item -ItemType Directory -Force -Path $pubDir | Out-Null
$pub = Join-Path $pubDir 'notes-service.exe'
Copy-Item -Force $exe.FullName $pub
$exeSha = (Get-FileHash -Algorithm SHA256 $exe.FullName).Hash.ToLower()
$pubSha = (Get-FileHash -Algorithm SHA256 $pub).Hash.ToLower()
Write-Output "[publish] source exe=$($exe.FullName)"
Write-Output "[publish] source sha256=$exeSha"
Write-Output "[publish] published=$pub"
Write-Output "[publish] published sha256=$pubSha"
Write-Output "[publish] receipt: notes HEAD=$notesSha exe=$pub sha256=$pubSha"
