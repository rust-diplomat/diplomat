import { RenderInfo } from "./demo/index.mjs";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="demo/${t.html}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});