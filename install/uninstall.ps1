# uninstall.ps1 — TRU Language (.tru) Uninstaller
# Run as Administrator:  powershell -ExecutionPolicy Bypass -File uninstall.ps1

#Requires -Version 5.1
param([switch]$Silent)

& "$PSScriptRoot\install.ps1" -Uninstall -Silent:$Silent
