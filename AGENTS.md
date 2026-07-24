# AGENTS.md

Single-crate Rust CLI: `startx.exe`, a Windows-native `Start-Process` alternative built on raw Win32 FFI (`ShellExecuteExW` via hand-written `extern` blocks). No external crates — `Cargo.lock` is intentionally near-empty; do not add dependencies without reason.

## Build & verify

- Build: `cargo build --release` (MSVC toolchain, Windows only).
- Check: `cargo check` / `cargo clippy`. No test suite exists; verification is manual (run `target/release/startx.exe` per examples in README).
- Static CRT is forced via `.cargo/config.toml` (`+crt-static`) for BOTH `x86_64-pc-windows-msvc` and `aarch64-pc-windows-msvc` — a hard requirement (README: "Static CRT", Windows 7+ compatible). Don't remove it.
- Release profile is aggressively size-optimized (`opt-level="z"`, lto, `panic="abort"`, strip). Keep it.
- `compile-project.bat` is the author's local clean-rebuild script.
- Local arm64 cross-build may fail with "linker `link.exe` not found" if the machine lacks ARM64 MSVC build tools — that's a local env limitation; CI (`windows-latest`) has them. x64 build works locally.

## Code conventions

- Everything lives in `src/main.rs` (single file). Win32 types/constants/structs are declared by hand in-file — extend that pattern rather than pulling in the `windows` crate.
- Style: compact, brace-on-new-line-ish, `x=1` without spaces around `=` in many places, minimal comments (only section banners). Match the existing look.
- `src/.main.rs.bak` is a leftover backup, not source — ignore it.
- CLI flags: `-v` is **Verb** (NOT version), `-V` is version, `-h` help, `-a` admin check, `-w` wait, `-d` directory, `-s` window style. Don't repurpose `-v`.
- Exit codes: 0 = success (or "is admin" for `-a`), 1 = runtime failure (or "not admin" for `-a`), 2 = argument error.
- The usage text is duplicated in `usage()` in main.rs AND in README.md — update both when adding/changing flags. Version is printed via `env!("CARGO_PKG_VERSION")`; never hardcode it.

## Releasing

- Use `release.sh +001|+010|+100` (bash; patch/minor/major). It bumps `Cargo.toml` version, commits `release vX.Y.Z`, tags, pushes, and prunes old tags (keeps 20, deletes from remote too).
- GitHub Release is automatic: `.github/workflows/release.yml` fires on `v*` tags, builds x64 + arm64 (matrix) and attaches `startx-vX.Y.Z-x64.zip` / `startx-vX.Y.Z-arm64.zip`. Never hand-edit the version in `Cargo.toml` for a release — let the script do it.
- `release.sh` has a bug: unknown args print an error but do NOT exit, so it proceeds with the last tag's version. Be careful to pass exactly `+001`/`+010`/`+100`.
