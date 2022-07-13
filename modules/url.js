import * as p$1 from 'punycode';
import * as o$1 from 'querystring';
import process from 'process';
import * as exports$1 from 'path';
import { URL } from 'whatwg_url'

var h = {}
var e = p$1
var a = {
  isString: function (t) { return "string" == typeof t },
  isObject: function (t) { return "object" == typeof t && null !== t },
  isNull: function (t) { return null === t },
  isNullOrUndefined: function (t) { return null == t }
};
function r() {
  this.protocol = null
  this.slashes = null
  this.auth = null
  this.host = null
  this.port = null
  this.hostname = null
  this.hash = null
  this.search = null
  this.query = null
  this.pathname = null
  this.path = null
  this.href = null
}
h.parse = O
h.resolve = function (t, s) {
  return O(t, !1, !0).resolve(s)
}
h.resolveObject = function (t, s) {
  return t ? O(t, !1, !0).resolveObject(s) : s
}
h.format = function (t) {
  a.isString(t) && (t = O(t))
  return t instanceof r ? t.format() : r.prototype.format.call(t)
}
h.Url = r
var o = /^([a-z0-9.+-]+:)/i, n = /:[0-9]*$/, i = /^(\/\/?(?!\/)[^\?\s]*)(\?[^\s]*)?$/, l = ["{", "}", "|", "\\", "^", "`"].concat(["<", ">", '"', "`", " ", "\r", "\n", "\t"]), p = ["'"].concat(l), c = ["%", "/", "?", ";", "#"].concat(p), u = ["/", "?", "#"], f = /^[+a-z0-9A-Z_-]{0,63}$/, m = /^([+a-z0-9A-Z_-]{0,63})(.*)$/, v = { javascript: !0, "javascript:": !0 }, g = { javascript: !0, "javascript:": !0 }, y = { http: !0, https: !0, ftp: !0, gopher: !0, file: !0, "http:": !0, "https:": !0, "ftp:": !0, "gopher:": !0, "file:": !0 }, b = o$1;
function O(t, s, h) {
  if (t && a.isObject(t) && t instanceof r) {
    return t;
  }
  var e = new r;
  return e.parse(t, s, h), e
}
r.prototype.parse = function (t, s, h) {
  if (!a.isString(t)) {
    throw new TypeError("Parameter 'url' must be a string, not " + typeof t);
  }
  var r = t.indexOf("?")
  var n = -1 !== r && r < t.indexOf("#") ? "?" : "#"
  var l = t.split(n);
  l[0] = l[0].replace(/\\/g, "/");
  var O = t = l.join(n);
  if (O = O.trim(), !h && 1 === t.split("#").length) {
    var d = i.exec(O);
    if (d) {
      return this.path = O, this.href = O, this.pathname = d[1], d[2] ? (this.search = d[2], this.query = s ? b.parse(this.search.substr(1)) : this.search.substr(1)) : s && (this.search = "", this.query = {}), this
    }
  }
  var j = o.exec(O);
  if (j) {
    var q = (j = j[0]).toLowerCase();
    this.protocol = q
    O = O.substr(j.length);
  }
  if (h || j || O.match(/^\/\/[^@\/]+@[^@\/]+/)) {
    var x = "//" === O.substr(0, 2);
    !x || j && g[j] || (O = O.substr(2), this.slashes = !0);
  }
  if (!g[j] && (x || j && !y[j])) {
    for (var A, C, I = -1, w = 0; w < u.length; w++) {
      -1 !== (N = O.indexOf(u[w])) && (-1 === I || N < I) && (I = N);
    }
    -1 !== (C = -1 === I ? O.lastIndexOf("@") : O.lastIndexOf("@", I)) && (A = O.slice(0, C), O = O.slice(C + 1), this.auth = decodeURIComponent(A))
    I = -1;
    for (w = 0; w < c.length; w++) {
      var N; -1 !== (N = O.indexOf(c[w])) && (-1 === I || N < I) && (I = N);
    }
    -1 === I && (I = O.length), this.host = O.slice(0, I), O = O.slice(I)
    this.parseHost()
    this.hostname = this.hostname || ""
    var U = "[" === this.hostname[0] && "]" === this.hostname[this.hostname.length - 1];
    if (!U) {
      for (var k = this.hostname.split(/\./), S = (w = 0, k.length); w < S; w++) {
        var R = k[w];
        if (R && !R.match(f)) {
          for (var $ = "", z = 0, H = R.length; z < H; z++) {
            R.charCodeAt(z) > 127 ? $ += "x" : $ += R[z];
          }
          if (!$.match(f)) {
            var L = k.slice(0, w), Z = k.slice(w + 1), _ = R.match(m); _ && (L.push(_[1]), Z.unshift(_[2])), Z.length && (O = "/" + Z.join(".") + O)
            this.hostname = L.join(".");
            break
          }
        }
      }
    }
    this.hostname.length > 255 ? this.hostname = "" : this.hostname = this.hostname.toLowerCase()
    U || (this.hostname = e.toASCII(this.hostname));
    var E = this.port ? ":" + this.port : ""
    P = this.hostname || "";
    this.host = P + E, this.href += this.host
    U && (this.hostname = this.hostname.substr(1, this.hostname.length - 2), "/" !== O[0] && (O = "/" + O));
  }
  if (!v[q]) {
    for (w = 0, S = p.length; w < S; w++) {
      var T = p[w];
      if (-1 !== O.indexOf(T)) {
        var B = encodeURIComponent(T); B === T && (B = escape(T)), O = O.split(T).join(B);
      }
    }
  }
  var D = O.indexOf("#"); -1 !== D && (this.hash = O.substr(D), O = O.slice(0, D));
  var F = O.indexOf("?");
  if (-1 !== F ? (this.search = O.substr(F), this.query = O.substr(F + 1), s && (this.query = b.parse(this.query)), O = O.slice(0, F)) : s && (this.search = "", this.query = {}), O && (this.pathname = O), y[q] && this.hostname && !this.pathname && (this.pathname = "/"), this.pathname || this.search) {
    E = this.pathname || ""; var G = this.search || ""; this.path = E + G;
  }
  return this.href = this.format(), this
}
r.prototype.format = function () {
  var t = this.auth || "";
  t && (t = (t = encodeURIComponent(t)).replace(/%3A/i, ":"), t += "@");
  var s = this.protocol || "", h = this.pathname || "", e = this.hash || "", r = !1, o = "";
  this.host ? r = t + this.host : this.hostname && (r = t + (-1 === this.hostname.indexOf(":") ? this.hostname : "[" + this.hostname + "]"), this.port && (r += ":" + this.port)), this.query && a.isObject(this.query) && Object.keys(this.query).length && (o = b.stringify(this.query));
  var n = this.search || o && "?" + o || "";
  return s && ":" !== s.substr(-1) && (s += ":"), this.slashes || (!s || y[s]) && !1 !== r ? (r = "//" + (r || ""), h && "/" !== h.charAt(0) && (h = "/" + h)) : r || (r = ""), e && "#" !== e.charAt(0) && (e = "#" + e), n && "?" !== n.charAt(0) && (n = "?" + n), s + r + (h = h.replace(/[?#]/g, (function (t) { return encodeURIComponent(t) }))) + (n = n.replace("#", "%23")) + e
}
r.prototype.resolve = function (t) {
  return this.resolveObject(O(t, !1, !0)).format()
}
r.prototype.resolveObject = function (t) {
  if (a.isString(t)) {
    var s = new r; s.parse(t, !1, !0), t = s;
  }
  for (var h = new r, e = Object.keys(this), o = 0; o < e.length; o++) {
    var n = e[o]; h[n] = this[n];
  }
  if (h.hash = t.hash, "" === t.href) {
    return h.href = h.format(), h;
  }
  if (t.slashes && !t.protocol) {
    for (var i = Object.keys(t), l = 0; l < i.length; l++) {
      var p = i[l];
      "protocol" !== p && (h[p] = t[p]);
    }
    return y[h.protocol] && h.hostname && !h.pathname && (h.path = h.pathname = "/"), h.href = h.format(), h
  }
  if (t.protocol && t.protocol !== h.protocol) {
    if (!y[t.protocol]) {
      for (var c = Object.keys(t), u = 0; u < c.length; u++) {
        var f = c[u]; h[f] = t[f];
      }
      return h.href = h.format(), h
    }
    if (h.protocol = t.protocol, t.host || g[t.protocol]) {
      h.pathname = t.pathname;
    } else {
      for (var m = (t.pathname || "").split("/"); m.length && !(t.host = m.shift());) { }
      t.host || (t.host = "")
      t.hostname || (t.hostname = "")
      "" !== m[0] && m.unshift("")
      m.length < 2 && m.unshift("")
      h.pathname = m.join("/");
    }
    if (h.search = t.search, h.query = t.query, h.host = t.host || "", h.auth = t.auth, h.hostname = t.hostname || t.host, h.port = t.port, h.pathname || h.search) {
      var v = h.pathname || "", b = h.search || ""; h.path = v + b;
    }
    return h.slashes = h.slashes || t.slashes, h.href = h.format(), h
  }
  var O = h.pathname && "/" === h.pathname.charAt(0), d = t.host || t.pathname && "/" === t.pathname.charAt(0), j = d || O || h.host && t.pathname, q = j, x = h.pathname && h.pathname.split("/") || [], A = (m = t.pathname && t.pathname.split("/") || [], h.protocol && !y[h.protocol]);
  if (A && (h.hostname = "", h.port = null, h.host && ("" === x[0] ? x[0] = h.host : x.unshift(h.host)), h.host = "", t.protocol && (t.hostname = null, t.port = null, t.host && ("" === m[0] ? m[0] = t.host : m.unshift(t.host)), t.host = null), j = j && ("" === m[0] || "" === x[0])), d) {
    h.host = t.host || "" === t.host ? t.host : h.host, h.hostname = t.hostname || "" === t.hostname ? t.hostname : h.hostname, h.search = t.search, h.query = t.query, x = m;
  } else if (m.length) {
    x || (x = []), x.pop(), x = x.concat(m), h.search = t.search, h.query = t.query;
  } else if (!a.isNullOrUndefined(t.search)) {
    if (A) {
      h.hostname = h.host = x.shift(), (U = !!(h.host && h.host.indexOf("@") > 0) && h.host.split("@")) && (h.auth = U.shift(), h.host = h.hostname = U.shift());
    }
    return h.search = t.search, h.query = t.query, a.isNull(h.pathname) && a.isNull(h.search) || (h.path = (h.pathname ? h.pathname : "") + (h.search ? h.search : "")), h.href = h.format(), h
  }
  if (!x.length) {
    return h.pathname = null, h.search ? h.path = "/" + h.search : h.path = null, h.href = h.format(), h;
  }
  for (var C = x.slice(-1)[0], I = (h.host || t.host || x.length > 1) && ("." === C || ".." === C) || "" === C, w = 0, N = x.length; N >= 0; N--) {
    "." === (C = x[N]) ? x.splice(N, 1) : ".." === C ? (x.splice(N, 1), w++) : w && (x.splice(N, 1), w--);
  } if (!j && !q) {
    for (; w--; w) { x.unshift(".."); }
  }
  !j || "" === x[0] || x[0] && "/" === x[0].charAt(0) || x.unshift(""), I && "/" !== x.join("/").substr(-1) && x.push("");
  var U, k = "" === x[0] || x[0] && "/" === x[0].charAt(0);
  A && (h.hostname = h.host = k ? "" : x.length ? x.shift() : "", (U = !!(h.host && h.host.indexOf("@") > 0) && h.host.split("@")) && (h.auth = U.shift(), h.host = h.hostname = U.shift()));
  return (j = j || h.host && x.length) && !k && x.unshift(""), x.length ? h.pathname = x.join("/") : (h.pathname = null, h.path = null), a.isNull(h.pathname) && a.isNull(h.search) || (h.path = (h.pathname ? h.pathname : "") + (h.search ? h.search : "")), h.auth = t.auth || h.auth, h.slashes = h.slashes || t.slashes, h.href = h.format(), h
}
r.prototype.parseHost = function () {
  var t = this.host, s = n.exec(t); s && (":" !== (s = s[0]) && (this.port = s.substr(1)), t = t.substr(0, t.length - s.length)), t && (this.hostname = t);
};

h.Url; h.format; h.resolve; h.resolveObject;

var exports = {},
  _dewExec = false;
function dew() {
  if (_dewExec) { return exports; }
  _dewExec = true;

  function assertPath(path) {
    if (typeof path !== "string") {
      throw new TypeError("Path must be a string. Received " + JSON.stringify(path));
    }
  } // Resolves . and .. elements in a path with directory names


  function normalizeStringPosix(path, allowAboveRoot) {
    var res = "";
    var lastSegmentLength = 0;
    var lastSlash = -1;
    var dots = 0;
    var code;

    for (var i = 0; i <= path.length; ++i) {
      if (i < path.length) { code = path.charCodeAt(i); } else if (code === 47
        /*/*/
      ) { break; } else {
        code = 47
          /*/*/
          ;
      }

      if (code === 47
        /*/*/
      ) {
        if (lastSlash === i - 1 || dots === 1); else if (lastSlash !== i - 1 && dots === 2) {
          if (res.length < 2 || lastSegmentLength !== 2 || res.charCodeAt(res.length - 1) !== 46
            /*.*/
            || res.charCodeAt(res.length - 2) !== 46
            /*.*/
          ) {
            if (res.length > 2) {
              var lastSlashIndex = res.lastIndexOf("/");

              if (lastSlashIndex !== res.length - 1) {
                if (lastSlashIndex === -1) {
                  res = "";
                  lastSegmentLength = 0;
                } else {
                  res = res.slice(0, lastSlashIndex);
                  lastSegmentLength = res.length - 1 - res.lastIndexOf("/");
                }

                lastSlash = i;
                dots = 0;
                continue;
              }
            } else if (res.length === 2 || res.length === 1) {
              res = "";
              lastSegmentLength = 0;
              lastSlash = i;
              dots = 0;
              continue;
            }
          }

          if (allowAboveRoot) {
            if (res.length > 0) { res += "/.."; } else { res = ".."; }
            lastSegmentLength = 2;
          }
        } else {
          if (res.length > 0) { res += "/" + path.slice(lastSlash + 1, i); } else { res = path.slice(lastSlash + 1, i); }
          lastSegmentLength = i - lastSlash - 1;
        }

        lastSlash = i;
        dots = 0;
      } else if (code === 46
        /*.*/
        && dots !== -1) {
        ++dots;
      } else {
        dots = -1;
      }
    }

    return res;
  }

  function _format(sep, pathObject) {
    var dir = pathObject.dir || pathObject.root;
    var base = pathObject.base || (pathObject.name || "") + (pathObject.ext || "");

    if (!dir) {
      return base;
    }

    if (dir === pathObject.root) {
      return dir + base;
    }

    return dir + sep + base;
  }

  var posix = {
    // path.resolve([from ...], to)
    resolve: function resolve() {
      var arguments$1 = arguments;

      var resolvedPath = "";
      var resolvedAbsolute = false;
      var cwd;

      for (var i = arguments.length - 1; i >= -1 && !resolvedAbsolute; i--) {
        var path;
        if (i >= 0) { path = arguments$1[i]; } else {
          if (cwd === undefined) { cwd = process.cwd(); }
          path = cwd;
        }
        assertPath(path); // Skip empty entries

        if (path.length === 0) {
          continue;
        }

        resolvedPath = path + "/" + resolvedPath;
        resolvedAbsolute = path.charCodeAt(0) === 47
          /*/*/
          ;
      } // At this point the path should be resolved to a full absolute path, but
      // handle relative paths to be safe (might happen when process.cwd() fails)
      // Normalize the path


      resolvedPath = normalizeStringPosix(resolvedPath, !resolvedAbsolute);

      if (resolvedAbsolute) {
        if (resolvedPath.length > 0) { return "/" + resolvedPath; } else { return "/"; }
      } else if (resolvedPath.length > 0) {
        return resolvedPath;
      } else {
        return ".";
      }
    },
    normalize: function normalize(path) {
      assertPath(path);
      if (path.length === 0) { return "."; }
      var isAbsolute = path.charCodeAt(0) === 47
        /*/*/
        ;
      var trailingSeparator = path.charCodeAt(path.length - 1) === 47
        /*/*/
        ; // Normalize the path

      path = normalizeStringPosix(path, !isAbsolute);
      if (path.length === 0 && !isAbsolute) { path = "."; }
      if (path.length > 0 && trailingSeparator) { path += "/"; }
      if (isAbsolute) { return "/" + path; }
      return path;
    },
    isAbsolute: function isAbsolute(path) {
      assertPath(path);
      return path.length > 0 && path.charCodeAt(0) === 47
        /*/*/
        ;
    },
    join: function join() {
      var arguments$1 = arguments;

      if (arguments.length === 0) { return "."; }
      var joined;

      for (var i = 0; i < arguments.length; ++i) {
        var arg = arguments$1[i];
        assertPath(arg);

        if (arg.length > 0) {
          if (joined === undefined) { joined = arg; } else { joined += "/" + arg; }
        }
      }

      if (joined === undefined) { return "."; }
      return posix.normalize(joined);
    },
    relative: function relative(from, to) {
      assertPath(from);
      assertPath(to);
      if (from === to) { return ""; }
      from = posix.resolve(from);
      to = posix.resolve(to);
      if (from === to) { return ""; } // Trim any leading backslashes

      var fromStart = 1;

      for (; fromStart < from.length; ++fromStart) {
        if (from.charCodeAt(fromStart) !== 47
          /*/*/
        ) { break; }
      }

      var fromEnd = from.length;
      var fromLen = fromEnd - fromStart; // Trim any leading backslashes

      var toStart = 1;

      for (; toStart < to.length; ++toStart) {
        if (to.charCodeAt(toStart) !== 47
          /*/*/
        ) { break; }
      }

      var toEnd = to.length;
      var toLen = toEnd - toStart; // Compare paths to find the longest common path from root

      var length = fromLen < toLen ? fromLen : toLen;
      var lastCommonSep = -1;
      var i = 0;

      for (; i <= length; ++i) {
        if (i === length) {
          if (toLen > length) {
            if (to.charCodeAt(toStart + i) === 47
              /*/*/
            ) {
              // We get here if `from` is the exact base path for `to`.
              // For example: from='/foo/bar'; to='/foo/bar/baz'
              return to.slice(toStart + i + 1);
            } else if (i === 0) {
              // We get here if `from` is the root
              // For example: from='/'; to='/foo'
              return to.slice(toStart + i);
            }
          } else if (fromLen > length) {
            if (from.charCodeAt(fromStart + i) === 47
              /*/*/
            ) {
              // We get here if `to` is the exact base path for `from`.
              // For example: from='/foo/bar/baz'; to='/foo/bar'
              lastCommonSep = i;
            } else if (i === 0) {
              // We get here if `to` is the root.
              // For example: from='/foo'; to='/'
              lastCommonSep = 0;
            }
          }

          break;
        }

        var fromCode = from.charCodeAt(fromStart + i);
        var toCode = to.charCodeAt(toStart + i);
        if (fromCode !== toCode) { break; } else if (fromCode === 47
          /*/*/
        ) { lastCommonSep = i; }
      }

      var out = ""; // Generate the relative path based on the path difference between `to`
      // and `from`

      for (i = fromStart + lastCommonSep + 1; i <= fromEnd; ++i) {
        if (i === fromEnd || from.charCodeAt(i) === 47
          /*/*/
        ) {
          if (out.length === 0) { out += ".."; } else { out += "/.."; }
        }
      } // Lastly, append the rest of the destination (`to`) path that comes after
      // the common path parts


      if (out.length > 0) { return out + to.slice(toStart + lastCommonSep); } else {
        toStart += lastCommonSep;
        if (to.charCodeAt(toStart) === 47
          /*/*/
        ) { ++toStart; }
        return to.slice(toStart);
      }
    },
    _makeLong: function _makeLong(path) {
      return path;
    },
    dirname: function dirname(path) {
      assertPath(path);
      if (path.length === 0) { return "."; }
      var code = path.charCodeAt(0);
      var hasRoot = code === 47
        /*/*/
        ;
      var end = -1;
      var matchedSlash = true;

      for (var i = path.length - 1; i >= 1; --i) {
        code = path.charCodeAt(i);

        if (code === 47
          /*/*/
        ) {
          if (!matchedSlash) {
            end = i;
            break;
          }
        } else {
          // We saw the first non-path separator
          matchedSlash = false;
        }
      }

      if (end === -1) { return hasRoot ? "/" : "."; }
      if (hasRoot && end === 1) { return "//"; }
      return path.slice(0, end);
    },
    basename: function basename(path, ext) {
      if (ext !== undefined && typeof ext !== "string") { throw new TypeError("\"ext\" argument must be a string"); }
      assertPath(path);
      var start = 0;
      var end = -1;
      var matchedSlash = true;
      var i;

      if (ext !== undefined && ext.length > 0 && ext.length <= path.length) {
        if (ext.length === path.length && ext === path) { return ""; }
        var extIdx = ext.length - 1;
        var firstNonSlashEnd = -1;

        for (i = path.length - 1; i >= 0; --i) {
          var code = path.charCodeAt(i);

          if (code === 47
            /*/*/
          ) {
            // If we reached a path separator that was not part of a set of path
            // separators at the end of the string, stop now
            if (!matchedSlash) {
              start = i + 1;
              break;
            }
          } else {
            if (firstNonSlashEnd === -1) {
              // We saw the first non-path separator, remember this index in case
              // we need it if the extension ends up not matching
              matchedSlash = false;
              firstNonSlashEnd = i + 1;
            }

            if (extIdx >= 0) {
              // Try to match the explicit extension
              if (code === ext.charCodeAt(extIdx)) {
                if (--extIdx === -1) {
                  // We matched the extension, so mark this as the end of our path
                  // component
                  end = i;
                }
              } else {
                // Extension does not match, so our result is the entire path
                // component
                extIdx = -1;
                end = firstNonSlashEnd;
              }
            }
          }
        }

        if (start === end) { end = firstNonSlashEnd; } else if (end === -1) { end = path.length; }
        return path.slice(start, end);
      } else {
        for (i = path.length - 1; i >= 0; --i) {
          if (path.charCodeAt(i) === 47
            /*/*/
          ) {
            // If we reached a path separator that was not part of a set of path
            // separators at the end of the string, stop now
            if (!matchedSlash) {
              start = i + 1;
              break;
            }
          } else if (end === -1) {
            // We saw the first non-path separator, mark this as the end of our
            // path component
            matchedSlash = false;
            end = i + 1;
          }
        }

        if (end === -1) { return ""; }
        return path.slice(start, end);
      }
    },
    extname: function extname(path) {
      assertPath(path);
      var startDot = -1;
      var startPart = 0;
      var end = -1;
      var matchedSlash = true; // Track the state of characters (if any) we see before our first dot and
      // after any path separator we find

      var preDotState = 0;

      for (var i = path.length - 1; i >= 0; --i) {
        var code = path.charCodeAt(i);

        if (code === 47
          /*/*/
        ) {
          // If we reached a path separator that was not part of a set of path
          // separators at the end of the string, stop now
          if (!matchedSlash) {
            startPart = i + 1;
            break;
          }

          continue;
        }

        if (end === -1) {
          // We saw the first non-path separator, mark this as the end of our
          // extension
          matchedSlash = false;
          end = i + 1;
        }

        if (code === 46
          /*.*/
        ) {
          // If this is our first dot, mark it as the start of our extension
          if (startDot === -1) { startDot = i; } else if (preDotState !== 1) { preDotState = 1; }
        } else if (startDot !== -1) {
          // We saw a non-dot and non-path separator before our dot, so we should
          // have a good chance at having a non-empty extension
          preDotState = -1;
        }
      }

      if (startDot === -1 || end === -1 || // We saw a non-dot character immediately before the dot
        preDotState === 0 || // The (right-most) trimmed path component is exactly '..'
        preDotState === 1 && startDot === end - 1 && startDot === startPart + 1) {
        return "";
      }

      return path.slice(startDot, end);
    },
    format: function format(pathObject) {
      if (pathObject === null || typeof pathObject !== "object") {
        throw new TypeError("The \"pathObject\" argument must be of type Object. Received type " + typeof pathObject);
      }

      return _format("/", pathObject);
    },
    parse: function parse(path) {
      assertPath(path);
      var ret = {
        root: "",
        dir: "",
        base: "",
        ext: "",
        name: ""
      };
      if (path.length === 0) { return ret; }
      var code = path.charCodeAt(0);
      var isAbsolute = code === 47
        /*/*/
        ;
      var start;

      if (isAbsolute) {
        ret.root = "/";
        start = 1;
      } else {
        start = 0;
      }

      var startDot = -1;
      var startPart = 0;
      var end = -1;
      var matchedSlash = true;
      var i = path.length - 1; // Track the state of characters (if any) we see before our first dot and
      // after any path separator we find

      var preDotState = 0; // Get non-dir info

      for (; i >= start; --i) {
        code = path.charCodeAt(i);

        if (code === 47
          /*/*/
        ) {
          // If we reached a path separator that was not part of a set of path
          // separators at the end of the string, stop now
          if (!matchedSlash) {
            startPart = i + 1;
            break;
          }

          continue;
        }

        if (end === -1) {
          // We saw the first non-path separator, mark this as the end of our
          // extension
          matchedSlash = false;
          end = i + 1;
        }

        if (code === 46
          /*.*/
        ) {
          // If this is our first dot, mark it as the start of our extension
          if (startDot === -1) { startDot = i; } else if (preDotState !== 1) { preDotState = 1; }
        } else if (startDot !== -1) {
          // We saw a non-dot and non-path separator before our dot, so we should
          // have a good chance at having a non-empty extension
          preDotState = -1;
        }
      }

      if (startDot === -1 || end === -1 || // We saw a non-dot character immediately before the dot
        preDotState === 0 || // The (right-most) trimmed path component is exactly '..'
        preDotState === 1 && startDot === end - 1 && startDot === startPart + 1) {
        if (end !== -1) {
          if (startPart === 0 && isAbsolute) { ret.base = ret.name = path.slice(1, end); } else { ret.base = ret.name = path.slice(startPart, end); }
        }
      } else {
        if (startPart === 0 && isAbsolute) {
          ret.name = path.slice(1, startDot);
          ret.base = path.slice(1, end);
        } else {
          ret.name = path.slice(startPart, startDot);
          ret.base = path.slice(startPart, end);
        }

        ret.ext = path.slice(startDot, end);
      }

      if (startPart > 0) { ret.dir = path.slice(0, startPart - 1); } else if (isAbsolute) { ret.dir = "/"; }
      return ret;
    },
    sep: "/",
    delimiter: ":",
    win32: null,
    posix: null
  };
  posix.posix = posix;
  exports = posix;
  return exports;
}

var path = dew();

// Copyright Joyent, Inc. and other Node contributors.

var processPlatform$1 = "wasi";

h.URL = typeof URL !== 'undefined' ? URL : null;
h.pathToFileURL = pathToFileURL$1;
h.fileURLToPath = fileURLToPath$1;

h.Url;
h.format;
h.resolve;
h.resolveObject;

h.URL;

var CHAR_FORWARD_SLASH$1 = 47;
var CHAR_LOWERCASE_A$1 = 97;
var CHAR_LOWERCASE_Z$1 = 122;

var isWindows$1 = processPlatform$1 === 'win32';

var forwardSlashRegEx$1 = /\//g;
var percentRegEx$1 = /%/g;
var backslashRegEx$1 = /\\/g;
var newlineRegEx$1 = /\n/g;
var carriageReturnRegEx$1 = /\r/g;
var tabRegEx$1 = /\t/g;

/**
 * Get fully resolved platform-specific file path from the given URL string/ object
 * @param path The file URL string or URL object to convert to a path
 */
function fileURLToPath$1(path) {
  if (typeof path === "string") { path = new URL(path); }
  else if (!(path instanceof URL)) {
    throw new TypeError(
      "invalid argument path , must be a string or URL"
    );
  }
  if (path.protocol !== "file:") {
    throw new TypeError("invalid url scheme");
  }
  return isWindows$1 ? getPathFromURLWin$1(path) : getPathFromURLPosix$1(path);
}

function getPathFromURLWin$1(url) {
  var hostname = url.hostname;
  var pathname = url.pathname;
  for (var n = 0; n < pathname.length; n++) {
    if (pathname[n] === "%") {
      var third = pathname.codePointAt(n + 2) || 0x20;
      if (
        (pathname[n + 1] === "2" && third === 102) || // 2f 2F /
        (pathname[n + 1] === "5" && third === 99)
      ) {
        // 5c 5C \
        throw new TypeError(
          "must not include encoded \\ or / characters"
        );
      }
    }
  }

  pathname = pathname.replace(forwardSlashRegEx$1, "\\");
  pathname = decodeURIComponent(pathname);
  if (hostname !== "") {
    //TODO add support for punycode encodings
    return ("\\\\" + hostname + pathname);
  } else {
    // Otherwise, it's a local path that requires a drive letter
    var letter = pathname.codePointAt(1) | 0x20;
    var sep = pathname[2];
    if (
      letter < CHAR_LOWERCASE_A$1 ||
      letter > CHAR_LOWERCASE_Z$1 || // a..z A..Z
      sep !== ":"
    ) {
      throw new TypeError("file url path must be absolute");
    }
    return pathname.slice(1);
  }
}
function getPathFromURLPosix$1(url) {
  if (url.hostname !== "") {
    throw new TypeError("invalid file url hostname");
  }
  var pathname = url.pathname;
  for (var n = 0; n < pathname.length; n++) {
    if (pathname[n] === "%") {
      var third = pathname.codePointAt(n + 2) || 0x20;
      if (pathname[n + 1] === "2" && third === 102) {
        throw new TypeError(
          "must not include encoded / characters"
        );
      }
    }
  }
  return decodeURIComponent(pathname);
}

/** Get fully resolved platform-specific File URL from the given file path */
function pathToFileURL$1(filepath) {
  var resolved = path.resolve(filepath);
  // path.resolve strips trailing slashes so we must add them back
  var filePathLast = filepath.charCodeAt(filepath.length - 1);
  if (
    (filePathLast === CHAR_FORWARD_SLASH$1 ||
      (isWindows$1)) &&
    resolved[resolved.length - 1] !== path.sep
  ) {
    resolved += "/";
  }
  var outURL = new URL("file://");
  if (resolved.includes("%")) { resolved = resolved.replace(percentRegEx$1, "%25"); }
  // In posix, "/" is a valid character in paths
  if (resolved.includes("\\")) {
    resolved = resolved.replace(backslashRegEx$1, "%5C");
  }
  if (resolved.includes("\n")) { resolved = resolved.replace(newlineRegEx$1, "%0A"); }
  if (resolved.includes("\r")) {
    resolved = resolved.replace(carriageReturnRegEx$1, "%0D");
  }
  if (resolved.includes("\t")) { resolved = resolved.replace(tabRegEx$1, "%09"); }
  outURL.pathname = resolved;
  return outURL;
}

// Copyright Joyent, Inc. and other Node contributors.

var processPlatform = "wasi";

h.URL = typeof URL !== 'undefined' ? URL : null;
h.pathToFileURL = pathToFileURL;
h.fileURLToPath = fileURLToPath;

var Url = h.Url;
var format = h.format;
var resolve = h.resolve;
var resolveObject = h.resolveObject;
var parse = h.parse;

var _URL = h.URL;
var CHAR_FORWARD_SLASH = 47;
var CHAR_LOWERCASE_A = 97;
var CHAR_LOWERCASE_Z = 122;

var isWindows = processPlatform === 'win32';

var forwardSlashRegEx = /\//g;
var percentRegEx = /%/g;
var backslashRegEx = /\\/g;
var newlineRegEx = /\n/g;
var carriageReturnRegEx = /\r/g;
var tabRegEx = /\t/g;

/**
 * Get fully resolved platform-specific file path from the given URL string/ object
 * @param path The file URL string or URL object to convert to a path
 */
function fileURLToPath(path) {
  if (typeof path === "string") { path = new URL(path); }
  else if (!(path instanceof URL)) {
    throw new TypeError(
      "invalid argument path , must be a string or URL"
    );
  }
  if (path.protocol !== "file:") {
    throw new TypeError("invalid url scheme");
  }
  return isWindows ? getPathFromURLWin(path) : getPathFromURLPosix(path);
}

function getPathFromURLWin(url) {
  var hostname = url.hostname;
  var pathname = url.pathname;
  for (var n = 0; n < pathname.length; n++) {
    if (pathname[n] === "%") {
      var third = pathname.codePointAt(n + 2) || 0x20;
      if (
        (pathname[n + 1] === "2" && third === 102) || // 2f 2F /
        (pathname[n + 1] === "5" && third === 99)
      ) {
        // 5c 5C \
        throw new TypeError(
          "must not include encoded \\ or / characters"
        );
      }
    }
  }

  pathname = pathname.replace(forwardSlashRegEx, "\\");
  pathname = decodeURIComponent(pathname);
  if (hostname !== "") {
    //TODO add support for punycode encodings
    return ("\\\\" + hostname + pathname);
  } else {
    // Otherwise, it's a local path that requires a drive letter
    var letter = pathname.codePointAt(1) | 0x20;
    var sep = pathname[2];
    if (
      letter < CHAR_LOWERCASE_A ||
      letter > CHAR_LOWERCASE_Z || // a..z A..Z
      sep !== ":"
    ) {
      throw new TypeError("file url path must be absolute");
    }
    return pathname.slice(1);
  }
}
function getPathFromURLPosix(url) {
  if (url.hostname !== "") {
    throw new TypeError("invalid file url hostname");
  }
  var pathname = url.pathname;
  for (var n = 0; n < pathname.length; n++) {
    if (pathname[n] === "%") {
      var third = pathname.codePointAt(n + 2) || 0x20;
      if (pathname[n + 1] === "2" && third === 102) {
        throw new TypeError(
          "must not include encoded / characters"
        );
      }
    }
  }
  return decodeURIComponent(pathname);
}

/** Get fully resolved platform-specific File URL from the given file path */
function pathToFileURL(filepath) {
  var resolved = exports$1.resolve(filepath);
  // path.resolve strips trailing slashes so we must add them back
  var filePathLast = filepath.charCodeAt(filepath.length - 1);
  if (
    (filePathLast === CHAR_FORWARD_SLASH ||
      (isWindows)) &&
    resolved[resolved.length - 1] !== exports$1.sep
  ) {
    resolved += "/";
  }
  var outURL = new URL("file://");
  if (resolved.includes("%")) { resolved = resolved.replace(percentRegEx, "%25"); }
  // In posix, "/" is a valid character in paths
  if (resolved.includes("\\")) {
    resolved = resolved.replace(backslashRegEx, "%5C");
  }
  if (resolved.includes("\n")) { resolved = resolved.replace(newlineRegEx, "%0A"); }
  if (resolved.includes("\r")) {
    resolved = resolved.replace(carriageReturnRegEx, "%0D");
  }
  if (resolved.includes("\t")) { resolved = resolved.replace(tabRegEx, "%09"); }
  outURL.pathname = resolved;
  return outURL;
}

export { _URL as URL, Url, h as default, fileURLToPath, format, parse, pathToFileURL, resolve, resolveObject };
