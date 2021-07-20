const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = (env, argv) => {
  const prod = argv.mode === 'production';

  return {
    entry: path.join(__dirname, 'index.js'),
    mode: prod ? 'production' : 'development',
    experiments: { asyncWebAssembly: true },
    plugins: [
      new HtmlWebpackPlugin({
        template: path.join(__dirname, 'index.template.html'),
      }),
      new WasmPackPlugin({
        // For some reason this creates a second 'pkg' dir in the
        // root dir with an empty wasm-game-of-life.js file in it.
        crateDirectory: path.join(__dirname, 'crate'),
        outName: 'wasm-game-of-life',
        forceMode: 'development',
      }),
    ],
  };
};
