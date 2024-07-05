import { RenderInfo } from "mini-icu4x/demo";
import * as lib from "mini-icu4x";
import { TerminusRender } from "./rendering.mjs";

RenderInfo.termini.forEach((t) => {
	document.getElementById("termini").appendChild(new TerminusRender(t, lib));
});