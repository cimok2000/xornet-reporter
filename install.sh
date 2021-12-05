rm -f /bin/xornet

curl --silent "https://api.github.com/repos/xornet-cloud/Reporter/releases/latest" | grep xornet-reporter.linux_x86_64  | grep browser_download_url  | cut -d '"' -f 4  | sudo wget -o /bin/xornet -i -