const path = require('path');
module.exports = {
  entry: './server/index.js',
  externals: [
    {"wasi_http": "wasi_http"},
    {"wasi_net": "wasi_net"},
    {"std": "std"}
  ],
  output: {
    path: path.resolve('server-build'),
    filename: 'index.js',
    chunkFormat: "module",
    library: {
      type: "module"
    },
  },
  experiments: {
    outputModule: true
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: 'babel-loader'
      },
      {
        test: /\.css$/,
        use: ["css-loader"]
      },
      {
        test: /\.svg$/,
        use: ["svg-url-loader"]
      }
    ]
  }
};
