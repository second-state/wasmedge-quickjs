// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

import process from "process";

export const blue = '';
export const green = '';
export const white = '';
export const red = '';
export const clear = '';
export const hasColors = false;
export function refresh() {
  if (false && process.stderr.isTTY) {
    const hasColors = process.stderr.hasColors();
    module.exports.blue = hasColors ? '\u001b[34m' : '';
    module.exports.green = hasColors ? '\u001b[32m' : '';
    module.exports.white = hasColors ? '\u001b[39m' : '';
    module.exports.red = hasColors ? '\u001b[31m' : '';
    module.exports.clear = hasColors ? '\u001bc' : '';
    module.exports.hasColors = hasColors;
  }
}

refresh();
