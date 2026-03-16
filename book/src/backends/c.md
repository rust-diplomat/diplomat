# C Backend

## Type Conversion
The C ABI is the backend by most backends access Diplomat's bindings (with the current exception of JS, which uses WebAssembly's ABI). So most backends will use the following types underlying most of their generation:


### Primitives
| Rust Type |   C Type   |
|-----------|------------|
|    u8     |   uint8_t  |
|    u16    |  uint16_t  |
|    u32    |  uint32_t  |
|    u64    |  uint64_t  |
|    u128   | unsupported|
|    i8     |    int8_t  |
|    i16    |    int16_t |
|    i32    |    int32_t |
|    i64    |    int64_t |
|    i128   | unsupported|
|   bool    |   bool     |
|   char    |   char32_t |
|   isize   |   intptr_t |
|   usize   |   size_t   |
|    f32    |    float   |
|    f64    |    double  |

### Struct Types
|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|  `#[diplomat::opaque] pub struct Type` | `struct Type*`    |
|           `pub struct Type`            | `struct Type{...}`|
|           `pub enum Type`              | `enum Type`       |

#### Opaques
Opaques are represented as pointers to zero-sized structs.

#### Structs
Structs are represented as simple C structs; each member is converted into the relevant C type as detailed here.

#### Enums
Diplomat does not support tagged enums, so all Rust enums are simply C enums.

### Options
|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|`Option<PrimitiveName>` or `DiplomatOption<PrimitiveName>`|`OptionPrimitiveName`|
|`Option<StructName>` or `Option<EnumName>`|`TypeName_option`|
|`Option<Box<OpaqueName>>` or `Option<&OpaqueName>`|`struct Type*`|

#### Primitive Options
Diplomat's C backend has custom structs included in `diplomat_runtime.h` which are named `Diplomat{PrimitiveName}View`.

`PrimitiveName` is the Rust primitive's name (but in UpperCamelCase). These have the following format:

```c
typedef struct OptionPrimitiveName {
    union { CPrimitiveType* ok;};
    bool is_ok;
} OptionPrimitiveName;
```

Options of Primitive slices have the same name as [Diplomat's slice types](#slices), but with `Option` prepended.

#### Struct and Enum Options

Structs and enums will generate a `TypeName_option` struct, with a similar layout as the primitive option above:

```c
typedef struct TypeName_option {
    union { TypeName ok; };
    bool is_ok;
} TypeName_option;
```

#### Opaque Options
Options of opaques are treated as nullable pointers in the C ABI.

### Results
For each function that returns a result, the C backend will generate the following struct:

```c
typedef struct FunctionName_result {
    union {
        SuccessType ok;
        ErrorType err;
    };
    bool is_ok;
} FunctionName_result;
```

Where `SuccessType` and `ErrorType` are converted from the given rust type into a C ABI friendly type, as described here.

### Slices

|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|           `&[Primitive]`               |`DiplomatPrimitiveView`|
|           `&mut[Primitive]`            |`DiplomatPrimitiveViewMut`|
|`&str` or `&DiplomatStr` or `DiplomatStrSlice` or `DiplomatUtf8StrSlice`|`DiplomatStringView`|
|`&DiplomatStr16` or `DiplomatStr16Slice`|`DiplomatString16View`|
|`&[&str] or &[DiplomatStrSlice]` or `&[DiplomatUtf8StrSlice]`|`DiplomatStringsView`|
|`&[DiplomatStr16Slice]`|`DiplomatStrings16View`|

#### Primitive Slices
Diplomat's C backend has custom structs included in `diplomat_runtime.h` which are named `Diplomat{PrimitiveName}View`. 


`PrimitiveName` is the Rust primitive's name (but in UpperCamelCase). These have the following format:

```c
typedef struct DiplomatPrimitiveNameView {
    const CPrimitiveType* data;
    size_t len;
};
```

There is also a `DiplomatPrimitiveViewMut` struct for slices with mutable pointers.

### Traits
For each trait, the C backend will generate a `Trait` struct and a `VTable` struct:

```c
typedef struct DiplomatTraitStruct_TraitName {
    void *data;
    TraitName_VTable vtable;
} DiplomatTraitStruct_TraitName;

typedef struct TraitName_VTable {
    void (*destructor)(const void*);
    size_t SIZE; size_t ALIGNMENT;
    /* ... */
} TraitName_VTable;
```

Where `data` is a pointer to the type that implements the VTable.

The VTable contains all of the pointers to the functions which implement the trait. `destructor` should be the destructor for `void* data`. `SIZE` and `ALIGNMENT` represent the size and the alignment of the `void* data` pointer.

### Callbacks

Callback arguments generate a struct:

```c
typedef struct DiplomatCallback_FunctionName_ParameterName {
    const void* data;
    void (*run_callback)(const void*, /*...*/);
    void (*destructor)(const void*);
} DiplomatCallback_FunctionName_ParameterName;
```

`data` represents the pointer to the `DiplomatCallback_FunctionName_ParameterName` struct in C. `run_callback` is a pointer to the C function that implements the callback, and `destructor` is a pointer to the destructor for `data`. `destructor` and `data` can both be nullptrs if there is no data associated with the callback (i.e., if in C++ you are only passing in a function, not a function and a pointer).

{{supports("c")}}