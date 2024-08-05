import { RenderInfo, lib } from "../index.mjs";
import { TerminusRender } from "./rendering.mjs";

let params = new URLSearchParams(window.location.search);

let func = params.get("func");

let terminus = new TerminusRender(lib, (param, updateParamEvent) => {
    throw Error(`Unrecognized parameter type ${param}`);
}, RenderInfo.termini[func]);

document.getElementById("render").appendChild(terminus);