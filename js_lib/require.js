//copy from https://github.com/IvanGaravito/quickjs-require
import * as std from 'std'
import * as os from 'os'

let modules = {}
let debug = console.log
{
	let _debugOptions = std.getenv('DEBUG')
	if (typeof _debugOptions == 'undefined' || _debugOptions.split(',').indexOf('require') === -1) {
		debug = function () {}
	}
}

class CJSModule {
	constructor (id) {
		this.id = id
		this._failed = null
		this._loaded = false
		this.exports = {}
	}

	load () {
		const __file = this.id
		const __dir = _basename(this.id)
		const _require = require

		let ctx = { exports: {} }
		// Prevents modules from changing exports
		Object.seal(ctx)

		const _mark = '<<SCRIPT>>'
		let _loaderTemplate = `(function _loader (exports, require, module, __filename, __dirname) {${_mark}})(ctx.exports, _require, ctx, __file, __dir)`

		let _script = std.loadFile(__file)
		this._failed = _script === null
		if (this._failed) {
			return new Error(`Can't load script ${__file}`)
		}

		_script = _loaderTemplate.replace('<<SCRIPT>>', _script)
		eval(_script)

		this.exports = ctx.exports
		this._loaded = true
		return true
	}

}

function _basename (path) {
	let idx = path.lastIndexOf('/')
	if (idx === 0)
		return '/'
	return path.substring(0, idx)
}

function _statPath (path) {
	const [fstat, err] = os.stat(path)
	return {
		errno: err,
		isFile: fstat && (fstat.mode & os.S_IFREG) && true,
		isDir: fstat && (fstat.mode & os.S_IFDIR) && true
	}
}

function _loadModule (path) {
	debug(`_loadModule# Module ${path}`)
//	const [id, err] = os.realpath(path)
//	if (err) {
//		throw new Error(`Module require error: Can't get real module path for ${path}`)
//		return
//	}
    const id = path

	debug(`_loadModule# id ${id}`)
	if (modules.hasOwnProperty(id)) {
		return modules[id]
	}

	let _module = new CJSModule(id)
	modules[id] = _module

	let _result = _module.load()
	if (_result !== true) {
		throw _result
		return
	}
	return _module
}

function _lookupModule (path) {
	let fstat = _statPath(path)

	debug(`_lookupModule# Looking for ${path}`)
	// Path found
	if (fstat.isFile) {
		debug(`_lookupModule# Found module file`)
		return path
	}

	// Path not found
	if (fstat.errno) {
		debug(`_lookupModule# Not found module file`)
		// Try with '.js' extension
		if (!path.endsWith('.js') && _statPath(`${path}.js`).isFile) {
			debug(`_lookupModule# Found appending .js to file name`)
			return `${path}.js`
		}
		return new Error(`Error: Module ${path} not found!`)
	}

	// Path found and it isn't a dir
	if (!fstat.isDir) {
		return new Error(`Error: Module file type not supported for ${path}`)
	}

	// Path it's a dir
	let _path = null	// Real path to module
	let _tryOthers = true	// Keep trying?

	debug(`_lookupModule# Path is a directory, trying options...`)
	// Try with package.json for NPM or YARN modules
	if (_statPath(`${path}/package.json`).isFile) {
		debug(`_lookupModule# It has package.json, looking for main script...`)
		let _pkg = JSON.parse(std.loadFile(`${path}/package.json`))
		if (_pkg && Object.keys(_pkg).indexOf('main') !== -1 && _pkg.main !== '' && _statPath(`${path}/${_pkg.main}`).isFile) {
			_tryOthers = false
			_path = `${path}/${_pkg.main}`
			debug(`_lookupModule# Found package main script!`)
		}
	}
	// Try other options
	if (_tryOthers && _statPath(`${path}/index.js`).isFile) {
		_tryOthers = false
		_path = `${path}/index.js`
		debug(`_lookupModule# Found package index.js file`)
	}
	if (_tryOthers && _statPath(`${path}/main.js`).isFile) {
		_tryOthers = false
		_path = `${path}/main.js`
		debug(`_lookupModule# Found package main.js file`)
	}

	if (_path === null) {
		return new Error(`Error: Module ${path} is a directory, but not a package`)
	}

	debug(`_lookupModule# Found module file: ${_path}`)
	// Returns what it founded
	return _path
}

export function require (path) {
	if (typeof __filename == 'undefined') {
		debug('require# Calling from main script')
	} else {
		debug(`require# Calling from ${__filename} parent module`)
	}
	let _path = _lookupModule(path)

	// Module not found
	if (_path instanceof Error) {
		throw _path
		return
	}

	let _module = _loadModule(_path)

	return _module.exports
}