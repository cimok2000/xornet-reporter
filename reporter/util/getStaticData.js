const si = require("systeminformation");
const getLocation = require("../util/getLocation");
const ReporterSettings = require("./settings");
const uuidRegex = /([a-f0-9]{32})|([a-f0-9]{16})/;

module.exports = async function getStaticData() {
  return new Promise(async (resolve) => {
    const data = await si.getStaticData();
    data.geolocation = await getLocation();
    if(!uuidRegex.test(ReporterSettings.getUUID())) {
      ReporterSettings.setUUID(data.uuid.hardware.replace(/-/g, "") || data.uuid.os.replace(/-/g, ""));
      if (ReporterSettings.getUUID() == "03000200040005000006000700080009" || !uuidRegex.test(ReporterSettings.getUUID())) {
        ReporterSettings.setUUID(data.uuid.os.replace(/-/g, ""));
      }
    }
    resolve(data);
  });
};
