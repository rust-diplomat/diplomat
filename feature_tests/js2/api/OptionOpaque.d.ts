// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



export class OptionOpaque {
	

	static new(i: number): OptionOpaque?;

	static newNone(): OptionOpaque?;

	static returns(): OptionStruct?;

	static newStruct(): OptionStruct;

	static newStructNones(): OptionStruct;

	assertInteger(i: number): void;

	static optionOpaqueArgument(arg: OptionOpaque?): boolean;

}