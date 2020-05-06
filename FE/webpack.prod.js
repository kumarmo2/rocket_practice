const commonConfig = require('./webpack.common');
const path = require("path");

const buildPath = path.resolve(__dirname, "../public");

const prodConfig = Object.assign({}, commonConfig, {
    mode: "production",
});

module.exports = prodConfig;