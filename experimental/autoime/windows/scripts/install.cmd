@echo off
rem AutoIME TSF install (PLAN-069 T-02). Requires an elevated prompt (HKLM).
rem Usage: install.cmd [path\to\autoime_ime.dll]
setlocal
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
copy /y "%SRC%" "%DST%\" >nul || (echo [install] copy failed & exit /b 1)
regsvr32 /s "%DST%\autoime_ime.dll"
if errorlevel 1 (
  echo [install] regsvr32 failed - rolling back copied files
  del "%DST%\autoime_ime.dll" 2>nul
  rmdir "%DST%" 2>nul
  exit /b 3
)
echo [install] OK: %DST%\autoime_ime.dll registered ^(zh-CN profile^)
endlocal
