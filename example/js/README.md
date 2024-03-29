# Diplomat NPM Demo package

This directory (`example/js/`) demonstrates how you would go about releasing an NPM package that uses Diplomat bindings for JavaScript. It contains three subdirectories, each a sample NPM package:
* `lib/`: Exports some demo bindings to ICU4X generated by Diplomat.
* `app/`: Contains a basic web demo and Node CLI app using functionality from `lib/`.
* `app-webpack/`: Contains a basic web demo with webpack using functionality from `lib/`.

This file documents how to test out the demos after cloning the repo. Details on how to test changes are described below in the FAQs.

## Using the demo library (`lib/`)

1. Build diplomat tool to the most recent version.
    ```sh
    # /diplomat
    cargo build
    ```

2. Invoke the newly built diplomat tool to generate fresh `.js` and `.d.ts` bindings.
    ```sh
    # /diplomat
    ./regen_integration_tests.sh
    ```

3. Build `wasm` binaries that the demo library will use.
    ```sh
    # /diplomat
    cargo build --target wasm32-unknown-unknown
    ```

4. Copy `diplomat_example.wasm` into `lib/`.
    ```sh
    # /diplomat
    cp ./target/wasm32-unknown-unknown/debug/diplomat_example.wasm ./example/js/lib/diplomat-lib.wasm
    ```

All at once:
```sh
# /diplomat
cargo build
cargo build --target wasm32-unknown-unknown
./regen_integration_tests.sh
cp ./target/wasm32-unknown-unknown/debug/diplomat_example.wasm ./example/js/lib/diplomat-lib.wasm
```

Now the demo library can be published or otherwise consumed by other packages.

## Using the Webpack app (`app-webpack/`)

When first cloning the repo, install webpack and TypeScript dependencies, and copy the contents of `example/js/lib/`:
```sh
# /diplomat/example/js/app-webpack
npm install
```

5. Run webpack, which automatically does TS -> JS transpilation (no need for `tsc`).
    ```sh
    # /diplomat/example/js/app-webpack
    npm run build
    ```
    
At this point, you can view `index.html` in the browser. I use [LiveServer](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) for this.

To remove the auto generated code, do
```sh
# /diplomat/example/js/app-webpack
npm run clean
```

## Using the non-webpack app (`app/`)

The instructions for the non-webpack app are nearly identical to the webpack version.

First, install npm dependencies, which are just `readline-sync` for the Node app, and the demo living in `example/js/lib/`.
```sh
# /diplomat/example/js/app
npm install
```

6. Transpile the TypeScript into JavaScript (this just aliases `tsc`).
    ```sh
    # /diplomat/example/js/app
    npm run build
    ```

For the web demo, just view `index.html` in the browser.

For the Node CLI demo, run the following:
```sh
# /diplomat/example/js/app
npm run start
```

To remove the auto generated code, do
```sh
# /diplomat/example/js/app-webpack
npm run clean
```
# FAQs

## I made a change in `example/js/app-webpack/`

Go to step 5.

## I made a change in `example/js/app/`

Go to step 6.

## I made a change in `example/src`

Go to step 3.

## I made a change in `tool/`

Go to step 1.

## Why do I have to do all these things manually

Hopefully soon we'll integrate `cargo make`. Until then we have this. Additionally, we also plan to support a smoother experience for setting up Diplomat on external NPM packages, not just the demos in `example/js`.
