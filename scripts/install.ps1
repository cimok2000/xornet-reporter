$SERVICE_NAME = "Xornet Reporter";

function print_logo {
  echo @'
  .,::      .:  ...    :::::::.. :::.    :::..,::::::::::::::::::
  `;;;,  .,;;.;;;;;;;. ;;;;``;;;;`;;;;,  `;;;;;;;'''';;;;;;;;''''
    '[[,,[[',[[     \[[,[[[,/[[['  [[[[[. '[[ [[cccc      [[     
     Y$$$P  $$$,     $$$$$$$$$c    $$$ "Y$c$$ $$""""      $$     
   oP"``"Yo,"888,_ _,88P888b "88bo,888    Y88 888oo,__    88,    
,m"       "Mm,"YMMMMMP" MMMM   "W" MMM     YM """"YUMMM   MMM   

v1.1.0

'@
}

function delete_old {
  if (Test-Path -Path "C:/Program Files/Xornet/xornet.exe") {
    Remove-Item -Recurse -Force -Confirm:$false "C:/Program Files/Xornet/xornet.exe" | Out-Null
    echo "Deleted old Xornet installation"
  }
}

function create_bin_folder {
  if (Test-Path -Path "C:/Program Files/Xornet") {
    echo "Destination folder exists already"
    return
  } else {
    New-Item -Path "C:/Program Files" -Name "Xornet" -ItemType "directory" | Out-Null
    echo "Created destination installation folder"
  }
}

function download_reporter {
  $XORNET_LATEST_RELEASES = "https://api.github.com/repos/xornet-cloud/Reporter/releases/latest"   
  $LATEST_RELEASES_JSON = ((Invoke-WebRequest -UseBasicParsing $XORNET_LATEST_RELEASES) | ConvertFrom-Json).assets.browser_download_url  
  $WINDOWS_DOWNLOAD_URL = $LATEST_RELEASES_JSON | Select-String -Pattern 'windows' -SimpleMatch

  $WINDOWS_DOWNLOAD_URL = $WINDOWS_DOWNLOAD_URL -replace '\s'

  Invoke-WebRequest -URI $WINDOWS_DOWNLOAD_URL -O "C:/Program Files/Xornet/xornet.exe"

  echo "Finished downloading Xornet Reporter latest"
}

function download_nssm {
  $NSSM_DOWNLOAD_URL = "https://cdn.discordapp.com/attachments/755597803102928966/933533332099190794/nssm.exe"
  Invoke-WebRequest -URI $NSSM_DOWNLOAD_URL -O "C:/Program Files/Xornet/nssm.exe"
  echo "Finished downloading NSSM"
}

function signup {
  if (Test-Path -Path "C:/Program Files/Xornet/config.json") {
    return
  } else {
    $signup_key = Read-Host -Prompt "Please enter your signup token: "
    cd "C:/Program Files/Xornet"
    ./xornet.exe -su $signup_key
  }
}

function check_if_service_exists {
  $result = & "C:/Program Files/Xornet/nssm.exe" get $SERVICE_NAME Name
  if ($result -eq $SERVICE_NAME){
    return 1
  }
  return 0;
}

function check_if_service_is_running {
  echo "Checking if theres a service already running"
  $service_state = & "C:/Program Files/Xornet/nssm.exe" status $SERVICE_NAME

  if ($service_state -eq "SERVICE_RUNNING") {
    return 1
  }

  return 0
}

function stop_xornet_service {
  echo "Stopping existing service (if any)"
  & "C:/Program Files/Xornet/nssm.exe" stop $SERVICE_NAME
}

function install_service {
  & "C:/Program Files/Xornet/nssm.exe" install $SERVICE_NAME 'C:/Program Files/Xornet/xornet.exe' --silent
  echo "Installed Xornet Reporter service sucessfully"
}

function start_service {
  & "C:/Program Files/Xornet/nssm.exe" start $SERVICE_NAME
  echo "Started Xornet Reporter Service"
}

function main {
  print_logo
  download_nssm
  $is_service_installed = check_if_service_exists -ErrorAction SilentlyContinue
  $is_running = check_if_service_is_running -ErrorAction SilentlyContinue
  if ($is_running -eq 1) {
    stop_xornet_service
  }
  delete_old
  create_bin_folder
  download_reporter 
  signup
  if ($is_service_installed -eq 0) {
    install_service
  }
  start_service
}

main