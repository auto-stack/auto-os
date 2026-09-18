# PLAN-024 走查驱动 v3 — 截图后保持进程存活供 MCP 检查。
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
public class W3 {
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
  $r = New-Object W3+RECT
  [W3]::GetWindowRect($h, [ref]$r) | Out-Null
  $wd = $r.R - $r.L; $ht = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap($wd, $ht)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $dc = $g.GetHdc()
  [W3]::PrintWindow($h, $dc, 2) | Out-Null
  $g.ReleaseHdc($dc)
  $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Host "shot: $path"
}

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
[W3]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Seconds 1
Shot $h "$ev/01-panel-open.png"
Start-Sleep -Milliseconds 1800
Shot $h "$ev/02-clock-ticking.png"
Write-Host "kept-alive pid=$($p.Id)"
