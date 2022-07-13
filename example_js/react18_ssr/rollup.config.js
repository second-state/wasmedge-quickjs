const {babel} = require('@rollup/plugin-babel');
const nodeResolve = require('@rollup/plugin-node-resolve');
const commonjs = require('@rollup/plugin-commonjs');
const replace = require('@rollup/plugin-replace');

const globals = require('rollup-plugin-node-globals');
const plugin_async = require('rollup-plugin-async');


const babelOptions = {
  'presets': ['@babel/preset-react']
};

module.exports = [
  {
    input: './main.mjs',
    output: {
      inlineDynamicImports: true,
      file: 'dist/main.mjs',
      format: 'esm',
    },
    plugins: [
      babel(babelOptions),
      plugin_async(),
      nodeResolve(),
      commonjs({ignoreDynamicRequires: false}),
      globals(),
      replace({
        'process.env.NODE_ENV': JSON.stringify('production'),
        'process.env.NODE_DEBUG': JSON.stringify(''),
      }),
    ],
  },
];