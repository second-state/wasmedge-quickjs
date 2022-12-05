'use strict';

import path from 'path';
import fs from 'fs';
import { pathToFileURL } from 'url';

const __dirname = path.join(args[0], '..');

const fixturesDir = path.join(__dirname, '..', 'fixtures');

function fixturesPath(...args) {
  return path.join(fixturesDir, ...args);
}

function fixturesFileURL(...args) {
  return pathToFileURL(fixturesPath(...args));
}

function readFixtureSync(args, enc) {
  if (Array.isArray(args))
    return fs.readFileSync(fixturesPath(...args), enc);
  return fs.readFileSync(fixturesPath(args), enc);
}

function readFixtureKey(name, enc) {
  return fs.readFileSync(fixturesPath('keys', name), enc);
}

function readFixtureKeys(enc, ...names) {
  return names.map((name) => readFixtureKey(name, enc));
}

export {
  fixturesDir,
  fixturesPath as path,
  fixturesFileURL as fileURL,
  readFixtureSync as readSync,
  readFixtureKey as readKey,
  readFixtureKeys as readKeys,
};

export default {
  fixturesDir,
  path: fixturesPath,
  fileURL: fixturesFileURL,
  readSync: readFixtureSync,
  readKey: readFixtureKey,
  readKeys: readFixtureKeys,
}
