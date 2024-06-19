const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: 'bootstrap.js',
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html','styles.css',
      '../static/background.png',
      '../static/possum_sprite_sheet.png',
      '../static/sprite_frag.glsl',
      '../static/sprite_vert.glsl'
    ])
  ],
};
