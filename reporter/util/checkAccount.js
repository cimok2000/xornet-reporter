const axios = require("axios");
const logger = require("./logger");

module.exports = async function checkAccount(staticData, mute) {
  return new Promise(async (resolve) => {
    if (!mute) logger.info("accChk");
    try {
      let response = await axios.post(`${process.env.BACKEND_URL}/reporter`, {
        uuid: process.env.TEST_UUID || staticData.system.uuid,
      });

      if (!mute) logger.info(response.data.message);
      staticData.reporter = {
        linked_account: response.data.account_uuid,
      };
      if (!mute) logger.info("authCmp");
      resolve();
    } catch (error) {
      if (!mute) logger.warn("svrDn");
      if (error.response.status == 403) {
        if (!mute) logger.warn(["goToURL", [`${process.env.FRONTEND_URL}/dashboard/machines/all?newMachine=${staticData.system.uuid}`, "red"]]);
        setTimeout(() => {
          process.exit();
        }, 60000);
      }
    }
  });
};
