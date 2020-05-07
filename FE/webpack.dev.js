const path = require('path');
const commonConfig = require('./webpack.common');

const buildPath = path.resolve(__dirname, '../public');

const devConfig = {
  ...commonConfig,
  mode: 'development',
  devtool: 'inline-source-map', // this prevents creating minified bundle.
  devServer: {
    contentBase: buildPath,
    port: 9000,
  },
};

module.exports = devConfig;
