#!/usr/bin/env bash

set -e
# Regens both example/ and feature_tests/

cd example && ./gen_all.sh
cd ..
cd feature_tests && ./gen_all.sh
cd ..