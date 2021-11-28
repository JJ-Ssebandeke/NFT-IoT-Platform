var webpack = require('webpack');
const { resolve } = require('path')
const HtmlWebpackPlugin = require("html-webpack-plugin")
const NodePolyfillPlugin = require("node-polyfill-webpack-plugin")

module.exports = {
    entry: './src/index.tsx',
    mode: 'development',
    output: {
        filename: 'bundle.js',
        path: resolve(__dirname, 'dist')
    },
    resolve: {extensions: ['.js','.jsx','.ts','.tsx']},
    module: {
        rules: [ 
            {
              test: /\.(ts|tsx)$/,
              exclude: /nodeModules/,
              loader: 'babel-loader',
           },
          {
             test: /\.css$/,
             use: ['style-loader', 'css-loader']
          }
        ]

    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './public/index.html',
            filename: 'index.html'
        }),
        new NodePolyfillPlugin()
    ],
    devServer: {
        compress: true,
        port: 7000,
        hot: true,
        open: true,
    },
    devtool: 'inline-source-map'

}