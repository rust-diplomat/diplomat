import { RenderInfo, lib } from "../index.mjs";
import { initialize } from "./rendering.mjs";

let params = new URLSearchParams(window.location.search);

let func = params.get("func");

initialize(lib);

document.getElementById("render").attachTerminus(RenderInfo.termini[func]);