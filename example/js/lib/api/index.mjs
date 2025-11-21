

export { FixedDecimalFormatterOptions } from "./FixedDecimalFormatterOptions.mjs"

export { DataProvider } from "./DataProvider.mjs"

export { FixedDecimalFormatter } from "./FixedDecimalFormatter.mjs"

export { FixedDecimal } from "./FixedDecimal.mjs"

export { Locale } from "./Locale.mjs"

export { FixedDecimalGroupingStrategy } from "./FixedDecimalGroupingStrategy.mjs"

import wasm from "./diplomat-wasm.mjs";
import {FUNCTION_PARAM_ALLOC, internalConstructor} from "./diplomat-runtime.mjs";

FUNCTION_PARAM_ALLOC.reserve(internalConstructor, wasm, 16);
