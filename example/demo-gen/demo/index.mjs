
export * as ICU4XFixedDecimalFormatter from "./ICU4XFixedDecimalFormatter.mjs";

export * as ICU4XFixedDecimal from "./ICU4XFixedDecimal.mjs";


export const RenderInfo = {
	termini: [
		{
			func: ICU4XFixedDecimalFormatter.formatWrite,
			parameters: [
				
				{
					name: "name",
					type: "String"
				},
				
				{
					name: "grouping_strategy",
					type: "ICU4XFixedDecimalGroupingStrategy"
				},
				
				{
					name: "some_other_config",
					type: "boolean"
				},
				
				{
					name: "v",
					type: "number"
				}
				
			]
		},
		
		{
			func: ICU4XFixedDecimal.toString,
			parameters: [
				
				{
					name: "v",
					type: "number"
				}
				
			]
		}
		],
};