const fs = require("fs");
const path = require("path");
const os = require("os");
const shell = require("shelljs");

const homeDir = os.homedir();

const isWindows = os.platform() === "win32";

let destinationDir;
if (isWindows) {
  destinationDir = path.join(
    homeDir,
    "AppData",
    "Local",
    "Programs",
    "commitiq"
  );
} else {
  destinationDir = path.join("/usr", "local", "bin");
}

if (!fs.existsSync(destinationDir)) {
  fs.mkdirSync(destinationDir, { recursive: true });
}

console.log(__dirname);
const sourcePath = path.join(
  __dirname,
  "commitiq-cli",
  "target",
  "release",
  "commitiq"
);
const destinationPath = path.join(destinationDir, "ciq");

fs.copyFileSync(sourcePath, destinationPath);

console.log(`CommitIQ has been installed to ${destinationDir}.`);

if (isWindows) {
  console.log(
    "On Windows, you may need to add the install location to your PATH environment variable."
  );
  console.log(`You can do this by adding ${destinationDir} to your PATH.`);
}

if (!isWindows) {
  shell.chmod("+x", destinationPath);
}
