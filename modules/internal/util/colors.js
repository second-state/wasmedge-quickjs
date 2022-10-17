// Copyright Joyent and Node contributors. All rights reserved. MIT license.

'use strict';

import process from "process";

let blue = '';
let green = '';
let white = '';
let red = '';
let clear = '';
let hasColors = false;
export function refresh() {
  if (true || process.stderr.isTTY) {
    hasColors = true || process.stderr.hasColors();
    blue = hasColors ? '\u001b[34m' : '';
    green = hasColors ? '\u001b[32m' : '';
    white = hasColors ? '\u001b[39m' : '';
    red = hasColors ? '\u001b[31m' : '';
    clear = hasColors ? '\u001bc' : '';
    hasColors = hasColors;
  }
}

export {
  blue,
  green,
  white,
  red,
  clear,
  hasColors
}

refresh();
