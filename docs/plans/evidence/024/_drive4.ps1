# PLAN-024 走查驱动 v4 — TopMost + CopyFromScreen（新帧）；无 BOM 种子。
$ErrorActionPreference = 'Stop'
$ev = 'D:/autostack/.wt/os-024/auto-os/docs/plans/evidence/024'
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
public class W4 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
}
"@

function Shot([IntPtr]$h, [string]$path) {
  [W4]::SetWindowPos($h, [IntPtr](-1), 0, 0, 0, 0, 0x0001 -bor 0x0002 -bor 0x0010 -bor 0x0040) | Out-Null
  Start-Sleep -Milliseconds 700
  [W4]::SetForegroundWindow($h) | Out-Null
  Start-Sleep -Milliseconds 300
  $r = New-Object W4+RECT
  [W4]::GetWindowRect($h, [ref]$r) | Out-Null
  $wd = $r.R - $r.L; $ht = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap($wd, $ht)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Host "shot: $path"
}

# 无 BOM 种子（WinPS UTF8 带 BOM 曾破坏 JSON 解析 → storage 恒空）。
[IO.File]::WriteAllText($storage, "{`n  `"shell.dashboard.enabled`": `"012-stopwatch,025-sys-monitor`",`n  `"shell.dashboard.span.025-sys-monitor`": `"2`"`n}`n")

$env:AUTO_DASHBOARD_BOOT = '1'
$env:AUTO_OS_ROOT = $osRoot
$env:AUTO_VM_STORAGE_FILE = $storage
Set-Location $osRoot
$p = Start-Process -FilePath $exe -PassThru -WindowStyle Hidden -RedirectStandardError $log
Write-Host "pid=$($p.Id)"
Start-Sleep -Seconds 14
$p.Refresh()
$h = $p.MainWindowHandle
Write-Host "handle=$h"

Shot $h "$ev/01-panel-open.png"
Start-Sleep -Milliseconds 2500
Shot $h "$ev/02-clock-ticking.png"

# Esc 关闭仲裁
[W4]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Milliseconds 400
[System.Windows.Forms.SendKeys]::SendWait('{ESC}')
Start-Sleep -Milliseconds 900
Shot $h "$ev/04-esc-closed.png"

# 取消置顶，进程保留供检查
[W4]::SetWindowPos($h, [IntPtr](-2), 0, 0, 0, 0, 0x0001 -bor 0x0002 -bor 0x0010 -bor 0x0040) | Out-Null
Write-Host "kept-alive pid=$($p.Id)"
