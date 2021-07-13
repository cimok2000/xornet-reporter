// PTYService.js

const os = require("os");
const pty = require("node-pty-prebuilt-multiarch");

class PTY {
  constructor() {
    // Setting default terminals based on user os
    this.shell = os.platform() === "win32" ? "powershell.exe" : "bash";
    this.ptyProcess = null;
    this.socket = null;
  }

  start(socket){
    this.socket = socket;

    // Write text from the client to the terminal
    this.socket.on("input", input => this.write(input));
    this.socket.on("terminateTerminal", () => this.killProcess());

    // Initialize PTY process.
    this.startPtyProcess();
  } 

  /**
   * Spawn an instance of pty with a selected shell.
   */
  startPtyProcess() {

    console.log("Psuedoterminal connection initialized");

    this.ptyProcess = pty.spawn(this.shell, [], {
      name: "xterm-color",
      cwd: process.env.HOME, // Which path should terminal start
      env: process.env, // Pass environment variables
    });

    // Add a "data" event listener.
    this.ptyProcess.onData((data) => {
      // Whenever terminal generates any data, send that output to socket.io client
      this.sendToClient(data);
    });
  }

  /**
   * Use this function to send in the input to Pseudo Terminal process.
   * @param {*} data Input from user like a command sent from terminal UI
   */

  killProcess() {
    console.log("Psuedoterminal connection closed");
    this.ptyProcess?.kill();
    this.ptyProcess = null;
    console.log(this.ptyProcess);
    this.socket.off("input");
    this.socket.off("terminateTerminal");
  }

  write(data) {
    this.ptyProcess.write(data);
  }

  sendToClient(data) {
    // Emit data to socket.io client in an event "output"
    this.socket.emit("output", data);
  }
}

module.exports = new PTY;
