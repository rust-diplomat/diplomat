import { lib } from "./index.mjs";

export function multiplyPow10(power) {
	let fixedDecimal = lib.FixedDecimal.new_(10);
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