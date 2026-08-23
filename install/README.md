# TRU Language (.tru) Installer

Registers the `.tru` file extension on Windows and puts `tru_id` on your PATH.

---

## Quick Start

```
install\install.bat
```

That's it. The batch file builds the project (if needed) then runs the installer.

---

## What Gets Installed

| Item | Value |
|---|---|
| Binary | `%LOCALAPPDATA%\tru_id\tru_id.exe` |
| Examples | `%LOCALAPPDATA%\tru_id\examples\*.tru` |
| File extension | `.tru` |
| MIME type | `text/x-tru` |
| ProgID | `tru_id.SourceFile` |
| PATH | `%LOCALAPPDATA%\tru_id` added to user PATH |
| Apps & Features | `TRU Language (tru_id_core)` entry |

## Registry Keys (HKCU)

```
HKCU\Software\Classes\.tru
  (Default) = tru_id.SourceFile
  Content Type = text/x-tru

HKCU\Software\Classes\tru_id.SourceFile
  shell\open\command   → tru_id run "%1"
  shell\run\command    → tru_id run "%1"
  shell\build\command  → tru_id build "%1"   (transpile → .rs)
  shell\repl\command   → tru_id repl
  shell\edit\command   → notepad.exe "%1"
```

## Right-Click Context Menu

After installation, right-clicking any `.tru` file shows:

- **Open** — run the file with `tru_id`
- **Run .tru file** — same as open
- **Transpile to Rust** — produces a `.rs` file alongside
- **Open TRU REPL** — launches interactive REPL
- **Edit source** — opens in Notepad

---

## Install Options

### Option A — Batch (Recommended)

```bat
install\install.bat
```

Builds release binaries automatically if missing, then runs the Rust installer.

### Option B — Rust Installer Direct

```powershell
cargo build --release --bin tru_id --bin tru_id_installer
.\target\release\tru_id_installer.exe install
.\target\release\tru_id_installer.exe install --dir C:\tools\tru
.\target\release\tru_id_installer.exe status
```

### Option C — PowerShell Script

```powershell
# Run as Administrator for full shell integration
powershell -ExecutionPolicy Bypass -File install\install.ps1
powershell -ExecutionPolicy Bypass -File install\install.ps1 -InstallDir C:\tools\tru
```

---

## Uninstall

```bat
install\uninstall.bat
```

Or via Apps & Features → search "TRU Language".

Or directly:

```powershell
tru_id_installer uninstall
```

---

## After Install

Open a **new terminal** (PATH changes need a fresh shell):

```
tru_id version
tru_id run   examples\hello.tru
tru_id build myfile.tru          # produces myfile.rs
tru_id repl
tru_id check myfile.tru
```

Restart Windows Explorer (or log off/on) for the right-click context menu to appear.
