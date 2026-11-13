#!/usr/bin/env pwsh
# PLAN-067 T-02 P-05（OS 面）：FeaturePack 受管 policy 供给与书籍范围授予
# （.sh 同源孪生）。policy 契约与数据目录解析同 auto-notes repository.at
# （NOTES_FP_POLICY 覆盖 > <data>/fp-policy.json；缺席 = 全拒绝）。
# 每 app 会话凭据注入点（NOTES_FP_CRED）属 auto-ui 运行时 owner 交付面。
[CmdletBinding()]
param(
    [ValidateSet('ensure', 'add-book', 'revoke-book')]
    [string]$Command = 'ensure',
    [string]$BookId
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
$doc = if (Test-Path $policy) {
    try { Get-Content $policy -Raw -Encoding UTF8 | ConvertFrom-Json } catch { Fail "policy parse failed: $_" 4 }
} else { [pscustomobject]@{ entries = @() } }
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
if ($created -or $changed) {
    $doc | ConvertTo-Json -Depth 8 | Set-Content -Path $policy -Encoding UTF8
}
Write-Output "policy=$policy"
Write-Output "principal=$principal"
Write-Output "cred=$($entry.cred)"
Write-Output "book_scopes=$($entry.book_scopes -join ',')"
