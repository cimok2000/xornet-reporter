// Require child_process
var exec = require("child_process").exec;
const {settings:ReporterSettings} = require("../util/settings");
const logger = require("../util/logger");

// Create shutdown function
module.exports = function shutdown() {
  return new Promise((resolve) => {
    if (ReporterSettings.allowShutdown) {
      switch (process.platform) {
        case "win32":
          var shutdownCommand = "shutdown /s";
          break;
        case "linux":
        case "darwin":
          var shutdownCommand = "sudo shutdown";
          break;
      }
      logger.info("System shutting down...");
      console.log("");
      exec(shutdownCommand, (error, stdout, stderr) => resolve(stdout));
    } else resolve();
  });
};
