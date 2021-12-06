function delete_old {
  if (Test-Path -Path "C:/Users/$env:UserName/Downloads/bin/xornet.exe") {
    Remove-Item -Recurse -Force -Confirm:$false "C:/Users/$env:UserName/Downloads/bin/xornet.exe"
    echo "Deleted old xornet installation"
  }
}

function create_bin_folder {
  if ( Test-Path -Path "C:/Users/$env:UserName/Downloads/bin") {
    return
  } else {
    New-Item -Path "C:/Users/$env:UserName/Downloads" -Name "bin" -ItemType "directory"
    echo "Created destination bin folder"
  }
}

function install {

  create_bin_folder

  $XORNET_LATEST_RELEASES = "https://api.github.com/repos/xornet-cloud/Reporter/releases/latest"   
  $LATEST_RELEASES_JSON = ((Invoke-WebRequest -UseBasicParsing $XORNET_LATEST_RELEASES) | ConvertFrom-Json).assets.browser_download_url  
  $WINDOWS_DOWNLOAD_URL = $LATEST_RELEASES_JSON | Select-String -Pattern 'windows' -SimpleMatch

  $WINDOWS_DOWNLOAD_URL = $WINDOWS_DOWNLOAD_URL -replace '\s'

  Invoke-WebRequest -URI $WINDOWS_DOWNLOAD_URL -O "C:/Users/$env:UserName/Downloads/bin/xornet.exe"
  Set-Item -Path Env:Path -Value ($Env:Path + ";C:/Users/$env:UserName/Downloads/bin/")

  echo "Finished installing new version"
}

function main {
  delete_old
  install
}

main