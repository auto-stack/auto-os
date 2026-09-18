# PLAN-024 实机走查驱动（无人值守证据采集；walkthrough §9.1 自动化半）。
# 用法：powershell -ExecutionPolicy Bypass -File _drive.ps1
# 产出（本目录）：01-panel-open.png / 02-clock-ticking.png /
#   03-sysmon-launched-live-face.png / 04-esc-closed.png / ui_desktop.err.log
# 场景：种子 storage（enabled=012-stopwatch,025-sys-monitor；sysmon span=2）
#   + AUTO_DASHBOARD_BOOT=1 → 面板 boot 召唤 → clock 孵化走秒（两张间隔
#   截图秒位变化）→ 点 sys-monitor 占位卡（宿主 launch 链）→ 活面数值
#   （inproc 合并 VM 真数据）→ Esc 关闭面板、桌面恢复。

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
public class W {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint dx, uint dy, uint d, UIntPtr e);
}
"@

function Shot([IntPtr]$h, [string]$path) {
  $r = New-Object W+RECT
  [W]::GetWindowRect($h, [ref]$r) | Out-Null
  $wd = $r.R - $r.L; $ht = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap($wd, $ht)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Host "shot: $path (${wd}x${ht})"
}

function ClickAt([IntPtr]$h, [int]$cx, [int]$cy) {
  $pt = New-Object W+POINT; $pt.X = $cx; $pt.Y = $cy
  [W]::ClientToScreen($h, [ref]$pt) | Out-Null
  [W]::SetForegroundWindow($h) | Out-Null
  Start-Sleep -Milliseconds 200
  [W]::SetCursorPos($pt.X, $pt.Y) | Out-Null
  Start-Sleep -Milliseconds 150
  [W]::mouse_event(0x02, 0, 0, 0, [UIntPtr]::Zero)  # LEFTDOWN
  [W]::mouse_event(0x04, 0, 0, 0, [UIntPtr]::Zero)  # LEFTUP
  Write-Host "click: client($cx,$cy) -> screen($($pt.X),$($pt.Y))"
}

# ---- 种子 storage（隔离临时文件，不动用户桌面库）----
@'
{
  "shell.dashboard.enabled": "012-stopwatch,025-sys-monitor",
  "shell.dashboard.span.025-sys-monitor": "2"
}
'@ | Set-Content -Path $storage -Encoding UTF8

# ---- 启动桌面（boot 召唤面板）----
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
[W]::SetForegroundWindow($h) | Out-Null

# ---- AC-02/AC-06a：面板展开 + clock 孵化走秒（两张间隔截图）----
$c = New-Object W+RECT
[W]::GetClientRect($h, [ref]$c) | Out-Null
$vw = $c.R; $vh = $c.B
Write-Host "client ${vw}x${vh}"
Shot $h "$ev/01-panel-open.png"
Start-Sleep -Milliseconds 1500
Shot $h "$ev/02-clock-ticking.png"

# ---- AC-04：占位卡点击 → dashboard_launch → sys-monitor 活面 ----
# 布局算式镜像（dashboard_layout）：panel_w=min(920,vw-32)，3 列，
#   cell_w=(panel_w-32-24)/3；行0 = clock(span1) + sysmon(span2)。
#   格位 x0 = (vw-panel_w)/2+16；sysmon x = x0+cell_w+12；y = 64+48+16。
$panelW = [Math]::Min(920, $vw - 32)
$cellW = ($panelW - 32 - 24) / 3.0
$x0 = [int](($vw - $panelW) / 2 + 16)
$ysm = [int](64 + 48 + 16 + 66)   # 行0 中心
$xsm = [int]($x0 + $cellW + 12 + ($cellW * 2 + 12) / 2)
ClickAt $h $xsm $ysm
Write-Host "waiting for sys-monitor launch/build..."
Start-Sleep -Seconds 10
Shot $h "$ev/03-sysmon-launched-live-face.png"

# ---- AC-02：Esc 关闭面板，桌面恢复（sys-monitor 窗保留）----
[W]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Milliseconds 300
[System.Windows.Forms.SendKeys]::SendWait('{ESC}')
Start-Sleep -Milliseconds 800
Shot $h "$ev/04-esc-closed.png"

Stop-Process -Id $p.Id -Force
Write-Host "done."
