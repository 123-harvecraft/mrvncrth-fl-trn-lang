@echo off
:: install.bat — TRU Language (.tru) Quick Installer
:: Double-click or run from any command prompt.
:: Builds the project first if binaries are not found.

title TRU Language Installer

echo.
echo   ==========================================
echo    TRU Language (.tru) Installer
echo    Crate: tru_id_core
echo   ==========================================
echo.

:: ── Locate project root (one level up from install\) ─────────────────────────
set "PROJ_DIR=%~dp0.."
pushd "%PROJ_DIR%"

:: ── Check for cargo ───────────────────────────────────────────────────────────
where cargo >nul 2>&1
if errorlevel 1 (
    echo   [ERR] cargo not found. Install Rust from https://rustup.rs
    pause
    exit /b 1
)

:: ── Build release binaries if not present ────────────────────────────────────
set "BIN_DIR=%PROJ_DIR%\target\release"
set "TRU_BIN=%BIN_DIR%\tru_id.exe"
set "INST_BIN=%BIN_DIR%\tru_id_installer.exe"

if not exist "%TRU_BIN%" (
    echo   [..] Building tru_id_core in release mode ...
    cargo build --release --bin tru_id --bin tru_id_installer
    if errorlevel 1 (
        echo   [ERR] Build failed. Check errors above.
        pause
        exit /b 1
    )
    echo   [OK] Build complete.
) else (
    echo   [OK] Release binaries already present.
)

:: ── Run Rust installer ────────────────────────────────────────────────────────
if exist "%INST_BIN%" (
    echo   [..] Running tru_id_installer ...
    echo.
    "%INST_BIN%" install
    if errorlevel 1 (
        echo.
        echo   [WARN] Rust installer failed, falling back to PowerShell installer...
        goto :ps_fallback
    )
    goto :done
)

:ps_fallback
:: ── PowerShell fallback ───────────────────────────────────────────────────────
echo   [..] Running PowerShell installer ...
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1"
if errorlevel 1 (
    echo.
    echo   [ERR] Installation failed. Try running install.ps1 as Administrator.
    pause
    exit /b 1
)

:done
popd
echo.
echo   ==========================================
echo    Done! Open a new terminal and try:
echo      tru_id run examples\hello.tru
echo      tru_id repl
echo   ==========================================
echo.
pause
