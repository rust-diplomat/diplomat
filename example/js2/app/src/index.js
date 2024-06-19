import { RenderInfo } from "mini-icu4x/demo";
import * as lib from "mini-icu4x";

class ParameterTemplate extends HTMLElement {
	constructor(template, ...args) {
		super();
		let clone = template.cloneNode(true);

		this.initialize(clone, ...args);
		
		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(clone);
	}

	initialize(clone) {

	}
}

class BooleanTemplate extends ParameterTemplate {
	static template = document.querySelector("template#boolean").content;
	constructor() {
		super(BooleanTemplate.template);
	}
}

customElements.define("terminus-param-boolean", BooleanTemplate);

class NumberTemplate extends ParameterTemplate {
	static template = document.querySelector("template#number").content;
	constructor() {
		super(NumberTemplate.template);
	}
}

customElements.define("terminus-param-number", NumberTemplate);

class StringTemplate extends ParameterTemplate {
	static template = document.querySelector("template#string").content;
	constructor() {
		super(StringTemplate.template);
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

class EnumTemplate extends ParameterTemplate {
	static template = document.querySelector("template#enum").content;

	constructor(enumType) {
		super(EnumTemplate.template, enumType);
	}

	initialize(clone, enumType) {
		let options = clone.querySelector("#options").parentNode;

		clone.querySelector("#options").remove();

		for (let value of enumType.values) {
			options.append(...(new EnumOption(value[0])).children);
		}
	}
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
	#params = {};

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

			// newChild.addEventListener("input", this.input.bind(this, paramName));

			newChild.appendChild(paramName);
			this.appendChild(newChild);
		}
	}

	input(paramName, event) {
		this.#params[paramName] = event.target.value;
	}
}

customElements.define("terminus-params", TerminusParams);

class TerminusRender extends HTMLElement {
	static template = document.querySelector("template#terminus").content;

	#func = null;
	constructor(func, params) {
		super();
		let clone = TerminusRender.template.cloneNode(true);

		this.#func = func;

		let button = clone.querySelector("button[onclick='submit()']");
		button.setAttribute("onclick", "");
		button.addEventListener("click", this.submit);

		let funcText = document.createElement("span");
		funcText.slot = "funcName";
		funcText.innerText = this.#func.name;
		this.appendChild(funcText);

		let parameters = new TerminusParams(params);
		parameters.slot = "parameters";
		this.appendChild(parameters);

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(clone);
	}

	submit() {
		alert("");
		this.#func();
	}
}

customElements.define("terminus-render", TerminusRender);

RenderInfo.termini.forEach((t) => {
	document.body.appendChild(new TerminusRender(t.func, t.parameters));
});