class ParameterTemplate extends HTMLElement {
	default = null;

	static baseTemplate = document.querySelector("#parameter").content;
	constructor(template, ...args) {
		super();
		let baseClone = ParameterTemplate.baseTemplate.cloneNode(true);

		let clone = template.cloneNode(true);

		this.initialize(clone, ...args);

		let input = clone.querySelector("*[data-oninput]");
		if (input !== null) {
			input.addEventListener("input", this.input.bind(this));
		}
		
		clone.slot = "parameter";
		baseClone.appendChild(clone);

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(baseClone);
	}

	getEventValue(event) {
		return event.target.value;
	}

	input(event) {
		this.dispatchEvent(new CustomEvent("parameter-input", {
			detail: this.getEventValue(event)
		}));
	}

	initialize(clone) {

	}
}

customElements.define("terminus-param", ParameterTemplate);

class BooleanTemplate extends ParameterTemplate {
	default = false;
	static template = document.querySelector("template#boolean").content;
	constructor() {
		super(BooleanTemplate.template);
	}
}

customElements.define("terminus-param-boolean", BooleanTemplate);

class NumberTemplate extends ParameterTemplate {
	default = 0;
	static template = document.querySelector("template#number").content;
	constructor() {
		super(NumberTemplate.template);
	}
	
	getEventValue(event) {
		return parseFloat(event.target.value);
	}
}

customElements.define("terminus-param-number", NumberTemplate);

class StringTemplate extends ParameterTemplate {
	default = "";
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

	#enumType;
	constructor(enumType) {
		super(EnumTemplate.template, enumType);
		this.#enumType = enumType;
	}

	initialize(clone, enumType) {
		let options = clone.querySelector("*[data-options]");

		this.default = enumType.values.values().next().value;

		for (let value of enumType.values) {
			options.append(...(new EnumOption(value[0])).children);
		}
	}

	getEventValue(event) {
		return this.#enumType[event.target.value];
	}
}

customElements.define("terminus-param-enum", EnumTemplate);

class TerminusParams extends HTMLElement {
	#params = [];

	constructor(params, lib){
		super();

		for (var i = 0; i < params.length; i++) {
			let param = params[i];
			let paramName = document.createElement("span");
			paramName.slot = "param-name";
			paramName.innerText = param.name;

			var newChild;

			switch (param.type) {
				case "string":
					newChild = new StringTemplate();
					this.#params[i] = "";
					break;
				case "boolean":
					newChild = new BooleanTemplate();
					this.#params[i] = false;
					break;
				case "number":
					newChild = new NumberTemplate();
					this.#params[i] = 0;
					break;
				default:
					if (param.type in lib && "values" in lib[param.type]) {
						newChild = new EnumTemplate(lib[param.type]);
						this.#params[i] = newChild.default
					} else {
						console.error("Could not evaluate parameter of type ", param.type, " for parameter ", param.name);
						return;
					}
					break;
			}

			newChild.addEventListener("parameter-input", this.input.bind(this, i));
			this.#params[i] = newChild.default;

			newChild.appendChild(paramName);
			this.appendChild(newChild);
		}
	}

	input(paramIdx, event) {
		this.#params[paramIdx] = event.detail;
	}

	get paramArray() {
		return this.#params;
	}
}

customElements.define("terminus-params", TerminusParams);

export class TerminusRender extends HTMLElement {
	static template = document.querySelector("template#terminus").content;

	#func = null;
	#parameters;
	#output;
	constructor(terminus, library) {
		super();
		let clone = TerminusRender.template.cloneNode(true);

		this.id = terminus.funcName;

		this.#func = terminus.func;

		let button = clone.querySelector("*[data-submit]");
		button.addEventListener("click", this.submit.bind(this));

		let funcText = document.createElement("span");
		funcText.slot = "func-name";
		funcText.innerText = terminus.funcName;
		this.appendChild(funcText);

		this.#parameters = new TerminusParams(terminus.parameters, library);
		this.#parameters.slot = "parameters";
		this.appendChild(this.#parameters);

		this.#output = document.createElement("span");
		this.#output.slot = "output";

		this.appendChild(this.#output);

		const shadowRoot = this.attachShadow({ mode: "open" });
		shadowRoot.appendChild(clone);
	}

	submit() {
		try {
			this.#output.innerText = this.#func(...this.#parameters.paramArray);
		} catch(e) {
			this.#output.innerText = e;
		}
	}
}

customElements.define("terminus-render", TerminusRender);