![Logo](https://cdn.discordapp.com/attachments/755597803102928966/931042317878587412/logo.svg)

# ⚡ How do I add my machine on Xornet?
1. Install the reporter with the scripts below or download it through the [Releases](https://github.com/xornet-cloud/Reporter/releases/) for your platform
2. Go on Xornet and click the + button and copy the generated token
3. Signup your reporter with the token `./xornet-reporter.exe -su 61F14F509A1F4824B27ADDAC6EC9F510`
4. If the signup succeeds run the reporter with `--silent` after
5. Your machine should now show up on Xornet's dashboard
6. (optional) if you wanna use your own backend change the "backend_hostname" field with `"backend.xornet.cloud"` to your own xornet backend

# ⚡ Installation

## Linux Service
```bash
curl https://raw.githubusercontent.com/xornet-cloud/Reporter/main/install.sh | sudo bash -s <signup token here>
```

## Windows Service
1. Download the windows version from the ![Releases](https://github.com/xornet-cloud/Reporter/releases)
2. Download ![nssm](https://cdn.discordapp.com/attachments/755597803102928966/933533332099190794/nssm.exe)
3. `nssm install` and install the service as shown below

![Example](https://cdn.discordapp.com/attachments/911762334979084368/931249917370957854/unknown.png)

4. Go on task manager > services > (Xornet Reporter) and click `Start`

# Reporter

This is the data collector that gets your system's state and sends it to the backend, it can also be used as a pure system stat inspector without needing to connect it to Xornet

![Reporter Running](https://cdn.discordapp.com/attachments/911762334979084368/916844660369010718/unknown.png)

# Usage

![Help](https://cdn.discordapp.com/attachments/915215882232406037/917175896224432238/unknown.png)
