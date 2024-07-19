import * as lib from "./js/index.mjs";
import { RenderInfo } from "./index.mjs";
import { TerminusRender } from "./rendering.mjs";

var nav = document.getElementById("termini-navigation");

var currentlySelected = null;
var navSelected = null;

RenderInfo.termini.forEach((t) => {
    var renderOut = new TerminusRender(t, lib, null);
    document.getElementById("termini").appendChild(renderOut);
    
    renderOut.classList.add("tab-pane");
    renderOut.role = "tabpanel";
    
    var outNav = document.createElement("li");
    outNav.innerHTML = `<button class="nav-link ${navSelected === null ? "active" : ""}" id="${t.funcName}-nav" data-toggle="pill" role="tab" aria-selected="${navSelected === null}" aria-controls="${t.funcName}">${t.funcName}</button>`;
    outNav.classList.add("nav-item");
    outNav.addEventListener("click", () => {
        currentlySelected.classList.remove("active");
        navSelected.classList.remove("active");

        renderOut.classList.add("active");
        outNav.children[0].classList.add("active");

        navSelected = outNav.children[0];
        currentlySelected = renderOut;
    });

    nav.appendChild(outNav);
    
    if (currentlySelected === null) {
        renderOut.classList.add("active");
        currentlySelected = renderOut;
        navSelected = outNav.children[0];
    }
});