const fs = require("fs");
const colors = require("colors/safe");

class Settings {
  constructor() {
    const dir = fs.readdirSync("./");
    dir.includes("settings.json") ? (this.settings = JSON.parse(fs.readFileSync(`${process.cwd()}/settings.json`))) : this.createNewSettings();
  }

  createNewSettings() {
    this.settings = {
      language: "auto",
      verbose: true,
      speedtests: false,
      allowRestart: false,
      allowShutdown: false,
      allowTerminal: false,
      uuid: null,
    };
    this.save();
  }

  save() {
    fs.writeFileSync("settings.json", JSON.stringify(this.settings, null, 2));
  }

  getUUID() {
    return this.settings.uuid;
  }

  setUUID(uuid) {
    this.settings.uuid = uuid;
    this.save();
  }

  displaySettings() {
    for(let setting in this.settings) {
      //T his is the most scuffed code ever, fix it someone who knows how to do uppercase stuff
      let capitalized = setting.charAt(0).toUpperCase() + setting.slice(1);
      console.log(`${colors.bgYellow(`[SETTINGS]`).black} ${capitalized}: ${this.settings[setting]}`);
    }
  }
}

const settings = new Settings();

module.exports = settings;
