

// Base enumerator definition
export class UnimportedEnum {
    #value = undefined;

    static values = new Map([
        ["A", 0],
        ["B", 1],
        ["C", 2]
    ]);
    constructor(value) {
        if (value instanceof UnimportedEnum) {
            this.#value = value.value;
            return;
        }

        if (UnimportedEnum.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a UnimportedEnum and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return UnimportedEnum.values.get(this.#value);
    }

    static A = new UnimportedEnum("A");

    static B = new UnimportedEnum("B");

    static C = new UnimportedEnum("C");


    

}