import process from 'process';
import { Buffer } from 'buffer';
import { text_encode } from '_encoding';
import { _memorySize } from '_node:os';

export * from 'qjs:os';

function unimplemented(name) {
  throw new Error('Node.js os ' + name + ' is not supported');
}

var EOL = '\n';

function arch() {
  return process.arch;
}

var constants = [];

function cpus() {
  unimplemented('cpus');
}

var devNull = '/dev/null';

function endianness() {
  return 'LE';
}

function freemem() {
  // memory.size instruction will return the current 
  // memory size in units of pages. 
  // A page size is 65536 bytes.
  return totalmem() - _memorySize() * 65536;
}

function getPriority(pid) {
  if (pid === undefined) {
    pid = 0;
  }
  return 0;
}

function homedir() {
  return process.env['HOME'] || '.';
}

function hostname() {
  return process.title;
}

function loadavg() {
  return [0, 0, 0];
}

function networkInterfaces() {
  return [];
}

function platform() {
  return process.platform;
}

function release() {
  return process.version;
}

function setPriority(pid, priority) {
  if (priority === undefined) {
    priority = pid;
    pid = 0;
  }
}

function tmpdir() {
  let path = process.env['TMPDIR'] || process.env['TMP'] || process.env['TEMP'] || '/tmp';
  if (path.length > 1 && path.endsWith('/')) {
    path = path.slice(0, -1);
  }
  return path;
}

function totalmem() {
  return 2 ** 32;
}

function type() {
  return 'wasmedge';
}

function uptime() {
  return process.uptime();
}

function userInfo(options) {
  const encoding = (options && options.encoding) || 'utf8';
  let username = 'wasmedge';
  let _homedir = homedir();
  if (encoding === 'Buffer' || encoding === 'buffer') {
    username = Buffer.from(username, 'utf8');
    _homedir = Buffer.from(_homedir, 'utf8');
  } else if (encoding !== 'utf-8' && encoding !== 'utf8') {
    let exist = [
      'utf8', 'utf-8', 'gbk', 'gb18030', 'hz-gb-2312', 'big5', 'euc-jp', 'iso-2022-jp',
      'utf-16be', 'utf-16le', 'x-user-defined', 'ibm866',
      'iso-8859-2', 'iso-8859-3', 'iso-8859-4', 'iso-8859-5', 'iso-8859-6', 'iso-8859-7', 'iso-8859-8',
      'iso-8859-8i', 'iso-8859-10', 'iso-8859-13', 'iso-8859-14', 'iso-8859-15', 'iso-8859-16',
      'windows-874', 'windows-1250', 'windows-1251', 'windows-1252', 'windows-1253', 'windows-1254',
      'windows-1255', 'windows-1256', 'windows-1257', 'windows-1258', ''
    ].indexOf(encoding);
    if (exist >= 0) {
      const decoder = new TextDecoder();
      username = text_encode(encoding, decoder.decode(username));
      _homedir = text_encode(encoding, decoder.decode(_homedir));
    }
  }
  return {
    uid: -1,
    pid: -1,
    username,
    homedir: _homedir,
    shell: null
  }
}

function version() {
  return process.version;
}

export {
  EOL,
  arch,
  constants,
  cpus,
  devNull,
  endianness,
  freemem,
  getPriority,
  homedir,
  hostname,
  loadavg,
  networkInterfaces,
  platform,
  release,
  setPriority,
  tmpdir,
  totalmem,
  type,
  uptime,
  userInfo,
  version
}