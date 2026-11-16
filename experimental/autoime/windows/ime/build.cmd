@echo off
rem AutoIME TSF service build (PLAN-069 T-02).
setlocal
set "VCVARS=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
if not exist "%VCVARS%" (echo [build] ERROR: vcvars64.bat not found & exit /b 1)
call "%VCVARS%" >nul || exit /b 1
cmake -S "%~dp0." -B "%~dp0build" -G Ninja %* || exit /b 1
cmake --build "%~dp0build" || exit /b 1
echo [build] OK: %~dp0build\autoime_ime.dll + verify_registered.exe
endlocal
