#!/usr/bin/env bash
set -e
../target/debug/diplomat-tool js js/lib/api --docs js/lib/docs/ --docs-base-urls=*:https://unicode-org.github.io/icu4x-docs/doc/
../target/debug/diplomat-tool c c/include
../target/debug/diplomat-tool c2 c2/include
../target/debug/diplomat-tool cpp cpp/include --docs cpp/docs/ --docs-base-urls=*:https://unicode-org.github.io/icu4x-docs/doc/