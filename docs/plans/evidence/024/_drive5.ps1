# PLAN-024 常驻版自检 — 截图后窗口保留给用户。
$ErrorActionPreference = 'Stop'
$ev = 'D:/autostack/.wt/os-024/auto-os/docs/plans/evidence/024'
$osRoot = 'D:/autostack/.wt/os-024/auto-os'
$langRoot = 'D:/autostack/.wt/os-024/auto-lang'
$exe = "$langRoot/target/debug/examples/ui_desktop.exe"
$log = "$ev/ui_desktop.err.log"

Add-Type -AssemblyName System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class W5 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
}
"@

function Shot([IntPtr]$h, [string]$path) {
  [W5]::SetWindowPos($h, [IntPtr](-1), 0, 0, 0, 0, 0x0001 -bor 0x0002) | Out-Null
  Start-Sleep -Milliseconds 800
  [W5]::SetForegroundWindow($h) | Out-Null
  Start-Sleep -Milliseconds 300
  $r = New-Object W5+RECT
  [W5]::GetWindowRect($h, [ref]$r) | Out-Null
  $wd = $r.R - $r.L; $ht = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap($wd, $ht)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Host "shot: $path"
}

$env:AUTO_OS_ROOT = $osRoot
Set-Location $osRoot
$p = Start-Process -FilePath $exe -PassThru -WindowStyle Hidden -RedirectStandardError $log
Write-Host "pid=$($p.Id)"
Start-Sleep -Seconds 14
$p.Refresh()
$h = $p.MainWindowHandle
Write-Host "handle=$h"
Shot $h "$ev/05-resident-bottom.png"
[W5]::SetWindowPos($h, [IntPtr](-2), 0, 0, 0, 0, 0x0001 -bor 0x0002) | Out-Null
Write-Host "kept-alive pid=$($p.Id)"
