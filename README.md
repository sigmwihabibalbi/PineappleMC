# PineappleMC

**PineappleMC by sigma_conexion** is an unofficial, green-themed Minecraft launcher built from the Horizon Rust project. It adds Ely.by account sign-in and a PineappleMC-branded Windows installer.

**Website:** [sigmwihabibalbi.github.io/PineappleMC](https://sigmwihabibalbi.github.io/PineappleMC/)

## Download

Download **PineappleMC-Setup** from this repository's [Releases](../../releases) page. Run the installer for a per-user installation; administrator access is not required. The project also includes `README.txt` with quick Windows setup notes.

## Ely.by accounts

1. Open PineappleMC and go to **Settings**.
2. Select **Sign in with Ely.by**.
3. Complete the device-code sign-in on the official Ely.by website.
4. Select the account you want from the account menu.

The selected Ely.by profile name is passed to the game. Account tokens are stored in the operating system credential store; passwords are entered on Ely.by, not in the launcher. The login browser opens once per sign-in attempt.

PineappleMC is an unofficial project. Ely.by authentication works with servers that accept Ely.by accounts; it does not provide Microsoft/Mojang ownership for launching the official Minecraft Java client.

## Build from source

Requirements: Rust stable with Cargo, Visual Studio C++ Build Tools, and the Windows 10/11 SDK.

```powershell
cargo run --manifest-path launcher\Cargo.toml
cargo build --release --manifest-path launcher\Cargo.toml
```

The Windows installer recipe is `packaging/windows/pineapplemc.iss`. Build it with Inno Setup 6 after placing the release executable at `dist/PineappleMC.exe`.

## Project and license

PineappleMC is a community fork based on [Horizon](https://github.com/horizon-rust/horizon). See [Ely.by API documentation](https://docs.ely.by/en/api.html), [LICENSE](LICENSE), and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Horizon's original notices and license terms are preserved.

PineappleMC is not affiliated with or endorsed by Mojang, Microsoft, or Ely.by. Minecraft is a trademark of Microsoft/Mojang.
