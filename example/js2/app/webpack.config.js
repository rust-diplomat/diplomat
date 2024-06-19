import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
	entry: './src/index.js',
	resolve: {
	  extensions: ['.js'],
	  fallback: {
		// Required to not error when `fs` can't be bundled, which is okay because
		// we're only anticipating webpacking for the browser which uses `fetch`
		// anyways.
		"fs": false,
	  },
	},
	mode: "production",
	output: {
	  filename: 'bundle.js',
	  path: path.resolve(__dirname, 'dist'),
	},
	experiments: {
	  // Enables using modules with top-level awaits (mainly wasm.mjs)
	  topLevelAwait: true,
	}
  };