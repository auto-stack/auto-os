@echo off
rem AutoIME Windows probe build (PLAN-069 T-01).
rem Usage: build.cmd [extra cmake args]
rem Requires MSVC 2022 (vcvars64) + CMake + Ninja on PATH after vcvars.
setlocal
set "VCVARS=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
if not exist "%VCVARS%" (
  echo [build] ERROR: vcvars64.bat not found at %VCVARS%
  exit /b 1
)
call "%VCVARS%" >nul || (echo [build] ERROR: vcvars64 failed & exit /b 1)
cmake -S "%~dp0." -B "%~dp0build" -G Ninja %* || exit /b 1
cmake --build "%~dp0build" || exit /b 1
echo [build] OK: %~dp0build\rime_probe.exe
