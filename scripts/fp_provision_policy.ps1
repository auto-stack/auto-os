#!/usr/bin/env pwsh
# PLAN-067 T-02 A-02：FeaturePack 受管 policy 供给与书籍范围授予
# （.sh 同源孪生，私有凭据面）。默认输出不含凭据（只出 cred 指纹）；
# -RevealCred 显式完整回显（私有传递窗口，stdout 不得归档）。
# policy 契约与数据目录解析同 auto-notes repository.at；消费面 =
# manifest daemon.host.credential（运行时注入，本脚本 stdout 不参与生产传递）。
[CmdletBinding()]
param(
    [ValidateSet('ensure', 'add-book', 'revoke-book')]
    [string]$Command = 'ensure',
    [string]$BookId,
    [switch]$RevealCred
)

$ErrorActionPreference = 'Stop'
function Fail([string]$Message, [int]$Code = 1) {
    Write-Error "fp_provision_policy: $Message"
    exit $Code
}
if ($Command -in 'add-book', 'revoke-book' -and [string]::IsNullOrWhiteSpace($BookId)) {
    Fail "missing BOOK_ID for $Command" 2
}

$policy = if ($env:NOTES_FP_POLICY) { $env:NOTES_FP_POLICY }
elseif ($env:NOTES_DATA_DIR) { Join-Path $env:NOTES_DATA_DIR 'fp-policy.json' }
else {
    $base = if ($env:APPDATA) { $env:APPDATA }
        elseif ($env:XDG_DATA_HOME) { $env:XDG_DATA_HOME }
        else { Join-Path $env:HOME '.local/share' }
    Join-Path $base 'auto-notes/data/fp-policy.json'
}
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $policy) | Out-Null

$principal = 'fp-reader'
$operations = @('capture', 'read', 'update', 'receipt', 'list_by_book')
$hadBom = $false
$doc = $null
if (Test-Path $policy) {
    $raw = [System.IO.File]::ReadAllBytes($policy)
    $hadBom = ($raw.Length -ge 3 -and $raw[0] -eq 0xEF -and $raw[1] -eq 0xBB -and $raw[2] -eq 0xBF)
    try { $doc = Get-Content $policy -Raw -Encoding UTF8 | ConvertFrom-Json } catch { Fail "policy parse failed: $_" 4 }
}
if (-not $doc) { $doc = [pscustomobject]@{ entries = @() } }
if (-not $doc.entries) { $doc | Add-Member -Force -NotePropertyName entries -NotePropertyValue @() }
if (-not $doc.entries) { $doc | Add-Member -Force -NotePropertyName entries -NotePropertyValue @() }

$entry = @($doc.entries) | Where-Object { $_.cred -and $_.principal -eq $principal } | Select-Object -First 1
$created = $false
if (-not $entry) {
    # 256-bit hex 凭据（CSPRNG；契约冻结 cred 形态 64 hex）。
    $bytes = [byte[]]::new(32)
    [System.Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
    $cred = ($bytes | ForEach-Object { $_.ToString('x2') }) -join ''
    $entry = [pscustomobject]@{
        cred = $cred
        principal = $principal
        operations = $operations
        book_scopes = @()
    }
    $doc.entries = @($doc.entries) + $entry
    $created = $true
}
$changed = $false
$scope = "book:$BookId"
if ($Command -eq 'add-book' -and $entry.book_scopes -notcontains $scope) {
    $entry.book_scopes = @($entry.book_scopes) + $scope
    $changed = $true
}
if ($Command -eq 'revoke-book' -and $entry.book_scopes -contains $scope) {
    $entry.book_scopes = @($entry.book_scopes | Where-Object { $_ -ne $scope })
    $changed = $true
}
# 历史 BOM 文件强制重写归一（运行时/服务侧均不容忍 BOM）。
if ($created -or $changed -or $hadBom) {
    # UTF-8 **无 BOM**（服务侧 fs_read 与运行时 serde_json 均不容忍 BOM；
    # Windows PowerShell 5.1 的 -Encoding UTF8 默认带 BOM，必须显式无 BOM 写）。
    $json = $doc | ConvertTo-Json -Depth 8
    [System.IO.File]::WriteAllText($policy, $json + "`n", [System.Text.UTF8Encoding]::new($false))
}
# 私有凭据面（A-02）：默认只出指纹；-RevealCred 显式完整回显。
$fp = ([System.Security.Cryptography.SHA256]::Create().ComputeHash(
    [Text.Encoding]::UTF8.GetBytes($entry.cred)) | ForEach-Object { $_.ToString('x2') }) -join ''
Write-Output "policy=$policy"
Write-Output "principal=$principal"
Write-Output "cred_fingerprint=sha256:$($fp.Substring(0, 12)) (first 12 hex; credential itself is NOT printed)"
Write-Output "book_scopes=$($entry.book_scopes -join ',')"
if ($RevealCred) {
    Write-Output "cred=$($entry.cred)  # private delivery window only; do not archive stdout"
}
