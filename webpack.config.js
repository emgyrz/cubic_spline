var CopyWebpackPlugin = require("copy-webpack-plugin");
var path = require('path');
var isProd = process.env.NODE_ENV === 'production';
module.exports = {
    entry: "./index.ts",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "main.js",
    },
    mode: 'development',
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/
            },
            {
                test: /\.s(c|a)ss$/,
                use: [
                    "style-loader",
                    "css-loader",
                    "sass-loader"
                ]
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js', '.wasm']
    },
};
