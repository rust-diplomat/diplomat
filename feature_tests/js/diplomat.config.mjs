// API_FOLDER is only set in workspaces/legacy/legacy.js right now:
let folder = "api";
if (global.api_folder !== undefined) {
    folder = global.api_folder;
}

export default {
    wasm_path: new URL(`./${folder}/diplomat_feature_tests.wasm`, import.meta.url),
};
