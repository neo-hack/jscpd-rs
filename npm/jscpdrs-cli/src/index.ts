import { Binary } from 'binary-install'
import { type as Type, arch as Arch } from 'os'


const getPlatform = () => {
  const type = Type();
  const arch = Arch();

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

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
  const platform = getPlatform();
  const version = require('../package.json').jscpdrsVersion
  const author = "spring-catponents";
  const name = "jscpd-rs";
  const binaryName = "jscpdrs";
  const url = `https://github.com.cnpmjs.org/${author}/${name}/releases/download/v${version}/${binaryName}-${platform}.tar.gz`;
  return new Binary(`${binaryName}-cli`, url);
};

export const run = () => {
  const binary = getBinary();
  binary.run();
};

export const install = () => {
  const binary = getBinary();
  binary.install();
};

export const uninstall = () => {
  const binary = getBinary();
  binary.uninstall();
};