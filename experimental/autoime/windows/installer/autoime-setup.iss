; AutoIME Windows x64 offline installer (PLAN-071 T-02).
;
; Machine-level install: program files into {autopf}\AutoIME, TSF
; registration via regsvr32 (elevated). Per-user steps (Rime user data
; pre-deploy with dictionary compilation, Win+Space switcher entry) run in
; the ORIGINAL user's context, so over-the-shoulder installs (a standard
; user + another admin's credentials) still prepare that user's own data.
;
; The payload is staged by build.cmd: the built IME DLL + deploy/verify
; tools, shared/resources, the locked bundled engine (rime.dll + Weasel
; shared data verified against bundle-manifest.json) and third-party notices.
; No network access at build or install time.

#define AutoIMEName "AutoIME"
#define AutoIMEVersion "0.1.0"

[Setup]
AppId={{7A1C0E52-91F4-4BD8-9F3A-2C6B4D5E8F70}
AppName={#AutoIMEName}
AppVersion={#AutoIMEVersion}
AppVerName={#AutoIMEName} {#AutoIMEVersion} (Windows x64 offline)
AppPublisher=AutoIME (AutoStack)
DefaultDirName={autopf}\AutoIME
DisableProgramGroupPage=yes
; No app shortcut: AutoIME is a TSF text service, not a launched app.
UninstallDisplayIcon={app}\autoime_ime.dll
OutputDir=dist
OutputBaseFilename=AutoIME-Setup-x64
SetupIconFile=..\ime\autoime.ico
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
; The upgrade path parks a DLL that hosts still hold (same-volume rename);
; no close-applications nag and no reboot requirement in the normal flow.
CloseApplications=no
SetupLogging=yes
; Generated from THIRD-PARTY-NOTICES.md by build.cmd (staging step 4, before
; ISCC runs) - shown on the installer's license page and shipped in licenses\.
LicenseFile=stage\licenses\THIRD-PARTY-NOTICES.txt

[Files]
Source: "stage\autoime_ime.dll"; DestDir: "{app}"; Flags: ignoreversion restartreplace
Source: "stage\autoime_deploy.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "stage\verify_registered.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "stage\enable-user-list.ps1"; DestDir: "{app}"; Flags: ignoreversion
Source: "stage\disable-user-list.ps1"; DestDir: "{app}"; Flags: ignoreversion
Source: "stage\resources\*"; DestDir: "{app}\resources"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "stage\rime\rime.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "stage\rime\data\*"; DestDir: "{app}\data"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "stage\licenses\*"; DestDir: "{app}\licenses"; Flags: ignoreversion recursesubdirs createallsubdirs

[InstallDelete]
; Files parked by earlier upgrades are removed here only when unlocked; the
; locked current DLL is parked in [Code] before this list runs.
; (Intentionally empty: parking cleanup happens in [Code] so the freshly
; parked file is not deleted by a later-ordered pass.)

[Run]
; Machine-level TSF registration (elevated context; HKLM writes).
Filename: "{sys}\regsvr32.exe"; Parameters: "/s ""{app}\autoime_ime.dll"""; \
    StatusMsg: "Registering AutoIME TSF text service..."; Flags: runascurrentuser
Filename: "{app}\verify_registered.exe"; \
    Parameters: """{app}\autoime_ime.dll"""; \
    StatusMsg: "Verifying registration (path/file/version)..."; \
    Flags: runascurrentuser
; Per-user steps in the original user's context (over-the-shoulder safe):
; pre-deploy personal Rime data (compiles dictionaries so the first key
; never waits) and add AutoIME to that user's Win+Space switcher list.
; {localappdata} matches the runtime DefaultUserDir (%LOCALAPPDATA%\AutoIME\rime)
; so the pre-deployed data IS the data the IME uses.
Filename: "{app}\autoime_deploy.exe"; \
    Parameters: "--resources-dir ""{app}\resources"" --user-data-dir ""{localappdata}\AutoIME\rime"""; \
    StatusMsg: "Preparing your Rime user data (compiling dictionaries)..."; \
    Flags: runasoriginaluser
Filename: "powershell.exe"; \
    Parameters: "-NoProfile -ExecutionPolicy Bypass -File ""{app}\enable-user-list.ps1"""; \
    StatusMsg: "Adding AutoIME to your input switcher list..."; \
    Flags: runasoriginaluser

[UninstallRun]
; Registration is revoked; program files are removed by the uninstaller.
; Personal data (%LOCALAPPDATA%\AutoIME) is KEPT (phrases + learning).
Filename: "{sys}\regsvr32.exe"; Parameters: "/u /s ""{app}\autoime_ime.dll"""; \
    RunOnceId: "UnregisterAutoIME"; Flags: runascurrentuser
Filename: "powershell.exe"; \
    Parameters: "-NoProfile -ExecutionPolicy Bypass -File ""{app}\disable-user-list.ps1"""; \
    RunOnceId: "RemoveAutoIMESwitcherEntry"

[UninstallDelete]
; Parked old DLLs are not Inno-owned; remove them with the program files
; (both parking schemes; a DLL still mapped by a live host can't be deleted
; and is cleaned by a later install/uninstall). The resources tree is Inno-
; owned but its empty parent dirs would otherwise survive uninstall.
Type: files; Name: "{app}\autoime_ime_*.dll"
Type: filesandordirs; Name: "{app}\resources"

[Code]
// Win32 tick count for unique parked-DLL names (not built into PascalScript).
function GetTickCount: DWORD;
  external 'GetTickCount@kernel32.dll stdcall';

// Removes files parked by earlier upgrades (their hosts have released them
// by the time a new install runs). The glob covers both parking schemes:
// autoime_ime_old_<n>.dll (Inno installs) and the numeric autoime_ime_<n>.dll
// scheme left by the 070-era scripts; autoime_ime.dll itself never matches.
procedure DeleteParkedDlls(const app: String);
var
  find: TFindRec;
begin
  if FindFirst(app + '\autoime_ime_*.dll', find) then begin
    try
      repeat
        if (find.Attributes and FILE_ATTRIBUTE_DIRECTORY) = 0 then
          DeleteFile(app + '\' + find.Name);
      until not FindNext(find);
    finally
      FindClose(find);
    end;
  end;
end;

// PLAN-071 T-03: upgrades must work while a host still holds the old DLL.
// A same-volume rename of a mapped image succeeds, so park the current DLL
// (autoime_ime_old_<tick>.dll) before [Files] copies the new build. Parked
// files are cleaned by later installs (above) and by uninstall.
procedure CurStepChanged(CurStep: TSetupStep);
var
  app, dll, parked: String;
begin
  if CurStep <> ssInstall then Exit;
  app := ExpandConstant('{app}');
  DeleteParkedDlls(app);
  dll := app + '\autoime_ime.dll';
  if FileExists(dll) then begin
    parked := app + Format('\autoime_ime_old_%d.dll', [GetTickCount]);
    RenameFile(dll, parked);
  end;
end;
