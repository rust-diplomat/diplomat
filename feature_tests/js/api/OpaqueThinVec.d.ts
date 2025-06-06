// generated by diplomat-tool
import type { OpaqueThin } from "./OpaqueThin"
import type { OpaqueThinIter } from "./OpaqueThinIter"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



export class OpaqueThinVec {
    /** @internal */
    get ffiValue(): pointer;


    [Symbol.iterator](): OpaqueThinIter;

    len(): number;

    get(idx: number): OpaqueThin | null;

    get first(): OpaqueThin | null;

    constructor(a: Array<number>, b: Array<number>);
}