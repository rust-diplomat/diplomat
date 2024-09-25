import { lib } from "./index.mjs";

function multiplyPow10(power) {
	let fixedDecimal = lib.FixedDecimal.new(10);
	fixedDecimal.multiplyPow10(power);
	return fixedDecimal.toString();
}

export default {
	"FixedDecimal.multiplyPow10": {
		func: multiplyPow10,
		funcName: "multiplyPow10",
		parameters: [
			{
				name: "power",
				type: "number"
			}
		]
	}
};