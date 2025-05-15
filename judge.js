#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const inputFile = process.argv[2];
if (!inputFile) {
  console.error("Usage: judge.js <input_file>");
  process.exit(1);
}

const content = fs.readFileSync(inputFile, "utf-8");

const TESTCASES = [
  "glibc-basic",
  "glibc-libctest",
  "glibc-lua",
  "glibc-busybox",
  "glibc-iozone",
  "musl-basic",
  "musl-libctest",
  "musl-lua",
  "musl-busybox",
  "musl-iozone",
];
const classroomDir = path.join(__dirname, ".github", "classroom");
let totalScore = 0, totalFullScore = 0;
for (const testcase of TESTCASES) {
  console.log(`Testing [${testcase}]`);
  let score = 0,
    fullScore = 0;
  const scriptDir = path.join(classroomDir, testcase);
  const script = path.join(scriptDir, fs.readdirSync(scriptDir)[0]);
  const exports = require(script);
  const result = exports.judge(content);
  for (const [name, res] of Object.entries(result)) {
    const [got, full] = res;
    if (got === full) {
      console.log(`  ✅ ${name} pass`);
    } else {
      console.log(`  ❌ ${name} points ${got}/${full}`);
    }
    score += got;
    fullScore += full;
  }
  console.log(`Score: ${score}/${fullScore}`);
  console.log();
  totalScore += score;
  totalFullScore += fullScore;
}

console.log(`Total ${totalScore}/${totalFullScore}`);
