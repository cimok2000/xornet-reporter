![Xornet Logo](https://cdn.discordapp.com/attachments/851974319370010655/854669456793534494/unknown.png)

# ⚡ Installation

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
