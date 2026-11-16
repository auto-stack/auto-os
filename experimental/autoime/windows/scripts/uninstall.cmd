@echo off
rem AutoIME TSF uninstall (PLAN-069 T-02). Requires an elevated prompt.
rem Only removes AutoIME's own registrations and install directory.
setlocal
set "DST=%ProgramFiles%\AutoIME"
net session >nul 2>&1
if errorlevel 1 (
  echo [uninstall] ERROR: not elevated. Run from an admin prompt.
  exit /b 2
)
regsvr32 /u /s "%DST%\autoime_ime.dll"
if errorlevel 1 echo [uninstall] WARN: regsvr32 /u returned %errorlevel%
del "%DST%\autoime_ime.dll" 2>nul
rmdir "%DST%" 2>nul
rem remove AutoIME from the current user's switcher lists (own entries only)
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0disable-user-list.ps1"
echo [uninstall] OK: AutoIME registration removed
endlocal
