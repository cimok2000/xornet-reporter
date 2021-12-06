function curl_check {
  echo "Checking for curl..."
  Invoke-WebRequest "https://curl.se/windows/dl-7.80.0/curl-7.80.0-win64-mingw.zip" -o curl.zip
  Expand-Archive curl.zip
  Remove-Item curl.zip
  Move-Item -Path ./curl/curl-7.80.0-win64-mingw/* -Destination ./curl
  Remove-Item ./curl/curl-7.80.0-win64-mingw
  New-Item -Path "C:/Program Files" -Name "curl" -ItemType "directory"
  Move-Item -Path ./curl/* -Destination "C:/Program Files/curl"
  Create-Item  
}

function wget_check {
  echo "Checking for wget..."
  
}

function delete_old {
  # sudo rm -f /usr/local/bin/xornet
  echo "Deleted old xornet installation"
}

function install {
  # curl --silent "https://api.github.com/repos/xornet-cloud/Reporter/releases/latest" \
  # | grep xornet-reporter.linux_x86_64 \
  # | grep browser_download_url \
  # | cut -d '"' -f 4 \
  # | sudo wget -O /usr/local/bin/xornet -i - \
  # && sudo chmod +x /usr/local/bin/xornet

  echo "Finished installing new version"
}

function main {
  curl_check
  wget_check
  delete_old
  install
  echo "Installtion finished"
}

main