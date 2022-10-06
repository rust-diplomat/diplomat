#!/usr/bin/env bash
set -e
../target/debug/diplomat-tool js js/api --docs js/docs/
cp ../tool/src/js/wasm.mjs js/api/diplomat-wasm.mjs
../target/debug/diplomat-tool c c/include
../target/debug/diplomat-tool cpp cpp/include --docs cpp/docs/
../target/debug/diplomat-tool dotnet dotnet/Lib/Generated/ -l dotnet/dotnet-interop-conf.toml
