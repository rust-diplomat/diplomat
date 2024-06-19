import { RenderInfo } from "mini-icu4x/demo";
import * as lib from "mini-icu4x";


class BooleanTemplate extends HTMLElement {
	static template = document.querySelector("template#boolean").content;
	constructor() {
		super();

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(BooleanTemplate.template.cloneNode(true));
	}
}

customElements.define("terminus-param-boolean", BooleanTemplate);


class NumberTemplate extends HTMLElement {
	static template = document.querySelector("template#number").content;
	constructor() {
		super();

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(NumberTemplate.template.cloneNode(true));
	}
}

customElements.define("terminus-param-number", NumberTemplate);

class StringTemplate extends HTMLElement {
	static template = document.querySelector("template#string").content;
	constructor() {
		super();

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(StringTemplate.template.cloneNode(true));
	}
}

customElements.define("terminus-param-string", StringTemplate);

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
			let paramName = document.createElement("span");
			paramName.slot = "param-name";
			paramName.innerText = param.name;

			var newChild;

			switch (param.type) {
				case "string":
					newChild = new StringTemplate();
					break;
				case "boolean":
					newChild = new BooleanTemplate();
					break;
				case "number":
					newChild = new NumberTemplate();
					break;
				default:
					if (param.type in lib) {
						newChild = new EnumTemplate(lib[param.type]);
					} else {
						console.error("Could not evaluate parameter of type ", param.type, " for parameter ", param.name);
						return;
					}
					break;
			}

			newChild.appendChild(paramName);
			this.appendChild(newChild);
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