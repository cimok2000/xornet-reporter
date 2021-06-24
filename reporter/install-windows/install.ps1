# Elevate the Powershell script to admin if it was run without it
if (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] 'Administrator')) {
  $CommandLine = "-File `"" + $MyInvocation.MyCommand.Path + "`" " + $MyInvocation.UnboundArguments
  Start-Process -FilePath PowerShell.exe -Verb Runas -ArgumentList $CommandLine
  Exit
}

# Iterate me with each release or things will break
$XornetVersion = "0.0.23"

$Logo = '
      ___           ___           ___           ___           ___           ___
     |\__\         /\  \         /\  \         /\__\         /\  \         /\  \
     |:|  |       /::\  \       /::\  \       /::|  |       /::\  \        \:\  \
     |:|  |      /:/\:\  \     /:/\:\  \     /:|:|  |      /:/\:\  \        \:\  \
     |:|__|__   /:/  \:\  \   /::\~\:\  \   /:/|:|  |__   /::\~\:\  \       /::\  \
 ____/::::\__\ /:/__/ \:\__\ /:/\:\ \:\__\ /:/ |:| /\__\ /:/\:\ \:\__\     /:/\:\__\
 \:::::/~~/~   \:\  \ /:/  / \/_|::\/:/  / \/__|:|/:/  / \:\~\:\ \/__/    /:/  \/__/
  ~~|:|~~|      \:\  /:/  /     |:|::/  /      |:/:/  /   \:\ \:\__\     /:/  /
    |:|  |       \:\/:/  /      |:|\/__/       |::/  /     \:\ \/__/     \/__/
    |:|  |        \::/  /       |:|  |         /:/  /       \:\__\
     \|__|         \/__/         \|__|         \/__/         \/__/   Installer v0.0.1
'

Write-Host $Logo

$GithubUrl     = "https://github.com/xornet-cloud/Xornet/releases/download/v$XornetVersion/xornet-reporter-win-v$XornetVersion.zip"
$InstallFolder = "C:\Program Files\Xornet\Reporter\"
$BinaryName    = "xornet-reporter-win-v$XornetVersion.exe"
$ZipName       = "xornet-reporter-win-v$XornetVersion.zip"

# Log variables to debug log
Write-Debug "Binary name:    $BinaryName"
Write-Debug "Zip name:       $ZipName"
Write-Debug "Install folder: $InstallFolder"
Write-Debug "Github URL:     $GithubUrl"

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
  Write-Host "Killing interfering processes..."
  # Kill processes that can prevent the service deletion from occuring immediately. Surpress errors
  taskkill /F /IM mmc.exe >$null 2>&1
  taskkill /F /IM taskmgr.exe >$null 2>&1

  # Use sc to delete the service
  sc.exe delete "Xornet Reporter" >$null 2>&1

  # This scriptlet was introduced in Powershell v6, which is not distributed by default on Windows 10/Server 2019
  #Remove-Service -Name "Xornet Reporter"

  Get-ChildItem -Path $InstallFolder -Include * -File -Recurse | foreach { $_.Delete()}
} Else {
    Write-Verbose "Xornet Reporter service not found"
}

Write-host "Fetching Xornet Reporter v$XornetVersion from Github repo ($GithubUrl)..."

# Supress command output
$ProgressPreference = "SilentlyContinue"

# Download + extract zip file
Invoke-WebRequest -Uri $GithubUrl -OutFile "$InstallFolder$ZipName"
Expand-Archive -LiteralPath "$InstallFolder\$ZipName" -DestinationPath "$InstallFolder"

Write-Host "Downloaded Xornet Reporter binary."

# Actually install the service
Write-Host "Installing service..."
Try { New-Service -Name "Xornet Reporter" -BinaryPathName "$InstallFolder$BinaryName" -StartupType "Automatic" }
Catch {
  Write-Host "An error occurred:"
  Write-Host $_.ScriptStackTrace
}

# Print ending messages
Write-Host "Service has been installed. If this is your first run, please run the binary manually for setup."
Write-Host "[Press any key to exit]";
$null = $Host.UI.RawUI.ReadKey("NoEcho, IncludeKeyDown");
