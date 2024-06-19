import { RenderInfo } from "mini-icu4x/demo";
import * as lib from "mini-icu4x";

class EnumOption extends HTMLElement {
	static template = document.querySelector("template#enum-option").content;
	constructor(optionText) {
		super();
		let clone = EnumOption.template.cloneNode(true);

		clone.querySelector("slot[name='option-text']").parentElement.innerText = optionText;
		
		this.append(...clone.children);
	}
}

customElements.define("terminus-enum-option", EnumOption);

class EnumTemplate extends HTMLElement {
	static template = document.querySelector("template#enum").content;
	constructor(enumType) {
		super();

		let clone = EnumTemplate.template.cloneNode(true);

		let options = clone.querySelector("#options").parentNode;

		clone.querySelector("#options").remove();

		for (let value of enumType.values) {
			options.append(...(new EnumOption(value[0])).children);
		}

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(clone);
	}
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
	constructor(params){
		super();
		for (let param of params) {
			switch (param.type) {
				case "string":
				case "boolean":
				case "number":
					break;
				default:
					if (param.type in lib) {
						this.appendChild(new EnumTemplate(lib[param.type]));
					} else {
						console.error("Could not evaluate parameter of type ", param.type, " for parameter ", param.name);
					}
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