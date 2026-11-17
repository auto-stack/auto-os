@echo off
rem AutoIME TSF uninstall (PLAN-069 T-02, PLAN-070 T-04).
rem Requires an elevated prompt. Only removes AutoIME's own registrations and
rem program files. User data (phrases + learning) is kept unless --purge is
rem given, so a reinstall restores the user's dictionary.
setlocal
set "DST=%ProgramFiles%\AutoIME"
net session >nul 2>&1
if errorlevel 1 (
  echo [uninstall] ERROR: not elevated. Run from an admin prompt.
  exit /b 2
)
if exist "%DST%\autoime_ime.dll" regsvr32 /u /s "%DST%\autoime_ime.dll"
if errorlevel 1 echo [uninstall] WARN: regsvr32 /u returned %errorlevel%
del "%DST%\autoime_ime.dll" 2>nul
del "%DST%\autoime_ime_old_*.dll" 2>nul

rem Remove the prototype's plaintext diagnostic log if it still exists (before
rem the deployment tool itself is deleted).
if exist "%DST%\autoime_deploy.exe" (
  "%DST%\autoime_deploy.exe" --cleanup-legacy-log
) else (
  del "%TEMP%\autoime-ime.log" 2>nul
)
del "%DST%\autoime_deploy.exe" 2>nul
del "%DST%\verify_registered.exe" 2>nul
if exist "%DST%\resources" rmdir /s /q "%DST%\resources"
rmdir "%DST%" 2>nul

if /i "%~1"=="--purge" (
  echo [uninstall] removing user data ^(phrases + learning^) at "%LOCALAPPDATA%\AutoIME"
  rmdir /s /q "%LOCALAPPDATA%\AutoIME" 2>nul
) else (
  echo [uninstall] user data kept at "%LOCALAPPDATA%\AutoIME\rime"
  echo [uninstall] pass --purge to remove it as well
)

rem Remove AutoIME from the current user's switcher lists (own entries only).
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0disable-user-list.ps1"
echo [uninstall] OK: AutoIME registration removed
endlocal
