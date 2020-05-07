const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const buildPath = path.resolve(__dirname, '../public');

const commonConfig = {
  entry: {
    app: './src/index.js',
  },
  output: {
    filename: '[name].[contenthash].js',
    path: buildPath,
  },
  module: {
    // laoders transform the assets.
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /(node_modules)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env', '@babel/preset-react'],
          },
        },
      },
      {
        test: /\.(css|scss)$/,
        use: [
          {
            loader: 'style-loader',
          },
          {
            loader: 'css-loader',
          },
        ],
      },
    ],
  },
  optimization: {
    splitChunks: {
      chunks: 'all', // consider all the chunks for optimization
      name: true,
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          priority: -10, // need to understand priority.
        },
        default: {
          // need to look what these do.
          minChunks: 2,
          priority: -20,
          reuseExistingChunk: true,
        },
      },
    },
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HTMLWebpackPlugin({
      filename: '../views/index.hbs',
      template: 'index.html',
    }),
  ],
};

module.exports = commonConfig;
