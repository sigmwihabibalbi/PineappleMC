PineappleMC
===========

PineappleMC by sigma_conexion is a green and black Minecraft launcher based on the
Horizon Rust project. It includes Ely.by account sign-in, which opens the official
Ely.by login page in your browser. Enter your password only on ely.by.

INSTALLING ON WINDOWS
---------------------
1. Run dist\PineappleMC-Setup-0.2.0.exe.
2. Choose an install folder (the default is per-user and needs no administrator).
3. Open PineappleMC from the Start menu or desktop shortcut.
4. Open Accounts, choose Add account, then select Ely.by and complete sign-in in
   your browser. Return to PineappleMC when the page confirms authorization.
5. Select a Minecraft account that owns Java Edition to play. Ely.by accounts
   can be used for servers that accept Ely.by authentication; they do not replace
   the Microsoft ownership check for launching the official Minecraft client.

PineappleMC stores launcher data in %APPDATA%\PineappleMC. Account tokens are
stored using Windows Credential Manager. Uninstalling removes the app; it keeps
your game data and accounts so they are not accidentally lost.

BUILDING FROM SOURCE
--------------------
Requirements: Rust stable (rustup/cargo), Visual Studio C++ Build Tools, Windows
10/11 x64 SDK, and Inno Setup 6. From this folder run:

  cargo run --manifest-path launcher\Cargo.toml

To make a release build, first open a Visual Studio Developer PowerShell, then:

  cargo build --release --manifest-path launcher\Cargo.toml

The installer recipe is packaging\windows\pineapplemc.iss. Build it with Inno
Setup Compiler (ISCC.exe) after placing the release binary at dist\PineappleMC.exe.

PROJECT AND CREDITS
-------------------
This is an unofficial community launcher fork based on Horizon:
https://github.com/horizon-rust/horizon
Ely.by API documentation: https://docs.ely.by/en/api.html
Creator and launcher branding: sigma_conexion / PineappleMC.

Horizon source notices, third-party notices and license terms are included in
this project. PineappleMC is not affiliated with or endorsed by Mojang, Microsoft,
or Ely.by. Minecraft is a trademark of Microsoft/Mojang.
