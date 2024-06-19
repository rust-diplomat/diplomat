import { RenderInfo } from "mini-icu4x/demo";

class TerminusParams extends HTMLElement {
	constructor(params){
		super();
	}
}

customElements.define("terminus-params", TerminusParams);

class TerminusRender extends HTMLElement {
	static template = document.querySelector("template#terminus").content;
	constructor(funcName, params) {
		super();

		let funcText = document.createElement("span");
		funcText.slot = "funcName";
		funcText.innerText = funcName;
		this.appendChild(funcText);

		let parameters = new TerminusParams(params);
		parameters.slot = "parameters";
		this.appendChild(parameters);

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(TerminusRender.template.cloneNode(true));
	}
}

customElements.define("terminus-render", TerminusRender);

RenderInfo.termini.forEach((t) => {
	document.body.appendChild(new TerminusRender(t.func.name));
});