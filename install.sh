
curl_check () {
  echo "Checking for curl..."
  if command -v curl > /dev/null; then
    echo "Detected curl..."
  else
    echo "Installing curl..."
    apt install -q -y curl
    if [ "$?" -ne "0" ]; then
      echo "Unable to install curl! Your base system has a problem; please check your default OS's package repositories because curl should work."
      echo "Repository installation aborted."
      exit 1
    fi
  fi
}

wget_check () {
  echo "Checking for wget..."
  if command -v wget > /dev/null; then
    echo "Detected wget..."
  else
    echo "Installing wget..."
    apt install -q -y wget
    if [ "$?" -ne "0" ]; then
      echo "Unable to install wget! Your base system has a problem; please check your default OS's package repositories because curl should work."
      echo "Repository installation aborted."
      exit 1
    fi
  fi
}


delete_old() {
  sudo rm -f /bin/xornet
  echo "Deleted old xornet installation"
}


install(){
  curl --silent "https://api.github.com/repos/xornet-cloud/Reporter/releases/latest"  \
  | grep xornet-reporter.linux_x86_64  \
  | grep browser_download_url  \
  | cut -d '"' -f 4  \
  | sudo wget -i - -o /bin/xornet  \

  echo "Finished installing new version"
}


main() {
  curl_check
  delete_old
  install
  echo "Installtion finished"
}

main
