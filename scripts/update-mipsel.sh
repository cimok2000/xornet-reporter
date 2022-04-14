cp /tmp/config.json /root || true
wget https://github.com/xornet-cloud/Reporter/releases/latest/download/xornet-reporter.mipsel-unknown-linux-musl -O /tmp/xornet-reporter 
chmod +x /tmp/xornet-reporter
cp /root/config.json /tmp || true
tmux new-session -d
tmux send-keys -t 0 "/tmp/xornet-reporter" Enter

