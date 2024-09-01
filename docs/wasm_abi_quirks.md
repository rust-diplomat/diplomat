# Rust (legacy) Wasm ABI quirks


The Rust Wasm ABI is rather strange, and does not follow [tool conventions]. There are plans to move Rust over to [something following conventions][rust-plans] (currently previewable as `-Zwasm-c-abi=spec`), as well as [plans for more efficient standardized ABIs][better-wasm]. Until those things settle down, we are using the "legacy" Rust Wasm ABI, which is highly quirky.

Diplomat has issue [#661] tracking the new conventions.


## Wasm parameter types

While at the Rust level and the LLVM IR level there are multiple different parameter types, Wasm itself only accepts two parameter/return types: `i32` and `i64`. The JS-Wasm interface maps `Number` to `i32` and `BigInt` to `i64`.

This means that a function accepting a single `u8` parameter will still show up as accepting an `i32` in the WAT:

```rust
#[no_mangle]
pub extern "C" fn inout(x: u8) -> u8 { 1 }
```

This has LLVM IR:

```llvm
define dso_local zeroext i8 @inout(i8 zeroext %x) unnamed_addr #0 { ... }
```

And Wasm/WAT:

```wat
(type $t0 (func (param i32) (result i32)))
(func $inout (export "inout") (type $t0) (param $p0 i32) (result i32) ...)
```


Our current code does not correctly handle large `u32`s, which will get turned into negative numbers across FFI when passed as parameters/return types.

We may additionally have gaps in our current code around `u64`s; since integers do not implicitly convert to `BigInt` (and if they do, we may be doing so erroneously). In particular, we need to check if padding code handles this correctly.



## Return values

This is not a Rust-specific quirk, but rather how Wasm works in non-multivalue mode. Switching to [multivalue], possible in Rust with `-C target-feature=+multivalue` will get past this, but Diplomat would need to be updated to produce multivalue-capable bindings.


All Wasm functions have a signature that looks like `fn(integer, integer, integer, ...) -> integer` (where the return type is optional). There are no non-integer types Wasm FFI at the lowest level: Pointers are integer indices into the wasm memory buffer, slices are a pair of integers, and structs are a bunch of integers (more on this later). As mentioned in the previous section, Wasm only really distinguishes between `i32` and `i64` here, everything else is converted.

As might be clear from this general signature, Wasm is only capable of returning _scalars_ over FFI (from the foreign language to JS). A scalar is something equivalent to a single integral primitive, which in wasm becomes all integer types[^1], booleans, `char`s, and pointers. Aggregates transitively containing multiple integral primitives, like structs with more than one field and slices, are not scalars. Aggregates containing a single scalar value are equivalent to that contained scalar in all FFI matters[^2].


This means that there is no way to have a signature like `fn(..) -> (integer, integer)`, for example when returning a slice or a two-field struct across FFI. Instead, for a struct like this:

```rust
// size 8, align 4
pub struct Big {
    a: u8, // size 1, offset 0
    b: u16, // size 2, offset 2
    c: u64, // size 8, offset 8
}
#[no_mangle]
pub extern "C" fn returns_big(arg1: u8, arg2: u8) -> Big { ... }
```

Instead, in this case, Wasm uses an "outparam" solution for this. Sufficient space for the struct must be allocated on the Wasm heap, and a pointer to this space should be passed in as the _last_ parameter for this function. Once the function is called the value can be read back.

This code generates the LLVM IR:

```llvm
; produced by rustc with -Zwasm-c-abi=legacy
define dso_local %Big @returns_big(i8 zeroext %arg1, i8 zeroext %arg2) unnamed_addr #0 { ... }
```

and the Wasm/WAT:

```wat
(type $t0 (func (param i32 i32 i32)))
(func $returns_big (export "returns_big") (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) ...)
```

Note that in the Wasm, there is no return value: instead there is an additional parameter. When calling this from JS, this will be called as `wasm.returns_big(arg1, arg2, outParam)`.

In Diplomat this is typically managed by DiplomatReceiveBuf, but in raw pseudocode the thing that needs to be done is roughly:

```js
// Allocate space for the struct with the right size/alignment
let structAlloc = wasm.diplomat_alloc(8, 4);

wasm.returns_big(arg1, arg2, structAlloc);

// Read the fields from wasm memory (ptrRead reads from wasm memory, given a memory location and a size)
let field0 = ptrRead(structAlloc + 0, 1);
let field1 = ptrRead(structAlloc + 2, 2);
let field2 = ptrRead(structAlloc + 8, 8);

// Clean up
wasm.diplomat_free(structAlloc);
```

There [may be bugs around when and where Diplomat chooses to apply DiplomatReceiveBuf](https://github.com/rust-diplomat/diplomat/issues/662).

## Aggregates in parameters

This is where stuff gets a bit weird.

### Direct vs indirect

There are a couple common ways that aggregates get passed as function parameters at the ABI level in different ABIs.

One is "direct" passing: where every field of the aggregate becomes a single parameter. For example, for the following struct

```rust
#[repr(C)]
struct MyStruct {
    a: u8,
    // 3 bytes padding
    b: u32
}

#[no_mangle]
extern "C" fn takes_struct(s: MyStruct) {...}
```

"direct" passing would mean that `takes_struct` is invoked as `wasm.takes_struct(a, b)`.

The LLVM IR to get this result in Wasm looks something like:

```llvm
; produced by rustc with (default) -Zwasm-c-abi=legacy
define dso_local void @takes_struct(i8 %x.0, i32 %x.1) unnamed_addr #0 { ... }
```

And the Wasm would look like:

```wat
(type $t0 (func (param i32 i32)))
(func $takes_struct (export "takes_struct") (type $t0) (param $p0 i32) (param $p1 i32) ...)
```

(Note that all parameter types are turned into `i32` in the Wasm/WAT, see the section above on "Wasm parameter types" for more)

This would work through layers of indirection; e.g. a struct with `MyStruct` as its only field would get passed similarly. The idea is that you pick out each scalar value transitively contained in the struct and pass them as arguments one by one.

Another is "indirect" passing, which works similarly to the return value thing where the struct is passed as a pointer. It can be allocated on the heap or the stack, however being able to manipulate the Wasm stack from JS is tricky so it's best to just heap-allocate.

This would end up being invoked something like:

```js
// Allocate space for the struct with the right size/alignment
let structAlloc = wasm.diplomat_alloc(8, 4);

ptrWrite(structAlloc, a, 1);
ptrWrite(structAlloc + 4, b, 4);

wasm.takes_struct(structAlloc);

// Clean up
wasm.diplomat_free(structAlloc);
```

The LLVM IR to get this result in Wasm looks something like:

```llvm
; produced by webassembly-clang:
%struct.MyStruct = type { i8, i32 }
define dso_local void @takes_struct(ptr noundef byval(%struct.MyStruct) align 4 %0) #0 { ... }

; produced by rustc with -Zwasm-c-abi=spec
define dso_local void @takes_struct(ptr byval([8 x i8]) align 4 %x) unnamed_addr #0 { ... }
```

And the Wasm would look like:

```wat
(type $t0 (func (param i32)))
(func $takes_struct (export "takes_struct") (type $t0) (param $p0 i32) ...)
```

The [tool conventions] ask for "direct" passing for scalars (including structs that transitively contain a single scalar), and "indirect" for everything else.

### What Rust does

However, Rust doesn't quite do this. Rust _does_ use "direct" passing for scalars and aggregates transitively containing two scalars. However, for aggregates transitively containing more than two scalars, Rust does something that I will call "**padded direct**".

In "padded direct" mode, every transitive scalar field is passed through as an argument just like "direct", however _padding is passed through as well_.

`MyStruct` above contains 3 bytes of padding between the two fields. In "padded direct" mode, this would be invoked as `wasm.takes_struct(a, 0, 0, 0, b)`, with each 0 subbing in for padding. The LLVM IR would look something like:

```llvm
; Edited from IR produced by rustc with (default) -Zwasm-c-abi=legacy
%MyStruct = type { i8, [3 x i8], i32 }
define dso_local void @takes_struct(%MyStruct %0) unnamed_addr #0 { ... }
```

Note that since `MyStruct` only has two fields, in practice it would not get passed in "padded direct" mode, the above is an illustrative example.

### Dealing with typed padding

A crucial thing about "padded direct" mode is that the _type_ of padding matters. This is something most do not have to think about in low level programming: padding is typically just a number of bytes. However at the LLVM level, "two bytes of `i8`-padding" is technically different from "1 byte of `i16`-padding", and that's relevant here

For example, with the following struct:

```rust
pub struct Big {
    a: u8,
    // 1 byte padding
    b: u16,
    // 4 bytes padding
    c: u64,
}
#[no_mangle]
pub extern "C" fn big(x: Big) {}
```

The LLVM IR looks something like:

```llvm
; produced by rustc with (default) -Zwasm-c-abi=legacy
%Big = type { i8, [1 x i8], i16, [2 x i16], i64 }
define dso_local void @big(%Big %0) unnamed_addr #0 { ... }
```

And the Wasm would look like:

```wat
(type $t0 (func (param i32 i32 i32 i32 i32 i64)))
(func $big (export "big") (type $t0)  (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i64) ...)
```

And it gets invoked as `wasm.big(a, 0, b, 0, 0, c)`. Even though there are four bytes of padding for the second padding segment, it's treated as `i16`-padding, which means only two fields are needed. The padding type appears to just be the alignment of the preceding field.

### Nested structs

The "contains two scalar fields" rule is only applied at the top level when an aggregate is passed as an argument. If a struct with additional fields contains a struct with two scalar fields, the padding of that internal struct does become relevant again. Effectively, the actual topology of the struct is mostly irrelevant for when it is being passed over FFI, just the transitive list of fields, and any alignment/size constraints.



## Unions in parameters

Unions are passed as `size / align` parameters, each of size `align` for a union with size `size` and alignment `align` (This is calculated by taking the max size and max align of the two fields).

For example, with the following union from Diplomat:

```rust
#[repr(C)]
union DiplomatResultValue<T, E> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<E>,
}

/// A [`Result`]-like type that can be passed across the FFI boundary
/// as a value. Used internally to return [`Result`]s and [`Option`]s
/// from functions.
#[repr(C)]
pub struct DiplomatResult<T, E> {
    value: DiplomatResultValue<T, E>,
    pub is_ok: bool,
}

/// A type to represent Option<T> over FFI.
///
/// Used internally to handle `Option<T>` arguments and return types, and needs to be
/// used explicitly for optional struct fields.
pub type DiplomatOption<T> = DiplomatResult<T, ()>;
```

`DiplomatOption<MyStruct>` (size 8, alignment 4) will be passed as two `u32` parameters, followed by an `u8` parameter for the `bool`, and three more padding `u8` parameters.


Wasm is little-endian. For the following code:

```rust
#[repr(C)]
#[derive(Debug)]
pub struct Inner {
    x: u8,
    y: u16,
    z: u32,
}
#[no_mangle]
pub extern "C" fn opt(x: DiplomatOption<Inner>) {
    let val = unsafe { x.value.ok };
    log(&format!("{val:#x?}"));
}
```

The call `wasm.opt(0x12345678, 0x9ABCDEF0, 1, 0, 0, 0)` (with the two u32 values corresponding to the `DiplomatResultValue` union) produces

```rust
Inner {
    x: 0x78,
    y: 0x1234,
    z: 0x9abcdef0,
}
```

with LLVM IR:

```llvm
; produced by rustc with (default) -Zwasm-c-abi=legacy
%Inner = type { i8, [1 x i8], i16, i32 }
%"DiplomatResult<Inner, ()>" = type { %"DiplomatResultValue<Inner, ()>", i8, [3 x i8] }
%"DiplomatResultValue<Inner, ()>" = type { [2 x i32] }

define dso_local void @opt(%"DiplomatResult<Inner, ()>" %0) unnamed_addr #0 { ... }
```

And the Wasm would look like:

```wat
(type $t0 (func (param i32 i32 i32 i32 i32 i32)))
(func $opt (export "opt") (type $t0)  (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32) (param $p5 i32) ...)
```



 [tool conventions]: https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md
 [better-wasm]: https://github.com/WebAssembly/tool-conventions/issues/88
 [#661]: https://github.com/rust-diplomat/diplomat/issues/661
 [rust-plans]: https://github.com/rust-lang/rust/pull/117919
 [multivalue]: https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/


[^1]: Potentially with the exception of u64 in some cases? I haven't investigated this. Wasm gets weird around BigInt stuff.
[^2]: Technically it is possible to break this equality by overriding the alignment of the struct, introducing padding. Diplomat currently doesn't handle this, and we may try to forbid using the JS backend with alignment-overridden structs