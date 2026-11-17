@echo off
rem AutoIME automated fixture runner (PLAN-070 T-01; review finding P070-F1).
rem This command does NOT build: preparation is the explicit `ime\build.cmd`
rem step, and a missing/stale binary is a named preparation error. The fixture
rem prints per-case wall time.
setlocal
set "EXE=%~dp0build\autoime_fixture.exe"
if not exist "%EXE%" (
  echo [test] ERROR: preparation missing - fixture not built: "%EXE%"
  echo [test] run "ime\build.cmd" first ^(explicit preparation step^)
  exit /b 2
)
"%EXE%"
if errorlevel 1 exit /b %errorlevel%
rem Privacy source audit: structural ImeLog must never receive key codes,
rem pre-edit, commit or selection text.
findstr /R /C:"ImeLog(.*preedit" /C:"ImeLog(.*commit" /C:"ImeLog(.*vk=" /C:"ImeLog(.*selection" "%~dp0*.cpp" >nul 2>&1
if not errorlevel 1 (
  echo [privacy] FAIL: payload-bearing ImeLog call found
  exit /b 1
)
echo [privacy] source audit OK: no payload-bearing ImeLog calls
endlocal
