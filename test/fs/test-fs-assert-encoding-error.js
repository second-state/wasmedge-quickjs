// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';
import common from '../common';
import assert from 'assert';
import fs from 'fs';

const options = 'test';
const expectedError = {
  code: 'ERR_INVALID_ARG_VALUE',
  name: 'TypeError',
};

assert.throws(() => {
  fs.readFile('path', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.readFileSync('path', options);
}, expectedError);

assert.throws(() => {
  fs.readdir('path', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.readdirSync('path', options);
}, expectedError);

assert.throws(() => {
  fs.readlink('path', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.readlinkSync('path', options);
}, expectedError);

assert.throws(() => {
  fs.writeFile('path', 'data', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.writeFileSync('path', 'data', options);
}, expectedError);

assert.throws(() => {
  fs.appendFile('path', 'data', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.appendFileSync('path', 'data', options);
}, expectedError);

// unsupport watch
/*
assert.throws(() => {
  fs.watch('path', options, common.mustNotCall());
}, expectedError);
*/

assert.throws(() => {
  fs.realpath('path', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.realpathSync('path', options);
}, expectedError);

assert.throws(() => {
  fs.mkdtemp('path', options, common.mustNotCall());
}, expectedError);

assert.throws(() => {
  fs.mkdtempSync('path', options);
}, expectedError);

assert.throws(() => {
  fs.createReadStream('path', options);
}, expectedError);

assert.throws(() => {
  fs.createWriteStream('path', options);
}, expectedError);
