$ErrorActionPreference = "Stop"

try {
    # Check if padc.exe exists in the current directory
    if (-not (Test-Path -Path ".\..\padc.exe" -PathType Leaf)) {
        throw "Error: padc.exe program not found in the current directory"
    }

    # Generate completion script
    Write-Host "Generating padc.ps1 file..."
    .\..\padc.exe generate-completion powershell | Out-File padc.ps1 -Encoding utf8 -Force

    # Get target profile file path
    $targetProfile = $PROFILE.CurrentUserAllHosts

    # Ensure the profile directory exists
    $profileDir = Split-Path $targetProfile -Parent
    if (-not (Test-Path $profileDir)) {
        New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
    }

    # Copy the generated script to the profile file
    Write-Host "Installing to user profile file..."
    Copy-Item padc.ps1 $targetProfile -Force

    # Clean up temporary files (optional)
    # Remove-Item padc.ps1 -Force

    Write-Host "`nComplete! Auto-completion has been successfully installed to $targetProfile" -ForegroundColor Green
    Write-Host "Effective in new terminal sessions (restart PowerShell to use)"
}
catch {
    Write-Host "`nError: $_" -ForegroundColor Red
    exit 1
}

# Pause the script
Write-Host "`nPress any key to continue..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")