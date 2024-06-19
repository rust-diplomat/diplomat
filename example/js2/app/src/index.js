import { RenderInfo } from "mini-icu4x/demo";

class EnumTemplate extends HTMLElement {
	static template = document.querySelector("template#enum").content;
	constructor(choices) {
		super();

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(EnumTemplate.template.cloneNode(true));
	}
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
	constructor(params){
		super();
		for (let param of params) {
			switch (param.type) {
				// Assume it's an enum if we don't recognize the type.
				default:
					this.appendChild(new EnumTemplate());
					break;
			}
		}
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
	document.body.appendChild(new TerminusRender(t.func.name, t.parameters));
});