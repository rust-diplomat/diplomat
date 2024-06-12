import test from "ava";

import { ICU4XFixedDecimalFormatter } from "demo-gen"; 

test("Format write exists.", t => {
	t.notThrows(ICU4XFixedDecimalFormatter.formatWrite);
});