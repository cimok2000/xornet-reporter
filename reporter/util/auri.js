const { spawn } = require("child_process");
const fs = require("fs");

/**
 * Auri data collector handler for windows
 * This class handles the auri process and buffers the data from it
 * So we can easily use it in the getStats file
 * @author Niko Huuskonen
 */
class Auri {
  constructor() {
    const files = fs.readdirSync("./bin");
    for (const file of files) {
      if (file.startsWith("auri")) {
        // Start the process
        this.process = spawn(`./bin/${file}`, {
          windowsHide: true,
        });
      }
    }

    // Prepare an empty object to buffer the stuff in
    this.data = {};

    // Parse data coming from Auri
    this.process.stdout.on("data", (data) => (this.data = this.parseAuriData(data)));
  }

  parseAuriData(data) {
    data = data.toString();
    try {
      data = JSON.parse(data);
    } catch (error) {}
    return data;
  }

  get cpu() {
    return this.data.cpu.reduce((a, b) => a + b.load, 0) / this.data.cpu.length;
  }

  get cores() {
    return this.data.cpu;
  }

  get ram() {
    return this.data.ram;
  }

  get disks() {
    return this.data.disks;
  }

  get network() {
    return this.data.network;
  }

  get dataAll() {
    return ({ cpu, ram, disks, network } = this.data);
  }
}

module.exports = new Auri();
