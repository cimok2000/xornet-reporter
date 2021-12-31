![Xornet Logo](https://cdn.discordapp.com/attachments/851974319370010655/854669456793534494/unknown.png)

# ⚡ How do I add my machine on Xornet?

1. Download the reporter through the [Releases](https://github.com/xornet-cloud/Reporter/releases/) for your platform
2. Run the reporter once with `./xornet-reporter.exe`
3. This will throw an error but it will generate a `config.json` where you ran it at
4. Fill in the "backend_hostname" field with `"backend.xornet.cloud"` or your own xornet backend if you're self-hosting
5. Go on Xornet and click the + button and copy the generated token
6. Signup your reporter with the token `./xornet-reporter.exe -su 61F14F509A1F4824B27ADDAC6EC9F510`
7. If the signup succeeds run the reporter with `--silent` after
8. Your machine should now show up on Xornet's dashboard

# Optional Steps
You can make a service on systemd to automatically start the reporter

1. Create the service file `sudo nano /etc/systemd/system/xornet.service`
2. Paste the following in the file:
```yaml
[Unit]
Description=Xornet Reporter
After=network.target

[Service]
Type=simple
User=$USER
Restart=always
RestartSec=3
WorkingDirectory=<xornet reporter binary path> # make sure your config.json is in this path so the reporter can see it
ExecStart=<xornet reporter binary path> --silent

[Install]
WantedBy=multi-user.target
```
3. Start the service `sudo systemctl start xornet`
4. Very that it works `sudo systemctl status xornet` 
5. If the service doesn't crash enable it so it starts automatically on startup with `sudo systemctl enable xornet`

# ⚡ One-shot Installation

## Linux

```bash
curl -s https://raw.githubusercontent.com/xornet-cloud/Reporter/main/scripts/install.sh | sudo bash
```

## Windows

```powershell
Invoke-Command -ScriptBlock ([Scriptblock]::Create((Invoke-WebRequest -UseBasicParsing 'https://raw.githubusercontent.com/xornet-cloud/Reporter/main/scripts/install.ps1').Content))
```

# Reporter

This is the data collector that gets your system's state and sends it to the backend, it can also be used as a pure system stat inspector without needing to connect it to Xornet

![Reporter Running](https://cdn.discordapp.com/attachments/911762334979084368/916844660369010718/unknown.png)

# Usage

![Help](https://cdn.discordapp.com/attachments/915215882232406037/917175896224432238/unknown.png)
