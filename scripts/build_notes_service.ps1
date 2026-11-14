#!/usr/bin/env pwsh
# PLAN-067 T-02 A-01/A-03：Notes FeaturePack 服务可执行件发布入口
# （.sh 同源孪生，参数与失败规则一致）。
# 严格产物绑定：成员名取自本构建日志，exe 取自该成员在本缓存根的产物且
# mtime 晚于构建起点；无任何跨根/全局最新回退——构建失败即发布失败。
# 回执绑定 source/工具/构建开关/支持端点与 SHA。只回收自有 PID；
# 按路径清孤儿仅限显式 -TargetRoot（私有根）。
[CmdletBinding()]
param(
    [string]$NotesRoot,
    [string]$Port = '17919',
    [int]$WaitSecs = 600,
    [string]$TargetRoot
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

$explicitTargetRoot = [bool]$TargetRoot
if (-not $TargetRoot) { $TargetRoot = Join-Path $langRoot 'target/generated-consumers' }
New-Item -ItemType Directory -Force -Path $TargetRoot | Out-Null
$TargetRoot = (Resolve-Path $TargetRoot).Path.Replace('\', '/')

$notesSha = (& git -C $notes rev-parse HEAD).Trim()
$toolRepo = Split-Path -Parent (Split-Path -Parent $autoBin)
$toolSha = (& git -C $toolRepo rev-parse HEAD 2>$null)
if ($LASTEXITCODE -ne 0) { $toolSha = 'unknown' }
$buildStart = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
Write-Output "[publish] notes=$notes"
Write-Output "[publish] notes HEAD=$notesSha"
Write-Output "[publish] auto=$autoBin"
Write-Output "[publish] tool HEAD=$toolSha"
Write-Output "[publish] target-root=$TargetRoot"
Write-Output "[publish] build port=$Port"

# 生成 + 构建（等待窗 -WaitSecs，缺省 600s 开发 profile 预算）。
# 显式 -TargetRoot 经 AUTO_GENERATED_TARGET_ROOT 钉住构建落点；缺省不设。
# AUTO_ALLOW_PARTIAL_DB=1：asset_upload（NOTES-002/PLAN-730）无 db.rs 匹配，
# notes 仓自有 battery 同款 env 接受面；fp v1 端点全部有 db.rs 匹配。
$env:AUTO_ALLOW_PARTIAL_DB = '1'
if ($explicitTargetRoot) { $env:AUTO_GENERATED_TARGET_ROOT = $TargetRoot }
else { Remove-Item Env:AUTO_GENERATED_TARGET_ROOT -ErrorAction SilentlyContinue }
$proc = Start-Process -FilePath $autoBin -ArgumentList @('service', '--server', 'rust', '-B', $Port) `
    -WorkingDirectory $notes -RedirectStandardOutput $log -RedirectStandardError "$log.err" `
    -PassThru -WindowStyle Hidden
$ready = $false
$deadline = (Get-Date).AddSeconds($WaitSecs)
while ((Get-Date) -lt $deadline) {
    if ((Test-Path $log) -and (Select-String -Path $log -Pattern 'AUTO_SERVICE_READY' -Quiet)) { $ready = $true; break }
    if ($proc.HasExited) { break }
    Start-Sleep -Seconds 2
}
if (-not $proc.HasExited) {
    & taskkill /T /F /PID $proc.Id 2>$null | Out-Null
}
if ($explicitTargetRoot) {
    Get-CimInstance Win32_Process -Filter "Name like 'auto-notes-%-back.exe'" |
        Where-Object { $_.CommandLine -like "$TargetRoot*" } |
        ForEach-Object { & taskkill /T /F /PID $_.ProcessId 2>$null | Out-Null }
}
Start-Sleep -Seconds 1
if (-not $ready) {
    # A-01 反例守门：构建未就绪 = 发布失败，绝不回退定位任何既有产物。
    Fail "auto service 未在 ${WaitSecs}s 内就绪（log: $log）" 3
}

# A-01 严格绑定：成员名取自本构建日志的生成行。
$member = $null
foreach ($line in (Get-Content $log -Encoding UTF8)) {
    if ($line -match 'Generated Rust server: ([A-Za-z0-9._-]+)/') { $member = $Matches[1] }
}
if (-not $member) { Fail "构建日志缺 'Generated Rust server: <member>/' 行（log: $log）" 4 }

# 产物定位限本缓存根（显式根 release/；受管根 <ctx>/release/），要求唯一命中。
$cands = @(Get-ChildItem -Path $TargetRoot -Filter "$member.exe" -Recurse -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName.Replace('\', '/') -match '/release/[^/]+\.exe$' })
if ($cands.Count -ne 1) {
    Fail "本构建成员产物定位非唯一或缺席（member=$member, root=$TargetRoot, n=$($cands.Count)；log: $log）" 4
}
$exe = $cands[0].FullName
# 绑定门 = READY + 成员名出自本日志 + 根内唯一定位；mtime 作关联记录
# （无变更增量重建不重链——早于起点须 cargo Finished 行佐证，否则拒绝）。
$exeMtime = [long](Get-Item $exe).LastWriteTimeUtc.Subtract([DateTime]'1970-01-01').TotalSeconds
$finishedInLog = $false
foreach ($lf in @($log, "$log.err")) {
    if ((Test-Path $lf) -and (Select-String -Path $lf -Pattern 'Finished' -Quiet)) { $finishedInLog = $true }
}
if ($exeMtime -lt $buildStart -and -not $finishedInLog) {
    Fail "产物 mtime 早于构建起点且日志无 cargo Finished——绑定证据不足，拒绝发布（A-01）" 5
}
$buildCorrelation = if ($exeMtime -ge $buildStart) {
    "fresh-build (mtime $exeMtime >= start $buildStart)"
} else {
    "no-change-incremental-rebuild (mtime $exeMtime < start $buildStart; cargo Finished in log)"
}

$pubDir = Join-Path $notes 'target/release'
New-Item -ItemType Directory -Force -Path $pubDir | Out-Null
$pub = Join-Path $pubDir 'notes-service.exe'
Copy-Item -Force $exe $pub
$exeSha = (Get-FileHash -Algorithm SHA256 $exe).Hash.ToLower()
$pubSha = (Get-FileHash -Algorithm SHA256 $pub).Hash.ToLower()
Write-Output "[publish] member=$member"
Write-Output "[publish] build correlation=$buildCorrelation"
Write-Output "[publish] source exe=$exe"
Write-Output "[publish] source sha256=$exeSha"
Write-Output "[publish] published=$pub"
Write-Output "[publish] published sha256=$pubSha"
Write-Output "[publish] build switches: AUTO_ALLOW_PARTIAL_DB=1 (asset_upload=NOTES-002/PLAN-730 mixed-state accepted per notes battery; fp v1 endpoints all db.rs-matched)"
Write-Output "[publish] supported fp surface: GET /api/health (notes.fpack/1); POST /api/fp/v1/{capture,get,list,update,receipt}; POST /api/fp/draft/{put,get} (main-app scratch, no policy gate)"
Write-Output "[publish] service profile: media_scan/photo_scan off (auto service default); NOT covered by this receipt: asset_upload mixed-state writes"
Write-Output "[publish] receipt: notes HEAD=$notesSha member=$member tool HEAD=$toolSha exe=$pub sha256=$pubSha"
