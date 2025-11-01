import { RenderInfo } from "./demo/index.mjs";

Object.values(RenderInfo.termini).toSorted((a, b) => a.funcName < b.funcName ? -1 : 1).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="demo/rendering/template.html?func=${t.funcName}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});