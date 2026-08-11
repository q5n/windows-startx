# AGENTS.md

Single-crate Rust CLI: `startx.exe`, a Windows-native `Start-Process` alternative built on raw Win32 FFI (`ShellExecuteExW` via hand-written `extern` blocks). No external crates — `Cargo.lock` is intentionally near-empty; do not add dependencies without reason.

## Build & verify

- Build: `cargo build --release` (MSVC toolchain, Windows only).
- Check: `cargo check` / `cargo clippy`. No test suite exists; verification is manual (run `target/release/startx.exe` per examples in README).
- Static CRT is forced via `.cargo/config.toml` (`+crt-static`) for `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`, and `i686-pc-windows-msvc` — a hard requirement (README: "Static CRT", Windows 7+ compatible). Don't remove it.
- Release profile is aggressively size-optimized (`opt-level="z"`, lto, `panic="abort"`, strip). Keep it.
- `compile-project.bat` is the author's local clean-rebuild script (clean + release build + runs `startx -h` as a smoke check).
- Local arm64 cross-build may fail with "linker `link.exe` not found" if the machine lacks ARM64 MSVC build tools — that's a local env limitation; CI (`windows-latest`) has them. x64 build works locally.

## Code conventions

- Everything lives in `src/main.rs` (single file). Win32 types/constants/structs are declared by hand in-file — extend that pattern rather than pulling in the `windows` crate.
- Style: compact, brace-on-new-line-ish, `x=1` without spaces around `=` in many places, minimal comments (only section banners). Match the existing look.
- CLI flags: `-v` is **Verb** (NOT version), `-V` is version, `-h` help, `-a` admin check, `-w` wait, `-d` directory, `-s` window style. Don't repurpose `-v`.
- Exit codes: 0 = success (or "is admin" for `-a`), 1 = runtime failure (or "not admin" for `-a`), 2 = argument error.
- The usage text is duplicated in `usage()` in main.rs AND in README.md — update both when adding/changing flags. Version is printed via `env!("CARGO_PKG_VERSION")`; never hardcode it.

## Releasing

- Use `release.sh +001|+010|+100` (bash; patch/minor/major). It bumps `Cargo.toml` version, commits `release vX.Y.Z`, tags, pushes, and prunes old tags (keeps 20, deletes from remote too). Any other arg exits with an error.
- GitHub Release is automatic: `.github/workflows/release.yml` fires on `v*` tags, builds `i686-pc-windows-msvc` and attaches `startx-vX.Y.Z-x86.zip`. Never hand-edit the version in `Cargo.toml` for a release — let the script do it.
- Manual x64 / arm64 releases: `.github/workflows/release-x64.yml` and `.github/workflows/release-arm64.yml` are `workflow_dispatch` workflows. They build `x86_64-pc-windows-msvc` / `aarch64-pc-windows-msvc` and upload `startx-vX.Y.Z-x64.zip` / `startx-vX.Y.Z-arm64.zip` to the specified release (defaults to the latest release).
- WinGet publishing is manual: run `.github/workflows/winget.yml` (workflow_dispatch) after a release. Requires secret `WINGET_TOKEN` (classic PAT, `public_repo`) and a fork of `microsoft/winget-pkgs`. Uses `wingetcreate` only (no third-party actions): version == `FIRST_SUBMIT_VERSION` (0.2.5) → `submit` of the hand-written manifests in `winget/` (SHA256 placeholders filled at runtime); any other version → `update`. First-ever submission must be merged by winget-pkgs before updates work.
