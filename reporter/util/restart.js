// Require child_process
var exec = require("child_process").exec;
const ReporterSettings = require("../util/settings");
const logger = require("../util/logger");

// Create restart function
module.exports = function shutdown() {
  return new Promise((resolve) => {
    if (ReporterSettings.allowRestart) {
      switch (process.platform) {
        case "win32":
          var shutdownCommand = "shutdown /r";
          break;
        case "linux":
        case "darwin":
          var shutdownCommand = "sudo reboot";
          break;
      }
      logger.info("System restarting...");
      console.log("");
      exec(shutdownCommand, (error, stdout, stderr) => resolve(stdout));
    } else resolve();
  });
};
