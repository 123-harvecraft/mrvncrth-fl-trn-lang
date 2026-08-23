@echo off
:: uninstall.bat — TRU Language (.tru) Uninstaller

title TRU Language Uninstaller

echo.
echo   ==========================================
echo    TRU Language (.tru) Uninstaller
echo   ==========================================
echo.

set "PROJ_DIR=%~dp0.."
set "INST_BIN=%PROJ_DIR%\target\release\tru_id_installer.exe"

if exist "%INST_BIN%" (
    echo   [..] Running tru_id_installer uninstall ...
    "%INST_BIN%" uninstall
    goto :done
)

echo   [..] Running PowerShell uninstaller ...
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0uninstall.ps1"

:done
echo.
echo   [OK] TRU Language uninstalled.
echo.
pause
