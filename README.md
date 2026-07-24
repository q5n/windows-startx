# windows-startx

A lightweight Windows Start-Process alternative.

Inspired by PowerShell Start-Process.

## Features

- Native Win32 API
- No PowerShell dependency
- No .NET dependency
- Static CRT
- Windows 7+ compatible

Supported options:

| startx | Start-Process |
|-|-|
| -v | -Verb |
| -w | -Wait |
| -d | -WorkingDirectory |
| -s | -WindowStyle |

Extra options (no Start-Process equivalent): `-a` (check admin rights), `-V` (show version), `-h` (show help).

## Usage

```cmd
Usage:
 startx [-v Verb] [-w] [-d Directory] [-s WindowStyle] <Program> [Arguments...]

Options:
 -v <Verb>          Shell Verb example: open/runas/edit/print/...
 -w                 Wait for process exit
 -d <Directory>     Working directory
 -s <WindowStyle>   Normal/Hidden/Minimized/Maximized
 -a                 Check admin rights (exit 0=admin, 1=not admin)
 -V                 Show version (current: v0.2.4)
 -h                 Show help and version

Examples:
  startx notepad.exe
  startx notepad.exe "C:\Documents\hello world.txt"
  startx -w notepad.exe "C:\Documents\test.txt"
  startx -v runas cmd.exe /k whoami
  startx -v runas -w cmd.exe /c "whoami && pause"
  startx -d "C:\Work" app.exe --config "dev config.json"
  startx -s Minimized app.exe
  startx -s Hidden cmd.exe /c "echo hello > C:\Temp\result.txt"
  startx -- "-special-name.exe" -v child-argument
  startx -a && echo running as admin
```


## Build
Requirements:
- Rust
- MSVC toolchain

Build:
```cmd
cargo build --release
```


## License

MIT