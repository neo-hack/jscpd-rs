"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.uninstall = exports.install = exports.run = void 0;
var binary_install_1 = require("binary-install");
var os_1 = require("os");
var getPlatform = function () {
    var type = os_1.type();
    var arch = os_1.arch();
    /**
     * @todo support windows and linux
    */
    // if (type === "Windows_NT" && arch === "x64") {
    //   return "x86_64-pc-windows-msvc";
    // }
    // if (type === "Linux" && arch === "x64") {
    //   return "x86_64-unknown-linux-musl";
    // }
    if (type === "Darwin" && arch === "x64") {
        return "x86_64-apple-darwin";
    }
    throw new Error("Unsupported platform: " + type + " " + arch);
};
var getBinary = function () {
    var platform = getPlatform();
    var version = require('../package.json').version;
    var author = "spring-catponents";
    var name = "jscpd-rs";
    var url = "https://github.com/" + author + "/" + name + "/releases/download/v" + version + "/" + name + "-v" + version + "-" + platform + ".tar.gz";
    return new binary_install_1.Binary(url, { name: name });
};
var run = function () {
    var binary = getBinary();
    binary.run();
};
exports.run = run;
var install = function () {
    var binary = getBinary();
    binary.install();
};
exports.install = install;
var uninstall = function () {
    var binary = getBinary();
    binary.uninstall();
};
exports.uninstall = uninstall;
