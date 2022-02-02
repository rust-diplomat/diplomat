#!/usr/bin/env bash
cargo build --all
mkdir -p Lib/lib/Debug/linux-x64/
cp ../../target/debug/libdiplomat_feature_tests.so Lib/lib/Debug/linux-x64/
rm Lib/Generated/*
../../target/debug/diplomat-tool dotnet Lib/Generated/ -l dotnet-interop-conf.toml --entry ../src/lib.rs
dotnet test
