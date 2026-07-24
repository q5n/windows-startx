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

## Usage

```cmd
Usage:
 startx.exe [-v Verb] [-w] [-d Directory] [-s WindowStyle] <Program> [Arguments...]

Options:
 -v <Verb>          Shell Verb example: open/runas/edit/print/...
 -w                 Wait for process exit
 -d <Directory>     Working directory
 -s <Style>         Normal/Hidden/Minimized/Maximized

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