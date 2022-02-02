#!/usr/bin/env bash
../target/debug/diplomat-tool js js --docs js/docs/
../target/debug/diplomat-tool c c/
../target/debug/diplomat-tool cpp cpp/include --docs cpp/docs/
../target/debug/diplomat-tool dotnet dotnet/Lib/Generated/ -l dotnet/dotnet-interop-conf.toml
