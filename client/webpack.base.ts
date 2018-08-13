import * as invariant from "invariant"
import * as webpack from "webpack"
// @ts-ignore
import * as MiniCssExtractPlugin from "mini-css-extract-plugin"
import * as HtmlWebpackPlugin from "html-webpack-plugin"
import * as path from "path"

const API_HOST = process.env.API_HOST

const ENTRY_ADMIN = "admin"
const ENTRY_INVESTOR = "investor"
const ENTRY_SIGN_IN = "sign-in"
const ENTRY_SIGN_UP = "sign-up"
const ENTRY_GRAPHIQL = "graphiql"
const COMMON = "common"
const STYLES = "styles"
const ASSETS_PATH = "/app"
const DEV_MODE = process.env.NODE_ENV !== "production"

invariant(API_HOST, "API_HOST must be defined")

let baseConfig: webpack.Configuration = {
	entry: {
		[ENTRY_GRAPHIQL]: "./src/graphiql.ts",
		[ENTRY_SIGN_IN]: "./src/signIn.ts",
		[ENTRY_SIGN_UP]: "./src/signUp.ts",
		[ENTRY_ADMIN]: "./src/admin.ts",
		[ENTRY_INVESTOR]: "./src/investor.ts",
	},
	output: {
		publicPath: ASSETS_PATH,
	},
	optimization: {
		splitChunks: {
			cacheGroups: {
				common: {
					// test: /[\\/]node_modules[\\/]/,
					name: COMMON,
					chunks: "initial",
					enforce: true
				},
				styles: {
					name: STYLES,
					test: /\.css$/,
					chunks: "all",
					enforce: true,
				},
			},
		},
	},
	module: {
		rules: [

			{
				test: /\.ts$/,
				use: {
					loader: "ts-loader",
					options: {
						// allowTsInNodeModules: false,
					},
				},
			},

			{
				test: /\.elm$/,
				exclude: [/elm-stuff/, /node_modules/],
				use: {
					loader: "elm-webpack-loader",
					options: {
						cwd: __dirname,
						// debug: true,
						// warn: true,
					},
				},
			},

			{
				test: /\.css$/,
				use: [
					{
						loader: MiniCssExtractPlugin.loader,
					},
					"css-loader",
					"postcss-loader",
				]
			}

		],
	},
	resolve: {
		extensions: [".ts", ".js"],
	},
	plugins: [
	  new webpack.DefinePlugin({
		API_HOST: JSON.stringify(API_HOST),
		}),
		new MiniCssExtractPlugin({
			filename: DEV_MODE ? "[name].css" :  "[name].[hash].css",
		}),
		new HtmlWebpackPlugin({
			chunks: [COMMON, ENTRY_GRAPHIQL],
			title: "Sign In",
			filename: ENTRY_GRAPHIQL + "/index.html",
			template: "src/graphiql.html",
		}),
		new HtmlWebpackPlugin({
			chunks: [COMMON, ENTRY_SIGN_IN],
			title: "Sign In",
			filename: ENTRY_SIGN_IN + "/index.html",
			template: "src/application.html",
		}),
		new HtmlWebpackPlugin({
			chunks: [COMMON, ENTRY_SIGN_UP],
			title: "Sign Ip",
			filename: ENTRY_SIGN_UP + "/index.html",
			template: "src/application.html",
		}),
		new HtmlWebpackPlugin({
			chunks: [COMMON, ENTRY_ADMIN],
			title: "Admin",
			filename: ENTRY_ADMIN + "/index.html",
			template: "src/application.html",
		}),
		new HtmlWebpackPlugin({
			chunks: [COMMON, ENTRY_INVESTOR],
			title: "Investor",
			filename: ENTRY_INVESTOR + "/index.html",
			template: "src/application.html",
		}),
	],
}

export default baseConfig
