# AutoIME offline installer - bundled engine staging (PLAN-071 T-02).
#
# Verifies every file in bundle-manifest.json against its locked SHA-256 and
# byte size, then copies it into the staging tree the Inno Setup definition
# packs. Any mismatch is a named preparation failure: no package is produced.
#
# Usage: pwsh -NoProfile -File stage-engine.ps1 [-Stage <staging dir>]
param(
  [Parameter(Mandatory = $true)][string]$Stage
)

$ErrorActionPreference = 'Stop'

$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifestPath = Join-Path $here 'bundle-manifest.json'
$manifest = Get-Content $manifestPath -Raw -Encoding UTF8 | ConvertFrom-Json

# Source root: env override -> manifest default -> WeaselRoot registry.
$root = $env:AUTOIME_WEASEL_ROOT
if (-not $root -or -not (Test-Path $root)) { $root = $manifest.source.default_root }
if (-not (Test-Path $root)) {
  foreach ($hive in 'HKLM:\SOFTWARE\WOW6432Node\Rime\Weasel', 'HKLM:\SOFTWARE\Rime\Weasel') {
    $key = Get-ItemProperty $hive -ErrorAction SilentlyContinue
    if ($key -and $key.WeaselRoot) { $root = $key.WeaselRoot; break }
  }
}
if (-not $root -or -not (Test-Path $root)) {
  throw ("[stage-engine] preparation failed: Weasel root not found " +
         "(env AUTOIME_WEASEL_ROOT, manifest default, or registry WeaselRoot)")
}
Write-Host "[stage-engine] source root: $root"

if (-not (Test-Path $Stage)) { New-Item -ItemType Directory -Force $Stage | Out-Null }
foreach ($file in $manifest.files) {
  $src = Join-Path $root ($file.path -replace '/', '\')
  if (-not (Test-Path $src)) {
    throw "[stage-engine] preparation failed: missing file $($file.path) (source $src)"
  }
  $item = Get-Item $src
  if ($item.Length -ne $file.bytes) {
    throw ("[stage-engine] preparation failed: $($file.path) size " +
           "$($item.Length) != locked $($file.bytes)")
  }
  $hash = (Get-FileHash $src -Algorithm SHA256).Hash
  if ($hash -ne $file.sha256) {
    throw ("[stage-engine] preparation failed: $($file.path) SHA-256 " +
           "$hash != locked $($file.sha256) - the source distribution does " +
           "not match the registered bundle; refresh the manifest from a " +
           "verified Weasel 0.17.4 install first")
  }
  $dest = Join-Path $Stage ($file.dest -replace '/', '\')
  $destDir = Split-Path -Parent $dest
  if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Force $destDir | Out-Null }
  Copy-Item $src $dest -Force
  Write-Host ("[stage-engine] OK {0} ({1} bytes, sha256 verified)" -f $file.dest, $item.Length)
}
Write-Host ("[stage-engine] staged {0} locked files from {1}" -f
            @($manifest.files).Count, $root)
