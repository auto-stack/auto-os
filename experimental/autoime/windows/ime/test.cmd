@echo off
rem AutoIME PLAN-070 T-01 automated fixture: build + replay.
setlocal
call "%~dp0build.cmd" || exit /b 1
"%~dp0build\autoime_fixture.exe"
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
