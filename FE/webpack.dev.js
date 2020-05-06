const commonConfig = require('./webpack.common');
const path = require("path");

const buildPath = path.resolve(__dirname, "../public");


const devConfig = Object.assign({}, commonConfig, {
    mode: "development",
    devtool: 'inline-source-map', // this prevents creating minified bundle.
    devServer: {
        contentBase: buildPath,
        port: 9000
    },
});

// const devConfig = Object.assign({}, commonConfig);
module.exports = devConfig;