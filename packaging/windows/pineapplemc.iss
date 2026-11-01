#define AppName "PineappleMC"
#define AppVersion "0.2.0"
#define AppPublisher "sigma_conexion"
#define AppExe "PineappleMC.exe"

[Setup]
AppId={{8E8087B1-6B66-468C-A3B7-9CFD59E24B7A}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={localappdata}\Programs\PineappleMC
DefaultGroupName={#AppName}
UninstallDisplayIcon={app}\{#AppExe}
SetupIconFile=..\..\packaging\icons\pineapplemc.ico
OutputDir=..\..\dist
OutputBaseFilename=PineappleMC-Setup-{#AppVersion}
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest
WizardStyle=modern
Compression=lzma2
SolidCompression=yes
CloseApplications=yes
ChangesAssociations=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Shortcuts:"; Flags: unchecked

[Files]
Source: "..\..\dist\PineappleMC.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\THIRD_PARTY_NOTICES.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\PineappleMC"; Filename: "{app}\{#AppExe}"
Name: "{autodesktop}\PineappleMC"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; Description: "Launch PineappleMC"; Flags: nowait postinstall skipifsilent
