// PTYService.js

const os = require("os");
const pty = require("node-pty-prebuilt-multiarch");
const Queue = require("./queue.js");
const logger = require("./logger");

class PTY {
  constructor(socket) {
    // Setting default terminals based on user os
    this.shell = os.platform() === "win32" ? "powershell.exe" : "bash";
    this.ptyProcess = null;
    this.socket = socket;
    this.queue = new Queue(50);

    this.startPtyProcess();

    // Write text from the client to the terminal
    this.socket.on("input", (input) => this.write(input));
  }
  /**
   * Spawn an instance of pty with a selected shell.
   */
  startPtyProcess() {
    logger.info("terminalStarted");

    this.ptyProcess = pty.spawn(this.shell, [], {
      name: "xterm-color",
      cwd: process.env.HOME, // Which path should terminal start
      env: process.env, // Pass environment variables
    });

    // Add a "data" event listener.
    this.ptyProcess.onData((data) => {
      // Whenever terminal generates any data, send that output to socket.io client
      this.lastPrint = this.queue.append(data);
      this.socket.emit("output", data);
    });
  }

  /**
   * Use this function to send in the input to Pseudo Terminal process.
   * @param {*} data Input from user like a command sent from terminal UI
   */
  write(data) {
    this.ptyProcess.write(data);
  }
}

module.exports = PTY;
