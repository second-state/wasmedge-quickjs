const {babel} = require('@rollup/plugin-babel');
const nodeResolve = require('@rollup/plugin-node-resolve');
const commonjs = require('@rollup/plugin-commonjs');
const replace = require('@rollup/plugin-replace');

const globals = require('rollup-plugin-node-globals');
const builtins = require('rollup-plugin-node-builtins');
const plugin_async = require('rollup-plugin-async');


const babelOptions = {
  'presets': ['@babel/preset-react']
};

module.exports = [
  {
    // input: './nodejs_main.mjs',
    input: './main.mjs',
    output: {
      inlineDynamicImports: true,
      // file: 'dist/nodejs_main.mjs',
      file: 'dist/main.mjs',
      format: 'esm',
    },
    external: ['process', 'wasi_net','http'],
    plugins: [
      babel(babelOptions),
      plugin_async(),
      nodeResolve(),
      commonjs({ignoreDynamicRequires: false}),
      globals(),
      builtins(),
      replace({
        'process.env.NODE_ENV': JSON.stringify('production'),
        'process.env.NODE_DEBUG': JSON.stringify(''),
      }),
    ],
  },
];