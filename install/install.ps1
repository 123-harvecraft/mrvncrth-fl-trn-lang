# install.ps1 — TRU Language (.tru) Windows Extension Installer
# Registers .tru with the tru_id runner, sets up file associations and PATH.
# Run as Administrator:  powershell -ExecutionPolicy Bypass -File install.ps1

#Requires -Version 5.1
param(
    [string]$InstallDir = "$env:LOCALAPPDATA\tru_id",
    [switch]$Uninstall,
    [switch]$Silent
)

$ErrorActionPreference = "Stop"

$VERSION    = "0.1.0"
$LANG_NAME  = "TRU Language"
$EXT        = ".tru"
$PROG_ID    = "tru_id.SourceFile"
$BIN_NAME   = "tru_id.exe"
$BIN_SRC    = Join-Path $PSScriptRoot "..\target\release\$BIN_NAME"

# ─── Helpers ──────────────────────────────────────────────────────────────────

function Write-Header {
    Write-Host ""
    Write-Host "  ████████╗██████╗ ██╗   ██╗    ██╗██████╗ " -ForegroundColor Cyan
    Write-Host "     ██╔══╝██╔══██╗██║   ██║    ██║██╔══██╗" -ForegroundColor Cyan
    Write-Host "     ██║   ██████╔╝██║   ██║    ██║██║  ██║" -ForegroundColor Cyan
    Write-Host "     ██║   ██╔══██╗██║   ██║    ██║██║  ██║" -ForegroundColor Cyan
    Write-Host "     ██║   ██║  ██║╚██████╔╝    ██║██████╔╝" -ForegroundColor Cyan
    Write-Host "     ╚═╝   ╚═╝  ╚═╝ ╚═════╝     ╚═╝╚═════╝ " -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  TRU Language Installer v$VERSION" -ForegroundColor White
    Write-Host "  Extension: $EXT  |  Crate: tru_id_core" -ForegroundColor Gray
    Write-Host ""
}

function Log-Step([string]$msg) {
    if (-not $Silent) { Write-Host "  [>] $msg" -ForegroundColor Yellow }
}

function Log-Ok([string]$msg) {
    if (-not $Silent) { Write-Host "  [OK] $msg" -ForegroundColor Green }
}

function Log-Err([string]$msg) {
    Write-Host "  [ERR] $msg" -ForegroundColor Red
}

function Require-Admin {
    $cur = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($cur)
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        Write-Host ""
        Write-Host "  This installer needs Administrator rights." -ForegroundColor Red
        Write-Host "  Re-launching as Administrator..." -ForegroundColor Yellow
        $args_fwd = "-ExecutionPolicy Bypass -File `"$PSCommandPath`""
        if ($InstallDir -ne "$env:LOCALAPPDATA\tru_id") { $args_fwd += " -InstallDir `"$InstallDir`"" }
        if ($Uninstall) { $args_fwd += " -Uninstall" }
        if ($Silent)    { $args_fwd += " -Silent" }
        Start-Process powershell -ArgumentList $args_fwd -Verb RunAs
        exit
    }
}

# ─── Uninstall ────────────────────────────────────────────────────────────────

function Do-Uninstall {
    Write-Header
    Write-Host "  Uninstalling TRU Language..." -ForegroundColor Magenta

    # Remove registry keys
    Log-Step "Removing registry entries..."
    $keys = @(
        "HKCU:\Software\Classes\$EXT",
        "HKCU:\Software\Classes\$PROG_ID",
        "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\$EXT"
    )
    foreach ($key in $keys) {
        if (Test-Path $key) {
            Remove-Item -Path $key -Recurse -Force
            Log-Ok "Removed $key"
        }
    }

    # Remove from PATH
    Log-Step "Removing from PATH..."
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -like "*$InstallDir*") {
        $newPath = ($currentPath -split ";" | Where-Object { $_ -ne $InstallDir }) -join ";"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Log-Ok "Removed from PATH"
    }

    # Remove files
    Log-Step "Removing installed files..."
    if (Test-Path $InstallDir) {
        Remove-Item -Path $InstallDir -Recurse -Force
        Log-Ok "Removed $InstallDir"
    }

    # Refresh shell
    if (-not $Silent) {
        $sig = '[DllImport("shell32.dll")] public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);'
        Add-Type -MemberDefinition $sig -Name "WinAPI" -Namespace "Shell32" -ErrorAction SilentlyContinue
        [Shell32.WinAPI]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)
    }

    Write-Host ""
    Write-Host "  TRU Language uninstalled successfully." -ForegroundColor Green
}

# ─── Install ──────────────────────────────────────────────────────────────────

