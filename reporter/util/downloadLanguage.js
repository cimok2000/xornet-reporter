const download = require("../util/download");
const axios = require("axios");
const ReporterSettings = require("./settings");
const locale = require("os-locale");
const fs = require("fs");

/**
 * Downloads the correct language from github based on the OS language.
 * @author cimok
 */
module.exports = async function downloadLanguage() {
  const lang = ReporterSettings.language === "auto" ? locale.sync() : ReporterSettings.language;
  const files = await fs.promises.readdir("./lang");
  for (file of files) {
    if (file.startsWith(lang)) {
      console.log(`${lang} already installed.`);
      return;
    }
  }
  const githubUrl = `https://api.github.com/repos/xornet-cloud/Xornet/contents/reporter/lang/${lang}.json`;
  const response = await axios.get(githubUrl);
  await download(response.data.download_url, "./lang");
  console.log(`${lang} installed. Please restart reporter for changes to take effect.`);
};