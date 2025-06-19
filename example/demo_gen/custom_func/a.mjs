import { somelib } from "./index.mjs";

export function multiplyPow10(power) {
	let fixedDecimal = new somelib.FixedDecimal(10);
	fixedDecimal.multiplyPow10(power);
	return fixedDecimal.toString();
}

export default {
	"FixedDecimal.multiplyPow10": {
		func: multiplyPow10,
		funcName: "FixedDecimal.multiplyPow10",
		parameters: [
			{
				name: "power",
				type: "number"
			}
		]
	}
};