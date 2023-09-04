const fs = require("fs");
const path = require("path");
const os = require("os");
const shell = require("shelljs");

const homeDir = os.homedir();
const platform = os.platform();
const arch = os.arch();
const packageJsonPath = path.join(__dirname, "package.json");
const packageJson = require(packageJsonPath);

let destinationDir;
let sourceFolder;

if (platform === "win32") {
  destinationDir = path.join(
    homeDir,
    "AppData",
    "Local",
    "Programs",
    "commitiq"
  );
  sourceFolder = "windows";
} else if (platform === "darwin") {
  destinationDir = path.join("/usr", "local", "bin");
  sourceFolder = arch === "arm64" ? "macos-m1" : "macos-intel";
} else {
  console.log("Currently, only Windows and MacOS are supported.");
  process.exit(1);
}

if (!fs.existsSync(destinationDir)) {
  fs.mkdirSync(destinationDir, { recursive: true });
}

packageJson["bin"] = { ciq: destinationPath };

const sourcePath = path.join(
  __dirname,
  "commitiq-cli",
  sourceFolder,
  "commitiq"
);
const destinationPath = path.join(destinationDir, "ciq");

fs.copyFileSync(sourcePath, destinationPath);

console.log(`CommitIQ has been installed to ${destinationDir}.`);

if (platform === "win32") {
  console.log(
    "On Windows, you may need to add the install location to your PATH environment variable."
  );
  console.log(`You can do this by adding ${destinationDir} to your PATH.`);
}

if (platform === "darwin") {
  shell.chmod("+x", destinationPath);
}
