const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");

config.output = config.output || {};
config.output.filename = "bangs.[contenthash:7].js";

config.plugins = config.plugins || [];
config.plugins.push(
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "../../../../build/processedResources/js/main/index.html"),
        minify: config.mode === "production" ? {
            collapseWhitespace: true,
            keepClosingSlash: true,
            removeComments: true,
            removeRedundantAttributes: true,
            removeScriptTypeAttributes: true,
            removeStyleLinkTypeAttributes: true,
            useShortDoctype: true,
            minifyCSS: true,
        } : false,
    }),
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "../../../../build/processedResources/js/main/search/index.html"),
        filename: 'search/index.html'
    })
);
