# PLAN-024 实机走查驱动 v2（遮挡无关截图 = PrintWindow；前台重试点击）。
$ErrorActionPreference = 'Stop'
$ev = Split-Path -Parent $MyInvocation.MyCommand.Path
$osRoot = 'D:/autostack/.wt/os-024/auto-os'
$langRoot = 'D:/autostack/.wt/os-024/auto-lang'
$exe = "$langRoot/target/debug/examples/ui_desktop.exe"
$storage = "$ev/_storage-test.json"
$log = "$ev/ui_desktop.err.log"

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class W2 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, UIntPtr e);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr dc, uint flags);
}
"@

function Shot([IntPtr]$h, [string]$path) {
  $r = New-Object W2+RECT
  [W2]::GetWindowRect($h, [ref]$r) | Out-Null
  $wd = $r.R - $r.L; $ht = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap($wd, $ht)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $dc = $g.GetHdc()
  # PW_RENDERFULLCONTENT = 2 —— GPU 合成窗内容直抓（遮挡无关）。
  [W2]::PrintWindow($h, $dc, 2) | Out-Null
  $g.ReleaseHdc($dc)
  $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Host "shot: $path (${wd}x${ht})"
}

function FocusWin([IntPtr]$h) {
  for ($i = 0; $i -lt 5; $i++) {
    [W2]::SetForegroundWindow($h) | Out-Null
    Start-Sleep -Milliseconds 300
    if ([System.Windows.Forms.Form]::ActiveForm -ne $null) { break }
  }
}

function ClickAt([IntPtr]$h, [int]$cx, [int]$cy) {
  $pt = New-Object W2+POINT; $pt.X = $cx; $pt.Y = $cy
  [W2]::ClientToScreen($h, [ref]$pt) | Out-Null
  FocusWin $h
  [W2]::SetCursorPos($pt.X, $pt.Y) | Out-Null
  Start-Sleep -Milliseconds 200
  [W2]::mouse_event(0x02, 0, 0, 0, [UIntPtr]::Zero)
  [W2]::mouse_event(0x04, 0, 0, 0, [UIntPtr]::Zero)
  Write-Host "click: client($cx,$cy) -> screen($($pt.X),$($pt.Y))"
}

@'
{
  "shell.dashboard.enabled": "012-stopwatch,025-sys-monitor",
  "shell.dashboard.span.025-sys-monitor": "2"
}
'@ | Set-Content -Path $storage -Encoding UTF8

$env:AUTO_DASHBOARD_BOOT = '1'
$env:AUTO_OS_ROOT = $osRoot
$env:AUTO_VM_STORAGE_FILE = $storage
Set-Location $osRoot
$p = Start-Process -FilePath $exe -PassThru -WindowStyle Hidden `
     -RedirectStandardError $log
Write-Host "desktop pid=$($p.Id)"
Start-Sleep -Seconds 14
$p.Refresh()
$h = $p.MainWindowHandle
if ($h -eq [IntPtr]::Zero) { throw "no main window (see $log)" }
FocusWin $h

$c = New-Object W2+RECT
[W2]::GetClientRect($h, [ref]$c) | Out-Null
$vw = $c.R; $vh = $c.B
Write-Host "client ${vw}x${vh}"

# 01/02: 面板展开 + clock 走秒（间隔截图，秒位变化）
Shot $h "$ev/01-panel-open.png"
Start-Sleep -Milliseconds 1500
Shot $h "$ev/02-clock-ticking.png"

# 03: 占位卡点击 → launch → sys-monitor 窗 + 活面
$panelW = [Math]::Min(920, $vw - 32)
$cellW = ($panelW - 32 - 24) / 3.0
$x0 = [int](($vw - $panelW) / 2 + 16)
$ysm = [int](64 + 48 + 16 + 66)
$xsm = [int]($x0 + $cellW + 12 + ($cellW * 2 + 12) / 2)
ClickAt $h $xsm $ysm
Write-Host "waiting for sys-monitor launch/build..."
Start-Sleep -Seconds 12
Shot $h "$ev/03-sysmon-launched-live-face.png"

# 04: Esc 关面板（桌面恢复；sys-monitor 窗保留）
FocusWin $h
Start-Sleep -Milliseconds 300
[System.Windows.Forms.SendKeys]::SendWait('{ESC}')
Start-Sleep -Milliseconds 800
Shot $h "$ev/04-esc-closed.png"

Write-Host "process left RUNNING for MCP inspection (pid=$($p.Id)); kill manually or via _kill.ps1"
