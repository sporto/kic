// const elmSource = __dirname;

import * as path from "path"
import * as webpack from "webpack"
import * as AssetsPlugin from "assets-webpack-plugin"
import * as HtmlWebpackPlugin from "html-webpack-plugin"

// const DEVELOPMENT = "development"
// const PRODUCTION = "production"
// let TARGET = process.env.NODE_ENV || DEVELOPMENT

let outputPath = path.join(__dirname, "dist")

let assetsPluginInstance = new AssetsPlugin({
    path: outputPath,
})

// let publicPath = "/webpack/"

const ENTRY_SIGN_IN = "sign-in"
const ENTRY_SIGN_UP = "sign-up"
const ENTRY_ADMIN = "admin"
const ENTRY_INVESTOR = "investor"

let baseConfig: webpack.Configuration = {
    entry: {
        [ENTRY_SIGN_IN]: "./src/signIn.ts",
        [ENTRY_SIGN_UP]: "./src/signUp.ts",
        [ENTRY_ADMIN]: "./src/admin.ts",
        [ENTRY_INVESTOR]: "./src/investor.ts",
    },
    output: {
        filename: "[name].js",
        path: outputPath,
        // publicPath: publicPath,
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: {
                    loader: "ts-loader",
                    options: {
                        // logInfoToStdOut: true,
                    },
                },
            },
            {
                test: /\.elm$/,
                exclude: [/elm-stuff/, /node_modules/],
                use: {
                    loader: "elm-webpack-loader",
                    options: {
                        // cwd: elmSource,
                    },
                },
            },
        ],
    },
    resolve: {
        extensions: [".ts", ".js"]
    },
    plugins: [
        assetsPluginInstance,
        new HtmlWebpackPlugin({
            chunks: [ENTRY_SIGN_IN],
            title: "Sign In",
            filename: "sign-in/index.html",
            template: "src/application.html",
        }),
        new HtmlWebpackPlugin({
            chunks: [ENTRY_SIGN_UP],
            title: "Sign Ip",
            filename: "sign-up/index.html",
            template: "src/application.html",
        }),
        new HtmlWebpackPlugin({
            chunks: [ENTRY_ADMIN],
            title: "Admin",
            filename: "admin/index.html",
            template: "src/application.html",
        }),
        new HtmlWebpackPlugin({
            chunks: [ENTRY_INVESTOR],
            title: "Investor",
            filename: "investor/index.html",
            template: "src/application.html",
        }),
    ],
}

export default baseConfig
