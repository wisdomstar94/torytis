const { Binary } = require("binary-install");
const os = require("os");
const path = require("path");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  const platformInfo = {
    arch: "",
    type: "",
  };

  switch (type) {
    case "Windows_NT":
      {
        platformInfo.type = "pc-windows-msvc";
      }
      break;
    case "Darwin":
      {
        platformInfo.type = "apple-darwin";
      }
      break;
    case "Linux":
      {
        platformInfo.type = "linux-gnu";
      }
      break;
  }

  if (["arm", "arm64"].includes(arch)) {
    platformInfo.arch = "aarch64";
  } else {
    platformInfo.arch = "x86_64";
  }

  return platformInfo;
}

function getBinary() {
  const platform = getPlatform();
  const version = require("../package.json").version;
  // const url = `https://github.com/wisdomstar94/torytis/releases/download/v${version}/torytis-${platform.arch}-${platform.type}.tar.gz`;
  const url = `https://github.com/wisdomstar94/torytis/releases/download/torytis-v${version}/torytis-${platform.arch}-${platform.type}.tar.gz`;
  const name = "torytis";
  const installDirectory = path.join(__dirname, "..", "bin");
  return new Binary(name, url, {
    installDirectory,
  });
}

module.exports = getBinary;
