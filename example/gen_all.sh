#!/usr/bin/env bash
../target/debug/diplomat-tool js js/ --docs js/docs/ --docs-base-urls=icu:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=fixed_decimal:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_plurals:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_properties:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_codepointtrie:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_uniset:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider_blob:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider_fs:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_segmenter:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider:https://unicode-org.github.io/icu4x-docs/doc/  --docs-base-urls=icu_testdata:https://unicode-org.github.io/icu4x-docs/doc/
../target/debug/diplomat-tool c c/
../target/debug/diplomat-tool cpp cpp/include --docs cpp/docs/ --docs-base-urls=icu:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=fixed_decimal:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_plurals:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_properties:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_codepointtrie:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_uniset:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider_blob:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider_fs:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_segmenter:https://unicode-org.github.io/icu4x-docs/doc/ --docs-base-urls=icu_provider:https://unicode-org.github.io/icu4x-docs/doc/  --docs-base-urls=icu_testdata:https://unicode-org.github.io/icu4x-docs/doc/