function Do-Install {
    Write-Header

    # 1. Locate binary
    Log-Step "Locating tru_id binary..."
    $binPath = $null

    if (Test-Path $BIN_SRC) {
        $binPath = $BIN_SRC
        Log-Ok "Found release build: $binPath"
    } else {
        $debugBin = Join-Path $PSScriptRoot "..\target\debug\$BIN_NAME"
        if (Test-Path $debugBin) {
            $binPath = $debugBin
            Log-Ok "Found debug build: $binPath"
        } else {
            $inPath = Get-Command $BIN_NAME -ErrorAction SilentlyContinue
            if ($inPath) {
                $binPath = $inPath.Source
                Log-Ok "Found in PATH: $binPath"
            } else {
                Log-Err "tru_id.exe not found. Build first with: cargo build --release"
                Write-Host ""
                Write-Host "  Run from project root:" -ForegroundColor Yellow
                Write-Host "    cargo build --release" -ForegroundColor White
                Write-Host "    powershell -ExecutionPolicy Bypass -File install\install.ps1" -ForegroundColor White
                exit 1
            }
        }
    }

    # 2. Create install directory and copy binary
    Log-Step "Installing to $InstallDir..."
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    Copy-Item -Path $binPath -Destination (Join-Path $InstallDir $BIN_NAME) -Force
    $installedBin = Join-Path $InstallDir $BIN_NAME
    Log-Ok "Copied $BIN_NAME to $InstallDir"

    # 3. Copy .tru example files
    Log-Step "Installing example .tru files..."
    $examplesDir = Join-Path $InstallDir "examples"
    if (-not (Test-Path $examplesDir)) {
        New-Item -ItemType Directory -Path $examplesDir -Force | Out-Null
    }
    $srcExamples = Join-Path $PSScriptRoot "..\examples"
    if (Test-Path $srcExamples) {
        Get-ChildItem "$srcExamples\*.tru" | ForEach-Object {
            Copy-Item $_.FullName -Destination $examplesDir -Force
        }
        Log-Ok "Examples installed to $examplesDir"
    }

    # 4. Create icon file (embedded base64 TRU logo .ico placeholder)
    Log-Step "Creating file type icon..."
    $iconPath = Join-Path $InstallDir "tru_file.ico"
    # 16x16 minimal .ico (valid ICO header + BMP, colored blue)
    $icoBytes = [Convert]::FromBase64String(
        "AAABAAEAEBAAAAEAIABoBAAAFgAAACgAAAAQAAAAIAAAAAEAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" +
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" +
        "AAAAAAD/AAAA/wAAAP8AAAD/AAAA/wAAAP8AAAD/AAAA/wAAAP8AAAD/AAAA/wAAAP8AAAD/AAAA/w" +
        "AAAP8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" +
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="
    )
    try {
        [System.IO.File]::WriteAllBytes($iconPath, $icoBytes)
        Log-Ok "Icon created: $iconPath"
    } catch {
        Log-Ok "Icon skipped (non-critical)"
        $iconPath = $installedBin  # fallback to exe icon
    }

    # 5. Register .tru extension in Windows Registry (HKCU — no admin needed)
    Log-Step "Registering .tru file extension..."

    # .tru → ProgID
    $extKey = "HKCU:\Software\Classes\$EXT"
    New-Item -Path $extKey -Force | Out-Null
    Set-ItemProperty -Path $extKey -Name "(Default)"         -Value $PROG_ID
    Set-ItemProperty -Path $extKey -Name "Content Type"      -Value "text/x-tru"
    Set-ItemProperty -Path $extKey -Name "PerceivedType"     -Value "text"
    Log-Ok "  $EXT -> $PROG_ID"

    # ProgID description
    $progKey = "HKCU:\Software\Classes\$PROG_ID"
    New-Item -Path $progKey -Force | Out-Null
    Set-ItemProperty -Path $progKey -Name "(Default)" -Value "TRU Language Source File"

    # DefaultIcon
    $iconKey = "$progKey\DefaultIcon"
    New-Item -Path $iconKey -Force | Out-Null
    Set-ItemProperty -Path $iconKey -Name "(Default)" -Value "`"$iconPath`",0"

    # shell\open\command  →  tru_id run "%1" %*
    $openKey = "$progKey\shell\open\command"
    New-Item -Path $openKey -Force | Out-Null
    Set-ItemProperty -Path $openKey -Name "(Default)" -Value "`"$installedBin`" run `"%1`" %*"
    Log-Ok "  open -> tru_id run `"%1`""

    # shell\run\command  (explicit "Run" verb)
    $runKey = "$progKey\shell\run\command"
    New-Item -Path $runKey -Force | Out-Null
    Set-ItemProperty -Path $runKey -Name "(Default)" -Value "`"$installedBin`" run `"%1`" %*"

    # shell\build\command  (transpile to .rs)
    $buildKey = "$progKey\shell\build"
    New-Item -Path $buildKey -Force | Out-Null
    Set-ItemProperty -Path $buildKey -Name "(Default)" -Value "Transpile to Rust (.rs)"
    $buildCmdKey = "$buildKey\command"
    New-Item -Path $buildCmdKey -Force | Out-Null
    Set-ItemProperty -Path $buildCmdKey -Name "(Default)" -Value "`"$installedBin`" build `"%1`""
    Log-Ok "  build -> tru_id build `"%1`""

    # shell\repl\command
    $replKey = "$progKey\shell\repl"
    New-Item -Path $replKey -Force | Out-Null
    Set-ItemProperty -Path $replKey -Name "(Default)" -Value "Open TRU REPL"
    $replCmdKey = "$replKey\command"
    New-Item -Path $replCmdKey -Force | Out-Null
    Set-ItemProperty -Path $replCmdKey -Name "(Default)" -Value "`"$installedBin`" repl"

    # shell\edit\command  →  open in default editor
    $editKey = "$progKey\shell\edit\command"
    New-Item -Path $editKey -Force | Out-Null
    Set-ItemProperty -Path $editKey -Name "(Default)" -Value "notepad.exe `"%1`""
    Log-Ok "  edit -> notepad `"%1`""

    # FileExts override
    $feKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\$EXT"
    New-Item -Path "$feKey\OpenWithProgids" -Force | Out-Null
    Set-ItemProperty -Path "$feKey\OpenWithProgids" -Name $PROG_ID -Value ([byte[]]@()) -Type Binary
    Log-Ok "  FileExts registered"

    # 6. Register MIME type
    Log-Step "Registering MIME type text/x-tru..."
    $mimeKey = "HKCU:\Software\Classes\MIME\Database\Content Type\text/x-tru"
    New-Item -Path $mimeKey -Force | Out-Null
    Set-ItemProperty -Path $mimeKey -Name "Extension" -Value $EXT
    Log-Ok "  MIME: text/x-tru -> $EXT"

    # 7. Add to PATH
    Log-Step "Adding $InstallDir to user PATH..."
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        $newPath = "$currentPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Log-Ok "Added to PATH"
    } else {
        Log-Ok "Already in PATH"
    }

    # 8. Notify Windows shell to refresh file associations
    Log-Step "Refreshing Windows shell..."
    $sig = '[DllImport("shell32.dll")] public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);'
    Add-Type -MemberDefinition $sig -Name "WinAPI" -Namespace "Shell32" -ErrorAction SilentlyContinue
    [Shell32.WinAPI]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)
    Log-Ok "Shell notified"

    # 9. Write an uninstall entry to Programs & Features
    Log-Step "Adding uninstall entry..."
    $uninstKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\tru_id"
    New-Item -Path $uninstKey -Force | Out-Null
    Set-ItemProperty -Path $uninstKey -Name "DisplayName"          -Value "TRU Language (tru_id_core)"
    Set-ItemProperty -Path $uninstKey -Name "DisplayVersion"       -Value $VERSION
    Set-ItemProperty -Path $uninstKey -Name "Publisher"            -Value "istamar"
    Set-ItemProperty -Path $uninstKey -Name "InstallLocation"      -Value $InstallDir
    Set-ItemProperty -Path $uninstKey -Name "UninstallString"      -Value "powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Uninstall"
    Set-ItemProperty -Path $uninstKey -Name "DisplayIcon"          -Value $iconPath
    Set-ItemProperty -Path $uninstKey -Name "NoModify"             -Value 1 -Type DWord
    Set-ItemProperty -Path $uninstKey -Name "NoRepair"             -Value 1 -Type DWord
    Log-Ok "Added to Apps & Features"

    # 10. Verify
    Log-Step "Verifying installation..."
    $verOut = & "$installedBin" version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Log-Ok "tru_id responds: $verOut"
    } else {
        Log-Err "Binary verification failed — extension registered but check your build"
    }

    # ─── Done ─────────────────────────────────────────────────────────────────
    Write-Host ""
    Write-Host "  ══════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "   TRU Language installed successfully!" -ForegroundColor Green
    Write-Host "  ══════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Install dir : $InstallDir" -ForegroundColor White
    Write-Host "  Binary      : $installedBin" -ForegroundColor White
    Write-Host "  Extension   : $EXT  (text/x-tru)" -ForegroundColor White
    Write-Host ""
    Write-Host "  Usage:" -ForegroundColor Yellow
    Write-Host "    tru_id run   examples\hello.tru" -ForegroundColor White
    Write-Host "    tru_id build myfile.tru          # -> myfile.rs" -ForegroundColor White
    Write-Host "    tru_id repl" -ForegroundColor White
    Write-Host ""
    Write-Host "  Right-click any .tru file for context menu actions." -ForegroundColor Gray
    Write-Host "  Restart Explorer or log off/on for full shell integration." -ForegroundColor Gray
    Write-Host ""
}

# ─── Entry ────────────────────────────────────────────────────────────────────

Require-Admin

if ($Uninstall) {
    Do-Uninstall
} else {
    Do-Install
}
