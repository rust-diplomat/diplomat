import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
	entry: { 
		index: [ 
			'./src/js/index.mjs',
			'./src/scss/main.scss'
		 ]
	},
	module: {
		rules: [
			{
				test: /\.(scss)$/,
				use: [
					{
					loader: 'style-loader',
					},
					{
					loader: 'css-loader'
					},
					{
					loader: 'postcss-loader',
					options: {
						postcssOptions: {
						plugins: () => [
							require('autoprefixer')
						]
						}
					}
					},
					{
					loader: 'sass-loader'
					}
				]
			}
		]
	},
	resolve: {
	  extensions: ['.mjs'],
	  fallback: {
		// Required to not error when `fs` can't be bundled, which is okay because
		// we're only anticipating webpacking for the browser which uses `fetch`
		// anyways.
		"fs": false,
	  },
	},
	mode: "production",
	output: {
	  filename: 'bundle.mjs',
	  path: path.resolve(__dirname, 'public/dist'),
	},
	experiments: {
	  // Enables using modules with top-level awaits (mainly wasm.mjs)
	  topLevelAwait: true,
	}
  };