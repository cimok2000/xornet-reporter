const settings = require("./settings");
const os = require("os");
if (os.platform() === "win32") {
  var auri = require("./auri.js");
} else {
  var si = require("systeminformation");
}

/**
 * Collects all the statistics from the system and returns an Object.
 * @returns {Object} with all stats for the report
 */
module.exports = async function getStats(staticData) {
  const hostname = os.hostname();
  const platform = os.platform();

  const mainData = async () => {
    if (platform == "win32") {
      return {
        ram: auri.ram,
        cpu: auri.cpu,
        cores: auri.cores,
        network: auri.network,
        disks: auri.disks,
      };
    }

    const data = await si.get({
      networkStats: `(*) tx_sec, rx_sec`,
      cpu: `cores`,
    });

    const currentLoad = await si.currentLoad();

    return {
      ram: {
        total: os.totalmem(),
        free: os.freemem(),
      },
      cpu: currentLoad.currentLoad,
      cores: currentLoad.cpus.map((core) => core.load),
      network: data.networkStats,
      disks: await si.fsSize(),
    };
  };

  const stats = {
    uuid: settings.getUUID(),
    isVirtual: staticData.system.virtual,
    hostname,
    platform,
    ...(await mainData()),
    reporterVersion: require("../package.json").version,
    uptime: os.uptime(),
    reporterUptime: Date.now() - parseInt(process.env.STARTTIME),
    timestamp: Date.now(),
  };
  return stats;
};
