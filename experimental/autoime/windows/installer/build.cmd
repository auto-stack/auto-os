@echo off
rem AutoIME Windows x64 offline installer build (PLAN-071 T-02).
rem
rem Stages the runtime payload from the repo, verifies the bundled engine
rem against bundle-manifest.json (locked SHA-256; mismatch = named
rem preparation failure, no package), then compiles the Inno Setup
rem definition into dist\AutoIME-Setup-x64.exe. Fully offline.
rem
rem Prerequisites: ime\build.cmd has produced ime\build\autoime_ime.dll +
rem autoime_deploy.exe + verify_registered.exe; Inno Setup 6 (ISCC.exe).
setlocal
set "HERE=%~dp0"
set "AUTOIME=%HERE%..\.."
set "IME_BUILD=%AUTOIME%\windows\ime\build"
set "STAGE=%HERE%stage"

set "ISCC=%ProgramFiles(x86)%\Inno Setup 6\ISCC.exe"
if not exist "%ISCC%" set "ISCC=%ProgramFiles%\Inno Setup 6\ISCC.exe"
if not exist "%ISCC%" set "ISCC=%LOCALAPPDATA%\Programs\Inno Setup 6\ISCC.exe"
if not exist "%ISCC%" (
  echo [installer] ERROR: Inno Setup 6 not found ^(ISCC.exe^).
  echo [installer] install Inno Setup 6 or set ISCC manually in this script.
  exit /b 1
)

if not exist "%IME_BUILD%\autoime_ime.dll" (
  echo [installer] ERROR: preparation missing - "%IME_BUILD%\autoime_ime.dll"
  echo [installer] run "ime\build.cmd" first ^(explicit preparation step^)
  exit /b 1
)

if exist "%STAGE%" rmdir /s /q "%STAGE%"
mkdir "%STAGE%" || exit /b 1

rem 1. IME DLL + deployment/verification tools + per-user list scripts.
copy /y "%IME_BUILD%\autoime_ime.dll" "%STAGE%\" >nul || exit /b 1
copy /y "%IME_BUILD%\autoime_deploy.exe" "%STAGE%\" >nul || exit /b 1
copy /y "%IME_BUILD%\verify_registered.exe" "%STAGE%\" >nul || exit /b 1
copy /y "%AUTOIME%\windows\scripts\enable-user-list.ps1" "%STAGE%\" >nul || exit /b 1
copy /y "%AUTOIME%\windows\scripts\disable-user-list.ps1" "%STAGE%\" >nul || exit /b 1

rem 2. Shared Rime resources (single source of truth, PLAN-071 T-01).
xcopy /e /i /y /q "%AUTOIME%\shared\resources" "%STAGE%\resources" >nul || (
  echo [installer] ERROR: resource staging failed
  exit /b 1
)

rem 3. Bundled engine + shared data, verified against the locked manifest.
powershell -NoProfile -ExecutionPolicy Bypass -File "%HERE%stage-engine.ps1" -Stage "%STAGE%\rime" || exit /b 1

rem 4. Licenses / third-party notices (shown on the license page too).
if not exist "%STAGE%\licenses" mkdir "%STAGE%\licenses"
copy /y "%HERE%THIRD-PARTY-NOTICES.md" "%STAGE%\licenses\" >nul || exit /b 1
powershell -NoProfile -Command ^
  "$c = Get-Content '%HERE%THIRD-PARTY-NOTICES.md' -Raw -Encoding UTF8; Set-Content '%STAGE%\licenses\THIRD-PARTY-NOTICES.txt' $c -Encoding UTF8" || exit /b 1

rem 5. Compile the installer.
if not exist "%HERE%dist" mkdir "%HERE%dist"
"%ISCC%" "%HERE%autoime-setup.iss" || exit /b 1

rem 6. Record the produced package hash next to it (deliverable identity).
powershell -NoProfile -Command ^
  "$exe = '%HERE%dist\AutoIME-Setup-x64.exe'; $h = (Get-FileHash $exe -Algorithm SHA256).Hash; Set-Content '%HERE%dist\AutoIME-Setup-x64.exe.sha256' ($h + '  AutoIME-Setup-x64.exe') -Encoding ASCII" || exit /b 1
echo [installer] OK: %HERE%dist\AutoIME-Setup-x64.exe
endlocal
