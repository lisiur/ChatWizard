const fs = require("fs");
const path = require("path");
const toml = require("@iarna/toml");
const changeLogVersionPackage = require("./package.json");
const tauriConf = require("./gui/tauri.conf.json");
const webPackage = require("./web/package.json");
const { execSync } = require("child_process");

const commands = {
  patch,
  minor,
  major,
  release,
  publish,
};

function main() {
  const command = process.argv[2];
  if (!command) {
    console.log("Usage: node release.js [patch|minor|major]");
    process.exit(1);
  }
  if (command in commands) {
    commands[command]();
  } else {
    console.error("Invalid command");
    process.exit(1);
  }
}

function patch() {
  bumpVersion(patchVersion);
}

function minor() {
  bumpVersion(minorVersion);
}

function major() {
  bumpVersion(majorVersion);
}

function release() {
  generateReleaseNote();
  const newVersion = changeLogVersionPackage.version;
  execSync("git add .");
  execSync(`git commit -m "chore: bump version to ${newVersion}"`);
  execSync(`git tag -a v${newVersion} -m "v${newVersion}"`);
}

function publish() {
  execSync("git push");
  execSync("git push --tags");
}

function generateReleaseNote() {
  const [latestChangeLog] = fs
    .readFileSync("./CHANGELOG.md", "utf8")
    .split("\n\n\n\n", 1);

  fs.writeFileSync("./RELEASE_NOTE.md", latestChangeLog);
}

// Check git status is clean
function ensureGitStatusClean() {
  const gitStatus = execSync("git status --porcelain").toString();
  if (gitStatus) {
    console.error("Git status is not clean");
    process.exit(1);
  }
}

// Bump version
function bumpVersion(type) {
  ensureGitStatusClean();
  const oldVersion = currentVersion();
  const newVersion = type(oldVersion);

  updateVersionInFile("./README.md", oldVersion, newVersion);
  updateVersionInFile("./README-ZH_CN.md", oldVersion, newVersion);

  updateVersion(newVersion);
  updateChangeLog();
}

function updateVersionInFile(filePath, oldVersion, newVersion) {
  const readme = fs.readFileSync(filePath, "utf8");
  const reg = new RegExp(oldVersion.replace(/\./g, "\\."), "g");
  const newReadme = readme.replace(reg, newVersion);
  fs.writeFileSync(filePath, newReadme);
}

function updateChangeLog() {
  execSync("npx conventional-changelog -p angular -i CHANGELOG.md -s");
}

function patchVersion(version) {
  let [major, minor, patch] = version.split(".").map(Number);
  patch += 1;
  return `${major}.${minor}.${patch}`;
}

function minorVersion(version) {
  let [major, minor] = version.split(".").map(Number);
  minor += 1;
  return `${major}.${minor}.0`;
}

function majorVersion(version) {
  let [major] = version.split(".").map(Number);
  major += 1;
  return `${major}.0.0`;
}

function currentVersion() {
  return changeLogVersionPackage.version;
}

function updateVersion(version) {
  updateChangelogVersion(version);
  updateTauriConfVersion(version);
  updateTomlVersion(path.resolve(__dirname, "./server/Cargo.toml"), version);
  updateWebVersion(version);
}

function updateTauriConfVersion(version) {
  tauriConf.package.version = version;
  fs.writeFileSync("./gui/tauri.conf.json", JSON.stringify(tauriConf, null, 2));
}

function updateTomlVersion(path, version) {
  let content = fs.readFileSync(path, "utf8");
  const data = toml.parse(content);
  data.package.version = version;
  content = toml.stringify(data);
  fs.writeFileSync(path, content);
}

function updateChangelogVersion(version) {
  changeLogVersionPackage.version = version;
  fs.writeFileSync(
    "./package.json",
    JSON.stringify(changeLogVersionPackage, null, 2)
  );
}

function updateWebVersion(version) {
  webPackage.version = version;
  fs.writeFileSync("./web/package.json", JSON.stringify(webPackage, null, 2));
}

main();
