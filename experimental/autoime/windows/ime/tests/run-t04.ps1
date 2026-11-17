# AutoIME deployment/recovery test suite (PLAN-070 T-04).
#
# Drives the real autoime_deploy.exe against a real Weasel-installed librime:
#   A  fresh user dir cold start (schema id, marker, compiled dicts, timing)
#   B  unchanged redeploy (content-addressed version, no re-copy)
#   C  engine completely unavailable -> exit 2 (keyboard pass-through path)
#   D  corrupt primary resources, no previous version -> Luna fallback
#   E  corrupt upgrade with a previous good version -> last-good rollback
#   F  resource upgrade preserves the user phrase file and learning db
#   G  two hosts cold-starting together -> single serialized deployment
#   H  ready-state key-to-candidate latency (median/P95)
#
# Usage: pwsh -NoProfile -File run-t04.ps1 [-RimeDll PATH] [-SharedDataDir DIR] [-Keep]
param(
  [string]$RimeDll = '',
  [string]$SharedDataDir = '',
  [switch]$Keep
)

$ErrorActionPreference = 'Stop'
$script:checks = 0
$script:failures = 0
$script:timings = @()

function Check([bool]$condition, [string]$name) {
  $script:checks++
  if ($condition) {
    Write-Host "PASS $name"
  } else {
    $script:failures++
    Write-Host "FAIL $name"
  }
}

function Resolve-RimePaths {
  if ($script:RimeDll -and (Test-Path $script:RimeDll)) { return }
  $root = $null
  $key = Get-ItemProperty 'HKLM:\SOFTWARE\WOW6432Node\Rime\Weasel' -ErrorAction SilentlyContinue
  if ($key -and $key.WeaselRoot) { $root = $key.WeaselRoot }
  if (-not $root) {
    $key = Get-ItemProperty 'HKLM:\SOFTWARE\Rime\Weasel' -ErrorAction SilentlyContinue
    if ($key -and $key.WeaselRoot) { $root = $key.WeaselRoot }
  }
  if (-not $root) { throw 'Weasel not found; pass -RimeDll and -SharedDataDir' }
  $script:RimeDll = Join-Path $root 'rime.dll'
  $script:SharedDataDir = Join-Path $root 'data'
}

function Invoke-Deploy {
  param(
    [string]$UserDir,
    [string]$ResourcesDir,
    [int]$Latency = 0,
    [string]$DllOverride = ''
  )
  $arguments = @('--resources-dir', $ResourcesDir, '--user-data-dir', $UserDir)
  if ($Latency -gt 0) { $arguments += @('--latency', "$Latency") }
  $saved = $env:AUTOIME_RIME_DLL
  if ($DllOverride) { $env:AUTOIME_RIME_DLL = $DllOverride }
  $watch = [System.Diagnostics.Stopwatch]::StartNew()
  $output = & $script:deploy @arguments 2>&1 | Out-String
  $code = $LASTEXITCODE
  $watch.Stop()
  $env:AUTOIME_RIME_DLL = $saved
  $json = $null
  foreach ($line in ($output -split "`r?`n")) {
    if ($line -match '^\{.*\}$') {
      try { $json = $line | ConvertFrom-Json } catch { }
    }
  }
  [pscustomobject]@{ code = $code; json = $json; output = $output; ms = $watch.ElapsedMilliseconds }
}

function New-Case([string]$name) {
  $dir = Join-Path $script:work $name
  New-Item -ItemType Directory -Force $dir | Out-Null
  return $dir
}

function Copy-Resources([string]$destination) {
  New-Item -ItemType Directory -Force $destination | Out-Null
  Copy-Item (Join-Path $script:resources '*') $destination -Recurse -Force
  return $destination
}

$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$ime = Split-Path -Parent $here
$windows = Split-Path -Parent $ime
$script:resources = Join-Path $windows 'resources'
$script:deploy = Join-Path $ime 'build\autoime_deploy.exe'
if (-not (Test-Path $script:deploy)) { throw "build the IME first: $($script:deploy) missing" }

