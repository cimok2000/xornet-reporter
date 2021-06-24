const download = require("../util/download");
const locale = require("os-locale");
const ReporterSettings = require("./settings");

/**
 * Downloads the correct language from the backend based on the OS language.
 * @author cimok
 */
module.exports = async function downloadLanguage() {
    const systemLocale = ReporterSettings.language === "auto" ? locale.sync() : ReporterSettings.language;
    await download(`http://localhost:8080/reporterLanguages/${systemLocale}.json`);
};
