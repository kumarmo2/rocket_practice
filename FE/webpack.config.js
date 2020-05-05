const path = require("path");

const buildPath = path.resolve(__dirname, "../public");

//TODO: need to add webpack-dev-server for faster development.
module.exports = {
    entry: {
        app: "./src/index.js"
    },
    output: {
        filename: "main.js",
        path: buildPath,
    },
    devServer: {
        contentBase: buildPath,
        port: 9000
    },
    module: { // laoders transform the assets.
        rules: [
            {
                test: /\.(js|jsx)$/,
                exclude: /(node_modules)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env', '@babel/preset-react']
                    }
                }
            },
            {
                test: /\.(css|scss)$/,
                use: [
                    {
                        loader: 'style-loader',
                    },
                    {
                        loader: 'css-loader',
                    }
                ]
            }
        ]
    }
}