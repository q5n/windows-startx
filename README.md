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
| -a | (check admin rights) |

## Usage

```cmd
Usage:
 startx.exe [-v Verb] [-w] [-d Directory] [-s WindowStyle] <Program> [Arguments...]

Options:
 -v <Verb>          Shell Verb example: open/runas/edit/print/...
 -w                 Wait for process exit
 -d <Directory>     Working directory
 -s <Style>         Normal/Hidden/Minimized/Maximized
 -a                 Check admin rights (exit 0=admin, 1=not admin)

Examples:
  startx.exe notepad.exe
  startx.exe notepad.exe "C:\Documents\hello world.txt"
  startx.exe -w notepad.exe "C:\Documents\test.txt"
  startx.exe -v runas cmd.exe /k whoami
  startx.exe -v runas -w cmd.exe /c "whoami && pause"
  startx.exe -d "C:\Work" app.exe --config "dev config.json"
  startx.exe -s Minimized app.exe
  startx.exe -s Hidden cmd.exe /c "echo hello > C:\Temp\result.txt"
  startx.exe -- "-special-name.exe" -v child-argument
  startx.exe -a && echo running as admin
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