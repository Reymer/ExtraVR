; Inno Setup script for the ExtraVR streamer (PC side / SteamVR driver).
; Packages build\alvr_streamer_windows into a single ExtraVR-Setup installer.
;
; Build: "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer\ExtraVR.iss
; The version can be overridden from the command line (this is what CI does):
;        ISCC.exe /DAppVersion=1.2.3 installer\ExtraVR.iss
;
; All paths are derived from SourcePath (the folder holding this script) so the
; script compiles from any checkout location, not just C:\Projects\ALVR.

#define AppName "ExtraVR"
#ifndef AppVersion
  #define AppVersion "1.0.0"
#endif
#define AppExe "ExtraVR Dashboard.exe"
#define RepoRoot SourcePath + "..\"
#define SourceDir RepoRoot + "build\alvr_streamer_windows"

[Setup]
AppId={{9F2C7A1E-3B4D-4E6F-9A2B-EXTRAVR000001}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppName}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
UninstallDisplayName={#AppName}
UninstallDisplayIcon={app}\{#AppExe}
OutputDir={#RepoRoot}build
OutputBaseFilename=ExtraVR-Setup-{#AppVersion}
SetupIconFile={#RepoRoot}alvr\dashboard\resources\dashboard.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin

[Languages]
Name: "cht"; MessagesFile: "{#SourcePath}ChineseTraditional.isl"

[Files]
; Dashboard executable, installed under the ExtraVR product name.
Source: "{#SourceDir}\ALVR Dashboard.exe"; DestDir: "{app}"; DestName: "{#AppExe}"; Flags: ignoreversion
; Everything else (SteamVR driver, resources, manifest); session.json is runtime state.
Source: "{#SourceDir}\*"; DestDir: "{app}"; Excludes: "ALVR Dashboard.exe,session.json,*.pdb"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{group}\解除安裝 {#AppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "建立桌面捷徑"; GroupDescription: "附加捷徑："

[Run]
Filename: "{app}\{#AppExe}"; Description: "啟動 {#AppName}"; Flags: nowait postinstall skipifsilent

[Code]
{ Locate the SteamVR runtime folder from openvrpaths.vrpath (authoritative source). }
function GetSteamVrRuntime(): String;
var
  Content: AnsiString;
  S: String;
  P, A, B: Integer;
begin
  Result := '';
  if not LoadStringFromFile(ExpandConstant('{localappdata}\openvr\openvrpaths.vrpath'), Content) then
    exit;
  S := String(Content);
  P := Pos('"runtime"', S);
  if P = 0 then exit;
  Delete(S, 1, P + 8);
  A := Pos('"', S);
  if A = 0 then exit;
  Delete(S, 1, A);
  B := Pos('"', S);
  if B = 0 then exit;
  Result := Copy(S, 1, B - 1);
  StringChangeEx(Result, '\\', '\', True);
end;

{ Add the same inbound firewall rules ExtraVR/ALVR adds (allow SteamVR's vrserver). }
procedure AddFirewallRules();
var
  Runtime: String;
  Code: Integer;
begin
  Runtime := GetSteamVrRuntime();
  if Runtime = '' then exit;
  Exec('netsh.exe',
    'advfirewall firewall add rule name="SteamVR ALVR vrserver" dir=in program="' + Runtime + '\bin\win64\vrserver.exe" action=allow',
    '', SW_HIDE, ewWaitUntilTerminated, Code);
  Exec('netsh.exe',
    'advfirewall firewall add rule name="SteamVR ALVR vrserver" dir=in program="' + Runtime + '\bin\win32\vrserver.exe" action=allow',
    '', SW_HIDE, ewWaitUntilTerminated, Code);
end;

{ ExtraVR (like ALVR) stores session.json and its logs INSIDE the install folder.
  vrserver.exe runs non-elevated, but Program Files denies write to standard users,
  so first-run session.json creation fails and the SteamVR driver crashes on load
  (C++ exception e06d7363 -> safe mode -> driver blocked). Grant the install folder
  Modify rights to BUILTIN\Users (SID S-1-5-32-545, language-neutral) to fix this. }
procedure GrantWritePermissions();
var
  Code: Integer;
begin
  Exec(ExpandConstant('{sys}\icacls.exe'),
    '"' + ExpandConstant('{app}') + '" /grant *S-1-5-32-545:(OI)(CI)M /T /C',
    '', SW_HIDE, ewWaitUntilTerminated, Code);
end;

{ Register the installed folder as a SteamVR driver (same as the in-app button). }
procedure RegisterDriver();
var
  Runtime, Vrpathreg: String;
  Code: Integer;
begin
  Runtime := GetSteamVrRuntime();
  if Runtime = '' then exit;
  Vrpathreg := Runtime + '\bin\win64\vrpathreg.exe';
  if not FileExists(Vrpathreg) then exit;
  Exec(Vrpathreg, 'adddriver "' + ExpandConstant('{app}') + '"',
    '', SW_HIDE, ewWaitUntilTerminated, Code);
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    GrantWritePermissions();
    AddFirewallRules();
    RegisterDriver();
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Runtime, Vrpathreg: String;
  Code: Integer;
begin
  if CurUninstallStep = usUninstall then
  begin
    { Unregister the driver + drop the firewall rule before files are removed. }
    Runtime := GetSteamVrRuntime();
    if Runtime <> '' then
    begin
      Vrpathreg := Runtime + '\bin\win64\vrpathreg.exe';
      if FileExists(Vrpathreg) then
        Exec(Vrpathreg, 'removedriver "' + ExpandConstant('{app}') + '"',
          '', SW_HIDE, ewWaitUntilTerminated, Code);
    end;
    Exec('netsh.exe', 'advfirewall firewall delete rule name="SteamVR ALVR vrserver"',
      '', SW_HIDE, ewWaitUntilTerminated, Code);
  end;
end;
