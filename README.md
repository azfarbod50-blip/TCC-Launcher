<div align="center">

<img src=".github/media/RepoBanner.png" alt="Repository Banner" />

# TCC Launcher
An offline Minecraft launcher - no Microsoft account required.

TCC Launcher is a Minecraft launcher featuring fully offline authentication, offering a clean UI for managing Minecraft instances without requiring a Microsoft account.

</div>

> [!NOTE]
> This is **TCC Launcher v1** — a fork of OneLauncher/OneClient rewritten for offline-only authentication.
> All Microsoft authentication code has been removed.

## Features
- **Offline Authentication**: Create and manage offline accounts (no Microsoft account needed)
- **Clean UI**: Modern, responsive interface built with Freya (Rust GUI framework)
- **Instance Management**: Create and manage Minecraft instances with custom configurations
- **Mod Support**: Install and manage mods, shaders, and resource packs
- **Cross-platform**: Windows, macOS, and Linux support

## Technologies Used
- **Rust** (edition 2024)
- **Freya** - native GUI framework for Rust
- **SQLite** - local database for account and configuration storage

## Contributing
We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING.md) before getting started.

### Requirements
- Install Rust via [rustup](https://rustup.rs/)
- Use a toolchain that supports edition 2024 (`rustc` **1.85+**)

### Building & Running
```sh
# Run the app
cargo run -p tcc_app

# Build a release binary
cargo build --release -p tcc_app
```

### Packaging / Releasing
Installers are produced with **cargo-packager**.

```sh
cargo install cargo-packager --locked

# Build the binary, then bundle it for the current OS:
cargo build --release -p tcc_app
cargo packager --release -p tcc_app --formats <targets>
#   Windows: nsis      macOS: app,dmg      Linux: deb,appimage
```

### Versioning
The workspace shares a single version, defined in the root `Cargo.toml` under `[workspace.package]`. Current version: **1.0.0**.

### Project Structure
All crates live under **`packages/`** in a single Cargo workspace.

- **`tcc_app/`** - The Freya desktop application (UI, routes, entry point).
- **`tcc_auth/`** - Offline authentication and credentials store.
- **`tcc_core/`** - Launcher core logic.
- **`tcc_db/`** - SQLx-based database layer.
- **`tcc_events/`** - Event system.
- **`tcc_java/`** - Java runtime management.
- **`tcc_macro/`** - Macro definitions.
- **`tcc_mc/`** - Minecraft utilities.
- **`tcc_net/`** - Networking utilities.
- **`tcc_cluster/`** - Instance/cluster management.
- **`tcc_content/`** - Content management.
- **`tcc_polyplus/`** - PolyPlus integration.
- **`polyio/`** - Shared IO utilities.

## Authentication
TCC Launcher uses **offline authentication only**. No Microsoft/Xbox account is required.

- Accounts are created locally with a username (3-16 characters, alphanumeric + underscore)
- Each account gets a deterministic UUID based on the username (OfflinePlayer format)
- Accounts are stored in a local SQLite database (`auth.db`)
- No network requests are made for authentication

## Hosting
All assets, updates, and releases are hosted on **GitHub**:
- Repository: https://github.com/azfarbod50-blip/TCC-Launcher
- Releases: https://github.com/azfarbod50-blip/TCC-Launcher/releases
- API: https://api.github.com/repos/azfarbod50-blip/TCC-Launcher

## License
GNU General Public License v3.0 - see [LICENSE](LICENSE) for details.