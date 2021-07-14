require("colors");
require("./util/printLogo");
const ReporterSettings = require("./util/settings");
const si = require("systeminformation");
const isSpeedtestInstalled = require("./util/isSpeedtestInstalled");
const installSpeedtest = require("./util/installSpeedtest");
const connectToXornet = require("./util/connectToXornet");
const getStats = require("./util/getStats");
const getStaticData = require("./util/getStaticData");
const speedtest = require("./util/speedtest");
const logger = require("./util/logger");
const downloadLanguage = require("./util/downloadLanguage");
process.env.REFRESH_INTERVAL = 1000;
process.env.BACKEND_URL = process.env.NODE_ENV.trim() === "development" ? "http://localhost:8080" : "https://backend.xornet.cloud";
process.env.FRONTEND_URL = process.env.NODE_ENV.trim() === "development" ? "http://localhost:8082" : "https://xornet.cloud";
process.env.BACKEND_WS_URL = process.env.NODE_ENV.trim() === "development" ? "ws://localhost:8080" : "wss://backend.xornet.cloud";
process.env.STARTTIME = Date.now();
process.env.PRINT_SENDING_STATS = true;

const PTYService = require("./util/PTYService");

async function main() {
  // This will not work in the compiled version for some reason.
  // PKG seems to save what happens before compiling so when I change
  // the 'settings.json' language it still uses what was previously set
  // before compiling. This is a bad bug because it prevents this
  // from working at all. On uncompiled it works well.
  // await downloadLanguage();
  logger.info("fetch");
  const staticData = await getStaticData();
  logger.info("sysInf", "green");

  logger.test("instChk");
  if (!(await isSpeedtestInstalled())) {
    logger.test("noTest");
    await installSpeedtest();
  }
  logger.test("isTest");

  // Connect to xornet
  const xornet = await connectToXornet(staticData);

  let statistics = {};
  setInterval(async () => {
    // Send data on a loop
    statistics = await getStats(staticData);
  }, process.env.REFRESH_INTERVAL);

  let emitter = null;

  xornet.on("connect", () => {
    logger.net(["con", [process.env.BACKEND_URL, "green"]]);
    logger.info("load");

    emitter = setInterval(function () {
      if (process.env.PRINT_SENDING_STATS === "true") {
        logger.info([
          ["send", "cyan"],
          [Date.now(), "cyan"],
          [`- ${ReporterSettings.getUUID()}`, "cyan"],
        ]);
      }
      xornet.emit("report", statistics);
    }, process.env.REFRESH_INTERVAL);
  });

  // Warns the user if the reporter disconnects from the Xornet Backend.
  // Clears the emitters interval so that the reporter does not send any data until it reconnects.
  xornet.on("disconnect", async () => {
    logger.net(["dis", [process.env.BACKEND_URL, "red"]]);
    clearInterval(emitter);
  });

  // Get a heartbeat from the backend and send a heartbeat response back with UUID.
  // Returns a response with the systems UUID which is then used later to calculate the ping.
  xornet.on("heartbeat", async (epoch) => {
    xornet.emit("heartbeatResponse", {
      uuid: process.env.TEST_UUID || ReporterSettings.getUUID(),
      epoch,
    });
  });

  // Get a event to run a speedtest
  // Returns a response with the results of the speedtest
  xornet.on("runSpeedtest", async () => xornet.emit("speedtest", await speedtest()));
  xornet.on("getProcesses", async () => xornet.emit("processes", await si.processes()));
  xornet.on("shutdown", async () => await require("./util/shutdown")());
  xornet.on("restart", async () => await require("./util/restart")());

  // Check if terminals are enabled in the settings.json
  if (ReporterSettings.settings.allowTerminal) var pseudoTerminal = new PTYService(xornet);

  xornet.on("startTerminal", async () => {
    let queue = pseudoTerminal.queue.peek();
    queue.forEach(line => xornet.emit("output", line));
  });
}

main();
