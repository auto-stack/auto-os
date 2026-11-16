# PLAN-069: remove AutoIME from the current user's input method lists
# (mirror of enable-user-list.ps1; only touches AutoIME entries).
$cls = '{02E1E367-4389-429E-B13C-483EB5555D37}'
$prof = '{E8713320-7AE4-40D8-9850-AF2EA17AFD00}'
$up = 'HKCU:\Control Panel\International\User Profile\zh-Hans-CN'
if (Test-Path $up) {
  Remove-ItemProperty -Path $up -Name "0804:$cls$prof" -ErrorAction SilentlyContinue
}
$entry = "HKCU:\Software\Microsoft\CTF\SortOrder\AssemblyItem\0x00000804\{34745C63-B2F0-4784-8B67-5E12C8701A31}\00000001"
Remove-Item -Path $entry -Recurse -Force -ErrorAction SilentlyContinue
Write-Output "AutoIME removed from user input lists"
