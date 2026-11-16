# PLAN-069: append AutoIME to the current user's input method lists (the
# same entries Windows Settings writes when adding a keyboard). Required for
# the Win+Space switcher to offer AutoIME in the live session.
$cls = '{02E1E367-4389-429E-B13C-483EB5555D37}'
$prof = '{E8713320-7AE4-40D8-9850-AF2EA17AFD00}'
$up = 'HKCU:\Control Panel\International\User Profile\zh-Hans-CN'
if (-not (Test-Path $up)) { Write-Output "zh-Hans-CN language not present"; exit 1 }
Set-ItemProperty -Path $up -Name "0804:$cls$prof" -Type DWord -Value 1
$base = "HKCU:\Software\Microsoft\CTF\SortOrder\AssemblyItem\0x00000804\{34745C63-B2F0-4784-8B67-5E12C8701A31}"
New-Item -Path "$base\00000001" -Force | Out-Null
Set-ItemProperty -Path "$base\00000001" -Name CLSID -Value $cls
Set-ItemProperty -Path "$base\00000001" -Name Profile -Value $prof
Set-ItemProperty -Path "$base\00000001" -Name KeyboardLayout -Type DWord -Value 0
Write-Output "AutoIME appended to user input lists"
