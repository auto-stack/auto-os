@echo off
rem AutoIME TSF install (PLAN-069 T-02, PLAN-070 T-04).
rem Requires an elevated prompt (HKLM write). Installs the DLL, the Rime
rem resources and the deployment tool, pre-compiles the dictionaries (so the
rem first key never waits), registers the TSF profile and cleans the prototype's
rem plaintext diagnostic log.
rem Usage: install.cmd [path\to\autoime_ime.dll]
setlocal EnableDelayedExpansion
set "BUILD=%~dp0..\ime\build"
set "SRC=%~dp0..\ime\build\autoime_ime.dll"
if not "%~1"=="" set "SRC=%~1"
if not exist "%SRC%" (
  echo [install] ERROR: dll not found: %SRC% ^(build ime/ first^)
  exit /b 1
)
set "DST=%ProgramFiles%\AutoIME"
net session >nul 2>&1
if errorlevel 1 (
  echo [install] ERROR: not elevated. Run from an admin prompt ^(HKLM write^).
  exit /b 2
)
if not exist "%DST%" mkdir "%DST%" || (echo [install] mkdir failed & exit /b 1)

rem Drop leftovers from a previous upgrade when the hosts have released them.
for %%f in ("%DST%\autoime_ime_old_*.dll") do del /q "%%f" 2>nul

rem Copy the DLL. An already loaded old DLL cannot be overwritten: park it and
rem copy the new build; the parked file is removed on a later install.
copy /y "%SRC%" "%DST%\autoime_ime.dll" >nul 2>&1
if errorlevel 1 (
  echo [install] old DLL is loaded by a host; parking it for later cleanup
  move /y "%DST%\autoime_ime.dll" "%DST%\autoime_ime_old_%RANDOM%.dll" >nul 2>&1
  copy /y "%SRC%" "%DST%\autoime_ime.dll" >nul || (echo [install] copy failed & exit /b 1)
)
if exist "%BUILD%\autoime_deploy.exe" copy /y "%BUILD%\autoime_deploy.exe" "%DST%\" >nul
if exist "%BUILD%\verify_registered.exe" copy /y "%BUILD%\verify_registered.exe" "%DST%\" >nul

rem Rime resources live next to the DLL (<install>\resources).
if exist "%DST%\resources" rmdir /s /q "%DST%\resources"
xcopy /e /i /y /q "%~dp0..\resources" "%DST%\resources" >nul || (echo [install] resource copy failed & exit /b 1)

rem Pre-deploy: compile + validate the schema now, not on the first key.
echo [install] deploying Rime resources (first deploy compiles dictionaries)...
"%DST%\autoime_deploy.exe" --resources-dir "%DST%\resources" --user-data-dir "%LOCALAPPDATA%\AutoIME\rime"
set "DEPLOY_EXIT=!errorlevel!"
echo [install] deploy exit=!DEPLOY_EXIT! (0=ready, 5=Luna fallback)
if "!DEPLOY_EXIT!"=="3" (
  echo [install] ERROR: engine disabled after deployment; IME will pass keys through.
  exit /b 3
)

regsvr32 /s "%DST%\autoime_ime.dll"
if errorlevel 1 (
  echo [install] regsvr32 failed - rolling back copied files
  del "%DST%\autoime_ime.dll" 2>nul
  rmdir /s /q "%DST%\resources" 2>nul
  exit /b 3
)
rem PLAN-070 T-04/F5: verify the registered path, binary and version.
"%DST%\verify_registered.exe" "%DST%\autoime_ime.dll"
if errorlevel 1 (
  echo [install] ERROR: registration verification failed ^(path/file/version^)
  exit /b 4
)
echo [install] OK: %DST%\autoime_ime.dll registered ^(zh-CN profile^)

rem The PLAN-069 prototype wrote plaintext key traces to %TEMP%\autoime-ime.log.
rem Remove it and tell the user where it was.
if exist "%DST%\autoime_deploy.exe" (
  "%DST%\autoime_deploy.exe" --cleanup-legacy-log
)

rem Add AutoIME to the current user's Win+Space switcher list (per-user hive).
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0enable-user-list.ps1"
echo [install] done. If the switcher list is stale, sign out/in or run:
echo   enable-user-list.ps1 again after a settings refresh.
endlocal
