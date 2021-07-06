const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {from: 'index.html', to: '.'},
        {
          from: '../assets/**',
          to: 'assets/'
        }
      ]
    })
  ],
  module: {
    rules: [
      {
        test: /\.(png|jpg|gif)$/i,
        type: 'asset/resource'
      },
    ],
  },
};