$script:RimeDll = $RimeDll
$script:SharedDataDir = $SharedDataDir
Resolve-RimePaths
$env:AUTOIME_RIME_DLL = $script:RimeDll
$env:AUTOIME_RIME_SHARED_DATA_DIR = $script:SharedDataDir
Remove-Item Env:AUTOIME_RESOURCES_DIR -ErrorAction SilentlyContinue
Remove-Item Env:AUTOIME_RIME_USER_DATA_DIR -ErrorAction SilentlyContinue

$script:work = Join-Path $env:TEMP ("autoime-t04-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force $script:work | Out-Null

try {
  # A: fresh cold start.
  $userA = New-Case 'A'
  $a = Invoke-Deploy $userA $script:resources 30
  $script:timings += [pscustomobject]@{ case = 'A fresh'; ms = $a.ms }
  Check ($a.code -eq 0) 'A exit 0 (primary schema ready)'
  Check ($a.json -and $a.json.schema -eq 'autoime_pinyin') 'A effective schema autoime_pinyin'
  Check ($a.json -and $a.json.deploy_ms -gt 100) 'A first-deploy time recorded'
  Check (Test-Path (Join-Path $userA '.autoime-resources')) 'A version marker written'
  Check (Test-Path (Join-Path $userA 'build\autoime_terms.table.bin')) 'A terms dictionary compiled'
  Check ($a.json -and $a.json.latency_n -ge 20) 'H latency samples collected'
  Check ($a.json -and $a.json.median_us -gt 0 -and $a.json.median_us -lt 10000) 'H median key->candidate under 10ms'
  Check ($a.json -and $a.json.p95_us -ge $a.json.median_us -and $a.json.p95_us -lt 10000) 'H P95 key->candidate under 10ms'

  # B: unchanged redeploy.
  $b = Invoke-Deploy $userA $script:resources
  $script:timings += [pscustomobject]@{ case = 'B unchanged'; ms = $b.ms }
  Check ($b.code -eq 0) 'B exit 0'
  Check ($b.json -and $b.json.resources_version -eq $a.json.resources_version) 'B content-addressed version unchanged'
  Check ($b.ms -lt $a.ms) 'B redeploy does not re-copy resources'

  # C: engine unavailable.
  $userC = New-Case 'C'
  $c = Invoke-Deploy $userC $script:resources 0 'C:\bogus\rime.dll'
  $script:timings += [pscustomobject]@{ case = 'C no engine'; ms = $c.ms }
  Check ($c.code -eq 2) 'C missing engine exits 2 (pass-through)'
  Check ($c.json -and $c.json.ready -eq 0) 'C reports not ready'

  # D: corrupt primary, no previous version -> Luna fallback.
  $userD = New-Case 'D'
  $corruptD = Copy-Resources (Join-Path $script:work 'D-resources')
  Set-Content (Join-Path $corruptD 'autoime_pinyin.schema.yaml') 'schema: [broken'
  $d = Invoke-Deploy $userD $corruptD
  $script:timings += [pscustomobject]@{ case = 'D corrupt/no prior'; ms = $d.ms }
  Check ($d.code -eq 5) 'D falls back (exit 5)'
  Check ($d.json -and $d.json.fallback -eq 1) 'D reports fallback'
  Check ($d.json -and $d.json.schema -eq 'luna_pinyin_simp') 'D uses Luna schema'
  Check ($d.json -and $d.json.disabled -eq 0) 'D IME still usable'

  # E: corrupt upgrade with a previous good version -> last-good rollback.
  $userE = New-Case 'E'
  $e1 = Invoke-Deploy $userE $script:resources
  Check ($e1.code -eq 0) 'E good deploy before corrupt upgrade'
  $corruptE = Copy-Resources (Join-Path $script:work 'E-resources')
  Set-Content (Join-Path $corruptE 'autoime_pinyin.schema.yaml') 'schema: [broken'
  $e2 = Invoke-Deploy $userE $corruptE
  $script:timings += [pscustomobject]@{ case = 'E corrupt/rollback'; ms = $e2.ms }
  Check ($e2.code -eq 0) 'E recovers to a working schema (exit 0)'
  Check ($e2.json -and $e2.json.fallback -eq 0) 'E primary schema restored'
  Check ($e2.json -and $e2.json.detail -match 'restored last known-good') 'E records last-known-good rollback'

  # F: resource upgrade preserves user phrase file and learning db.
  $userF = New-Case 'F'
  $f1 = Invoke-Deploy $userF $script:resources
  Check ($f1.code -eq 0) 'F baseline deploy'
  $phraseFile = Join-Path $userF 'custom_phrase.txt'
  Add-Content -Path $phraseFile -Value "保留短语`tbaoliuduan`t1" -Encoding utf8
  Check (Test-Path (Join-Path $userF 'autoime_user.userdb')) 'F learning db created'
  $upgraded = Copy-Resources (Join-Path $script:work 'F-resources')
  Add-Content -Path (Join-Path $upgraded 'autoime_terms.dict.yaml') -Value "升级标记`tsheng ji biao ji`t500" -Encoding utf8
  $f2 = Invoke-Deploy $userF $upgraded
  $script:timings += [pscustomobject]@{ case = 'F upgrade preserves'; ms = $f2.ms }
  Check ($f2.code -eq 0) 'F upgrade deploy succeeds'
  Check ($f2.json -and $f2.json.resources_version -ne $f1.json.resources_version) 'F resource version changed'
  Check ((Get-Content $phraseFile -Raw) -match '保留短语') 'F user phrase preserved'
  Check (Test-Path (Join-Path $userF 'autoime_user.userdb')) 'F learning db preserved'

  # G: two hosts cold-starting together.
  $userG = New-Case 'G'
  $out1 = Join-Path $script:work 'G-1.out.log'
  $err1 = Join-Path $script:work 'G-1.err.log'
  $out2 = Join-Path $script:work 'G-2.out.log'
  $err2 = Join-Path $script:work 'G-2.err.log'
  $p1 = Start-Process -FilePath $script:deploy -ArgumentList @('--resources-dir', $script:resources, '--user-data-dir', $userG) -PassThru -NoNewWindow -RedirectStandardOutput $out1 -RedirectStandardError $err1
  $p2 = Start-Process -FilePath $script:deploy -ArgumentList @('--resources-dir', $script:resources, '--user-data-dir', $userG) -PassThru -NoNewWindow -RedirectStandardOutput $out2 -RedirectStandardError $err2
  $gWatch = [System.Diagnostics.Stopwatch]::StartNew()
  $p1.WaitForExit(); $p2.WaitForExit()
  $gWatch.Stop()
  $script:timings += [pscustomobject]@{ case = 'G concurrent'; ms = $gWatch.ElapsedMilliseconds }
  Check ($p1.ExitCode -eq 0 -and $p2.ExitCode -eq 0) 'G both cold starts succeed'
  Check (Test-Path (Join-Path $userG '.autoime-resources')) 'G shared deployment marker written'
  Check (Test-Path (Join-Path $userG 'build\autoime_pinyin.schema.yaml')) 'G schema compiled'
  Check (-not (Test-Path (Join-Path $userG '.autoime-staging'))) 'G staging cleaned'
  Check (-not (Test-Path (Join-Path $userG '.autoime-backup'))) 'G backup cleaned after commit'
} finally {
  if (-not $Keep) {
    Remove-Item -Recurse -Force $script:work -ErrorAction SilentlyContinue
  } else {
    Write-Host "work dir kept: $script:work"
  }
}

$totalMs = ($script:timings | Measure-Object -Property ms -Sum).Sum
Write-Host 'TIMING (per case):'
foreach ($t in $script:timings) { Write-Host ("  {0,-22} {1} ms" -f $t.case, $t.ms) }
Write-Host ("  {0,-22} {1} ms" -f 'total', $totalMs)
$summary = [pscustomobject]@{ suite = 't04'; checks = $script:checks; failures = $script:failures; total_ms = $totalMs }
Write-Host ("SUMMARY " + ($summary | ConvertTo-Json -Compress))
exit ($(if ($script:failures -eq 0) { 0 } else { 5 }))
