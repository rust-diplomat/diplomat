import { RenderInfo, lib } from "../index.mjs";
import { initialize } from "./rendering.mjs";

let params = new URLSearchParams(window.location.search);

initialize(lib);

let func = params.get("func");
document.getElementById("render").attachTerminus(RenderInfo.termini[func]);