import { RenderInfo } from "mini-icu4x/demo";
import * as lib from "mini-icu4x";
import { TerminusRender, TerminusNavigation } from "./rendering.mjs";
import bootstrap from "bootstrap";

RenderInfo.termini.forEach((t) => {
	document.getElementById("termini").appendChild(new TerminusRender(t, lib));
	document.getElementById("termini-navigation").appendChild(new TerminusNavigation(t));
});