#####################################################
# File       : install.ps1
# Author     : CelluloidRacer2 <celluloidracer2@gmail.com>
# Description: Script to install Xornet Reporter as a service
#####################################################

# Elevate the Powershell script to admin if it was run without it
if (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] 'Administrator')) {
  $CommandLine = "-File `"" + $MyInvocation.MyCommand.Path + "`" " + $MyInvocation.UnboundArguments
  Start-Process -FilePath PowerShell.exe -Verb Runas -ArgumentList $CommandLine
  Exit
}

# Iterate me with each release or things will break
$XornetVersion = "0.0.26"

$Logo = '
      ___           ___           ___           ___           ___           ___
     |\__\         /\  \         /\  \         /\__\         /\  \         /\  \
     |:|  |       /::\  \       /::\  \       /::|  |       /::\  \        \:\  \
     |:|  |      /:/\:\  \     /:/\:\  \     /:|:|  |      /:/\:\  \        \:\  \
     |:|__|__   /:/  \:\  \   /::\~\:\  \   /:/|:|  |__   /::\~\:\  \       /::\  \
 ____/::::\__\ /:/__/ \:\__\ /:/\:\ \:\__\ /:/ |:| /\__\ /:/\:\ \:\__\     /:/\:\__\
 \:::::/~~/    \:\  \ /:/  / \/_|::\/:/  / \/__|:|/:/  / \:\~\:\ \/__/    /:/  \/__/
  ~~|:|~~|      \:\  /:/  /     |:|::/  /      |:/:/  /   \:\ \:\__\     /:/  /
    |:|  |       \:\/:/  /      |:|\/__/       |::/  /     \:\ \/__/     \/__/
    |:|  |        \::/  /       |:|  |         /:/  /       \:\__\
     \|__|         \/__/         \|__|         \/__/         \/__/   Installer v0.1.1
'

Write-Host $Logo

$GithubUrl     = "https://github.com/xornet-cloud/Xornet/releases/download/v$XornetVersion/xornet-reporter-win-v$XornetVersion.zip"
$WinSWUrl      = "https://github.com/winsw/winsw/releases/latest/download/WinSW-x64.exe"
$InstallFolder = "C:\Program Files\Xornet\Reporter\"
$BinaryName    = "xornet-reporter-win-v$XornetVersion.exe"
$ZipName       = "xornet-reporter-win-v$XornetVersion.zip"

# Log variables to debug log
Write-Debug "Binary name:    $BinaryName"
Write-Debug "Zip name:       $ZipName"
Write-Debug "Install folder: $InstallFolder"
Write-Debug "Github URL:     $GithubUrl"
Write-Debug "WinSW URL:      $WinSWUrl"

# Check for existing installs
Write-Verbose "Checking for the existence of the Xornet program folder..."
if (Test-Path -Path $InstallFolder) {
    Write-Verbose "Found."
} else {
    Write-Verbose "Not found. Creating..."
    New-Item -ItemType Directory -Force -Path $InstallFolder
}

# Check for the reporter service and remove it if found
Write-Verbose "Looking for Xornet Reporter service..."
If ( Get-Service "Xornet Reporter" -ErrorAction SilentlyContinue ) {
  Write-Verbose "Xornet Reporter service found."

  If ( (Get-Service "Xornet Reporter").Status -eq 'Running' ) {
    Write-Host "Stopping Xornet Reporter service"
    Stop-Service "Xornet Reporter"
  } Else {
    Write-Debug "Xornet Reporter service found but not running."
  }

  Write-Host "Removing existing Xornet Reporter service..."

  # Delete the service & make sure no Xornet Reporter binaries are running
  Start-Process -FilePath "$InstallFolder/bin/XReporterSW.exe" -ArgumentList "uninstall" -Wait -NoNewWindow
  taskkill /F /IM xornet-reporter-win.exe >$null 2>&1
  taskkill /F /IM XReporterSW.exe >$null 2>&1

  Get-ChildItem -Path $InstallFolder -Include * -File -Recurse | foreach { $_.Delete()}
} Else {
    Write-Verbose "Xornet Reporter service not found"
}

# Supress command output
$ProgressPreference = "SilentlyContinue"

# Download + extract zip file
Write-host "Fetching Xornet Reporter v$XornetVersion from Github repo ($GithubUrl)..."
Invoke-WebRequest -Uri $GithubUrl -OutFile "$InstallFolder$ZipName"
Expand-Archive -LiteralPath "$InstallFolder\$ZipName" -DestinationPath "$InstallFolder" -Force
Write-Host "Downloaded Xornet Reporter binary."

# Download the service wrapper binary (WinSW). For platform support, we're downloading the .NET Core version here, however the binary is 16x larger then the .NET Framework v2/v4/v4.61 binary.
Write-host "Fetching WinSW from Github repo ($WinSWUrl)..."
Invoke-WebRequest -Uri $WinSWUrl -OutFile "$InstallFolder/bin/XReporterSW.exe"
Write-Host "Downloaded WinSW binary."

# Copy premade WinSW config
Copy-Item "$PSScriptRoot\XReporterSW.xml" -Destination "$InstallFolder/bin"
Write-Verbose "Copied service wrapper config"

# Give it the correct executable name
Write-Host "Reconfiguring WinSW..."
$SWConfig = New-Object System.Xml.XmlDocument
$SWConfig.Load("$InstallFolder\bin\XReporterSW.xml")
$SWConfigExecutable = $SWConfig.SelectSingleNode("//executable")
$SWConfigExecutable.InnerText = "$InstallFolder$BinaryName"
$SWConfig.save("$InstallFolder\bin\XReporterSW.xml")
Write-Debug "Reconfigured WinSW config to use '$InstallFolder$BinaryName' as executable path"

# Actually install the service
Write-Host "Installing service..."
Start-Process -FilePath "$InstallFolder/bin/XReporterSW.exe" -ArgumentList "install" -Wait -NoNewWindow

# Print ending messages
Write-Host "Service has been installed. If this is your first run, please run the binary manually for setup."
Write-Host "[Press any key to exit]";
$null = $Host.UI.RawUI.ReadKey("NoEcho, IncludeKeyDown");
