const fs = require("fs");

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
      uuid: null,
    };
    this.save();
  }

  save() {
    fs.writeFileSync("settings.json", JSON.stringify(this.settings));
  }

  getUUID() {
    return this.settings.uuid;
  }

  setUUID(uuid) {
    this.settings.uuid = uuid;
    this.save();
  }
}

const settings = new Settings();

module.exports = settings;
