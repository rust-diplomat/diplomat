// API_FOLDER is only set in ava.legacy.config.js right now:
let folder = API_FOLDER;
if (folder === undefined) {
    folder = "api";
}

export default {
    // TODO: Swap out based on current test.
    wasm_path: new URL(`./${API_FOLDER}/diplomat_feature_tests.wasm`, import.meta.url),
};
