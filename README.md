![Logo](https://cdn.discordapp.com/attachments/755597803102928966/931042317878587412/logo.svg)

# ⚡ How do I add my machine on Xornet?

1. Go on Xornet and click the + button and copy the generated token
2. Run the installation script for your platform as noted below

# ⚡ Installation

## Linux Service

```bash
curl https://raw.githubusercontent.com/xornet-cloud/Reporter/main/scripts/install.sh | sudo bash
```

## Windows Service
1. Make sure you have [Powershell 7](https://www.microsoft.com/store/productId/9MZ1SNWT0N5D)
2. Run Powershell as Admin
```powershell
Invoke-Command -ScriptBlock ([Scriptblock]::Create((Invoke-WebRequest -UseBasicParsing 'https://raw.githubusercontent.com/xornet-cloud/Reporter/main/scripts/install.ps1').Content))
```
# Reporter

This is the data collector that gets your system's state and sends it to the backend, it can also be used as a pure system stat inspector without needing to connect it to Xornet

![Reporter Running](https://cdn.discordapp.com/attachments/911762334979084368/916844660369010718/unknown.png)

# Usage

![Help](https://cdn.discordapp.com/attachments/915215882232406037/917175896224432238/unknown.png)
