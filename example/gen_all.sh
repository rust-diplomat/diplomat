#!/usr/bin/env bash
set -e
../target/debug/diplomat-tool js js/api --docs js/docs/ --docs-base-urls=*:https://unicode-org.github.io/icu4x-docs/doc/ 
../target/debug/diplomat-tool c c/include
../target/debug/diplomat-tool cpp cpp/include --docs cpp/docs/ --docs-base-urls=*:https://unicode-org.github.io/icu4x-docs/doc/