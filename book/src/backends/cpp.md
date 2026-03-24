# C++ Backend

## Type Conversion
The C++ type conversions are built upon the [C ABI](./c.md#type-conversion), with some additionall STL types to make working with the C ABI more intuitive from a C++ standpoint.


### Primitives
See [C Primitives](./c.md#primitives).

### Struct Types
|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|  `#[diplomat::opaque] pub struct Type` | `class Type {...}` |
|           `pub struct Type`            | `class Type{...}`|
|           `pub enum Type`              | `enum Type`       |

### Opaques
Opaques are treated as classes that wrap their C ABI pointer, but upon return either wrapped in a `std::unique_ptr<Type>` or a simple `Type*` pointer.

#### Structs
Structs are represented as C++ structs with methods. Each C++ struct is converted from the C ABI into its relevant C++ type.

#### Enums
See [C Enums](./c.md#enums).

### Options
All options (with the exception of opaques) are represented as `std::optional<InnerType>`.

#### Opaque Options
These are nullable pointers.

### Results
All results are returned as `diplomat::result<T, E>`, which is backed by `std::variant`.

```cpp
auto result = MyClass::get_result();
// Get the ok value:
if (result.is_ok()) {
    auto ok = std::move(result).ok().value();
} else if (result.is_err()) {
    auto err = std::move(result).err().value();
}
```

### Slices

|    Diplomat Type                       |       C Type      |
|----------------------------------------|-------------------|
|           `&[Primitive]`               |`diplomat::span<const Primitive>`|
|           `&mut[Primitive]`            |`diplomat::span<Primitive>`|
|`&str` or `&DiplomatStr` or `DiplomatStrSlice` or `DiplomatUtf8StrSlice`|`std::string_view`|
|`&DiplomatStr16` or `DiplomatStr16Slice`|`std::u16string_view`|
|`&[&str] or &[DiplomatStrSlice]` or `&[DiplomatUtf8StrSlice]`|`diplomat::span<diplomat::string_view_for_slice>`|
|`&[DiplomatStr16Slice]`|`diplomat::span<diplomat::u16string_view_for_slice>`|

#### `diplomat::span`
All slices are stored in the `diplomat::span` struct. For C++17, this is the `std::span` struct. Otherwise, it is implemented in diplomat with the following rough structure:

```cpp
constexpr size_t dynamic_extent = std::numeric_limits<std::size_t>::max();
template <class T, std::size_t Extent = dynamic_extent>
class span {
    private:
    T* data;
    size_t size;
};
```

With helper method for constructing `diplomat::span` from arrays and pointers to sized data.

### Callbacks
Callbacks are represented as `std::function<Ret(Args...)>`, where `Ret` and `Args...` are diplomat-friendly C++ types. These work just like any other C++ callback function and are converted in to the C ABI with some templating.


{{supports("cpp")}}