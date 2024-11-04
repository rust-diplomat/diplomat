"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var bench_harness_1 = require("@stdlib/bench-harness");
var Opaque_1 = require("../api/Opaque");
(0, bench_harness_1.default)("Init opaque", function () {
    Opaque_1.Opaque.new_();
});
