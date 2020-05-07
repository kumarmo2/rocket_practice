const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
const commonConfig = require('./webpack.common');

const prodConfig = {
  ...commonConfig,
  mode: 'production',
  optimization: {
    splitChunks: {
      chunks: 'all', // consider all the chunks for optimization
      name: true,
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          priority: -10,
        },
        react: {
          test: /[\\/]node_modules[\\/](react|react-dom)[\\/]/,
          name: 'react',
          chunks: 'all', // TODO: need to understand this property.
        },
        'material-ui': {
          test: /[\\/]node_modules[\\/](@material-ui)[\\/]/,
          name: 'material-ui',
          chunks: 'all',
        },
        default: {
          minChunks: 2,
          priority: -20,
          reuseExistingChunk: true,
        },
      },
    },
  },
  plugins: [
    // do bundle analysis on production build only because that makes sense.
    new BundleAnalyzerPlugin({
      // default port is 8888
      generateStatsFile: true,
      analyzerMode: 'disabled', // don't start the analyzer server on production build, using option `generateStatsFile`, only stats file will be generated.
      analyzerPort: 8888,
    }),
  ],
};

module.exports = prodConfig;
