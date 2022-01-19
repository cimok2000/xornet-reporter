#!/usr/bin/env bash
set +x
trap handle_ctrl_c INT
path=$(pwd)

# Cleanup
cleanup() {
  echo "Cleaning up..."
  cd $path
  if [ -f xornet-reporter ]; then
    rm xornet-reporter
  fi
  echo "Ok."
}

# Handle exit code for command
handle_exit_code() {
  if [ $? -ne 0 ]; then
    echo "Installation failed."
    echo "Please check the logs for any errors."
    cleanup
    exit
  else
    echo "Ok."
    echo
  fi
}

# Handle exit code for command (doesn't exit)
handle_exit_code_non_crucial() {
  if [ $? -ne 0 ]; then
    echo "Previous command failed with exit code $?."
    echo "Please check the logs for any errors."
    echo
  else
    echo "Ok."
    echo
  fi
}

# Ctrl-C
handle_ctrl_c() {
  echo "CTRL-C detected."
  cleanup
  exit
}

echo "Xornet reporter install script v1.0.0"
echo "--------------------------------------"
echo "This script will install and set up Xornet reporter for your system."
echo
echo "Checking for root privileges..."
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script as root"
  exit 1
fi
echo "Ok."
echo
echo "Checking CPU architecture..."
arch=$(uname -m)
# TODO: Add support for lowercase
case $arch in
  armv7*)
    arch="armv7"
    ;;
  *)
    arch=$arch
    ;;
esac
echo $arch
echo
echo "Checking for required apps for the script to run..."
if [ ! -f /usr/bin/curl ]; then
  echo "Curl is not installed or not in PATH. Please install it and try again."
  exit 1
fi
echo "Curl is installed."

if [ ! -f /usr/bin/systemctl ]; then
  echo "Systemd is not installed. This script cannot create the systemd service."
  systemctl_installed=false
else
  echo "Systemd is installed."
  systemctl_installed=true
fi
echo "Ok."
echo

echo "Checking for existing Xornet reporter installation..."
if [ -d /opt/xornet ]; then
  echo "Xornet reporter is already installed."
  ready=false
  while [ $ready = false ]; do
    echo "Please choose one of the following options:"
    echo "1. Clean install Xornet reporter"
    echo "2. Update existing Xornet reporter"
    echo "3. Exit"
    read -p "Please enter your choice: " choice </dev/tty
    echo
    case $choice in
    1)
      echo "Uninstalling old Xornet reporter entirely..."
      if [ $systemctl_installed = true ]; then
        echo "Stopping Xornet reporter service..."
        systemctl stop xornet-reporter.service
        handle_exit_code_non_crucial
        echo "Disabling Xornet reporter service..."
        systemctl disable xornet-reporter.service
        handle_exit_code_non_crucial
        echo "Removing Xornet reporter service..."
        rm /etc/systemd/system/xornet-reporter.service
        handle_exit_code_non_crucial
      fi
      echo "Removing old Xornet reporter installation directory..."
      rm -rf /opt/xornet
      handle_exit_code
      echo "Creating new Xornet reporter installation directory..."
      mkdir /opt/xornet
      handle_exit_code
      ready=true
      ;;
    2)
      echo "Updating Xornet reporter..."
      if [ $systemctl_installed = true ]; then
        echo "Stopping Xornet reporter service..."
        systemctl stop xornet-reporter.service
        handle_exit_code_non_crucial
      fi
      echo "Removing old Xornet reporter executable..."
      rm -rf /opt/xornet/xornet-reporter
      handle_exit_code_non_crucial
      ready=true
      ;;
    3)
      echo "Exiting..."
      exit
      ;;
    *)
      echo "Invalid choice. Try again."
      echo
      ;;
    esac
  done
else
  echo "Xornet reporter is not installed. Proceeding with installation..."
  echo "Creating directory /opt/xornet..."
  mkdir /opt/xornet
  handle_exit_code
fi

# Download the latest release from github and extract it
echo "Downloading the latest release from github..."
download_url=$(curl -s https://api.github.com/repos/xornet-cloud/Reporter/releases | grep browser_download_url | grep "${arch}-" | head -n 1 | cut -d '"' -f 4)
curl -L $download_url -o xornet-reporter
handle_exit_code
echo "Moving the executable into /opt/xornet..."
mv xornet-reporter /opt/xornet
handle_exit_code
echo "Flagging as executable..."
chmod +x /opt/xornet/xornet-reporter
handle_exit_code

if [ ! -f /opt/xornet/config.json ]; then
  cd /opt/xornet
  echo "Setting up as a new installation..."
  if [ ! $1 ]; then
    read -p "Please enter your Xornet Signup Token: " token </dev/tty
  else
    token=$1
  fi
  ./xornet-reporter -su $token
  handle_exit_code
  cd $path
fi

if [ $systemctl_installed = true ]; then
  if [ ! -f /etc/systemd/system/xornet-reporter.service ]; then

    # Create the systemd service
    echo "Creating the systemd service..."
    cat >/etc/systemd/system/xornet-reporter.service <<EOF
[Unit]
Description=Xornet reporter
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/xornet
ExecStart=/opt/xornet/xornet-reporter --silent
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF
    echo "Ok."
    echo

    # Enable the service
    echo "Enabling the systemd service..."
    systemctl enable xornet-reporter.service
    echo "Ok."
    echo
  fi

  # Start the service
  echo "Starting the systemd service..."
  systemctl start xornet-reporter.service
  echo "Ok."
  echo
fi
cleanup

echo "Installation complete. Xornet reporter is now installed."
echo "Please check the logs for any errors."
if [ $systemctl_installed = true ]; then
  echo "You can stop the service by running the following command:"
  echo "systemctl stop xornet-reporter.service"
else
  echo "Systemd is not installed. No background service was created."
  echo "You can start the reporter by running the following command:"
  echo "sudo /opt/xornet/xornet-reporter --silent"
fi
