#ifndef SOMELIB_DIPLOMAT_RUNTIME_CPP_H
#define SOMELIB_DIPLOMAT_RUNTIME_CPP_H

#include <optional>
#include <string>
#include <string_view>
#include <type_traits>
#include <variant>
#include <cstdint>
#include <functional>
#include <memory>
#include <limits>


#if __cplusplus >= 202002L
#include <span>
#else
#include <array>
#endif

#ifndef DIPLOMAT_LIFETIME_BOUND
#if defined(__has_cpp_attribute)
#if __has_cpp_attribute(msvc::lifetimebound)
#define DIPLOMAT_LIFETIME_BOUND [[msvc::lifetimebound]]
#elif __has_cpp_attribute(clang::lifetimebound)
#define DIPLOMAT_LIFETIME_BOUND [[clang::lifetimebound]]
#endif
#endif
#endif

#ifndef DIPLOMAT_LIFETIME_BOUND
#define DIPLOMAT_LIFETIME_BOUND
#endif

namespace somelib {
namespace diplomat {

namespace capi {
extern "C" {

static_assert(sizeof(char) == sizeof(uint8_t), "your architecture's `char` is not 8 bits");
static_assert(sizeof(char16_t) == sizeof(uint16_t), "your architecture's `char16_t` is not 16 bits");
static_assert(sizeof(char32_t) == sizeof(uint32_t), "your architecture's `char32_t` is not 32 bits");

typedef struct DiplomatWrite {
    void* context;
    char* buf;
    size_t len;
    size_t cap;
    bool grow_failed;
    void (*flush)(struct DiplomatWrite*);
    bool (*grow)(struct DiplomatWrite*, size_t);
} DiplomatWrite;

bool diplomat_is_str(const char* buf, size_t len);

#define MAKE_SLICES(name, c_ty) \
    typedef struct Diplomat##name##View { \
        const c_ty* data; \
        size_t len; \
    } Diplomat##name##View; \
    typedef struct Diplomat##name##ViewMut { \
        c_ty* data; \
        size_t len; \
    } Diplomat##name##ViewMut; \
    typedef struct Diplomat##name##Array { \
        const c_ty* data; \
        size_t len; \
    } Diplomat##name##Array;

#define MAKE_SLICES_AND_OPTIONS(name, c_ty) \
    MAKE_SLICES(name, c_ty) \
    typedef struct Option##name {union { c_ty ok; }; bool is_ok; } Option##name; \
    typedef struct Option##name##View {union { Diplomat##name##View ok; }; bool is_ok; } Option##name##View; \
    typedef struct Option##name##ViewMut {union { Diplomat##name##ViewMut ok; }; bool is_ok; } Option##name##ViewMut; \
    typedef struct Option##name##Array {union { Diplomat##name##Array ok; }; bool is_ok; } Option##name##Array; \

MAKE_SLICES_AND_OPTIONS(I8, int8_t)
MAKE_SLICES_AND_OPTIONS(U8, uint8_t)
MAKE_SLICES_AND_OPTIONS(I16, int16_t)
MAKE_SLICES_AND_OPTIONS(U16, uint16_t)
MAKE_SLICES_AND_OPTIONS(I32, int32_t)
MAKE_SLICES_AND_OPTIONS(U32, uint32_t)
MAKE_SLICES_AND_OPTIONS(I64, int64_t)
MAKE_SLICES_AND_OPTIONS(U64, uint64_t)
MAKE_SLICES_AND_OPTIONS(Isize, intptr_t)
MAKE_SLICES_AND_OPTIONS(Usize, size_t)
MAKE_SLICES_AND_OPTIONS(F32, float)
MAKE_SLICES_AND_OPTIONS(F64, double)
MAKE_SLICES_AND_OPTIONS(Bool, bool)
MAKE_SLICES_AND_OPTIONS(Char, char32_t)
MAKE_SLICES_AND_OPTIONS(String, char)
MAKE_SLICES_AND_OPTIONS(String16, char16_t)
MAKE_SLICES_AND_OPTIONS(Strings, DiplomatStringView)
MAKE_SLICES_AND_OPTIONS(Strings16, DiplomatString16View)

} // extern "C"
} // namespace capi

extern "C" inline void _flush(capi::DiplomatWrite* w) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(w->len);
}

extern "C" inline bool _grow_impl(capi::DiplomatWrite* w, uintptr_t requested) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(requested);
  w->cap = string->length();
  w->buf = &(*string)[0];
  return true;
}

extern "C" inline bool _grow(capi::DiplomatWrite* w, size_t requested) {
  return _grow_impl(w, static_cast<uintptr_t>(requested));
}

inline capi::DiplomatWrite WriteFromString(std::string& string) {
  capi::DiplomatWrite w;
  w.context = &string;
  w.buf = &string[0];
  w.len = string.length();
  w.cap = string.length();
  // Will never become true, as _grow is infallible.
  w.grow_failed = false;
  w.flush = _flush;
  w.grow = _grow;
  return w;
}

// This "trait" allows one to use _write() methods to efficiently
// write to a custom string type. To do this you need to write a specialized
// `WriteTrait<YourType>` (see WriteTrait<std::string> below)
// that is capable of constructing a DiplomatWrite, which can wrap
// your string type with appropriate resize/flush functionality.
template<typename T> struct WriteTrait {
  // Fill in this method on a specialization to implement this trait
  // static inline capi::DiplomatWrite Construct(T& t);
};

template<> struct WriteTrait<std::string> {
  static inline capi::DiplomatWrite Construct(std::string& t) {
    return diplomat::WriteFromString(t);
  }
};

template <typename T, typename CPtr> class Ref;
template <typename T, typename Enable = void> class Optional;

/// CRTP base class for generated opaque wrapper classes (`class Foo : public
/// OpaquePointer<Foo, capi::Foo, capi::Foo_destroy>`). Holds the single owned pointer and
/// implements the ctor/dtor/move/AsFFI/FromFFI boilerplate once instead of once per opaque type.
///
/// `T` must add no data members of its own: its entire layout must stay exactly one `CPtr*` (no
/// virtual functions either), since `Ref` below reinterpret_casts its own pointer-sized storage
/// as a `T`/`const T` to give borrowed access without ever constructing or destroying a `T`
/// through that cast.
template <typename T, typename CPtr, void (*Destructor)(CPtr*)>
class OpaquePointer {
public:
    inline const CPtr* AsFFI() const { return ptr_; }
    inline CPtr* AsFFI() { return ptr_; }
    inline static T FromFFI(CPtr* ptr) {
        T result{};
        result.ptr_ = ptr;
        return result;
    }

    inline ~OpaquePointer() {
        if (ptr_) {
            Destructor(ptr_);
        }
    }
    OpaquePointer(const OpaquePointer&) = delete;
    OpaquePointer& operator=(const OpaquePointer&) = delete;
    inline OpaquePointer(OpaquePointer&& old) noexcept : ptr_(old.ptr_) {
        old.ptr_ = nullptr;
    }
    inline OpaquePointer& operator=(OpaquePointer&& old) noexcept {
        if (this == &old) {
            return *this;
        }
        if (ptr_) {
            Destructor(ptr_);
        }
        ptr_ = old.ptr_;
        old.ptr_ = nullptr;
        return *this;
    }

    inline explicit operator bool() const { return ptr_ != nullptr; }

    /// Releases ownership of the underlying pointer without destroying it -- the caller becomes
    /// responsible for it (e.g. by round-tripping it through `FromFFI` again later). Used by the
    /// owning specialization of `Optional<T>` to take ownership from a live `T` without
    /// double-freeing when the original `T` goes out of scope.
    inline CPtr* release() {
        CPtr* p = ptr_;
        ptr_ = nullptr;
        return p;
    }

    /// A non-owning, trivially-copyable, immutable view of this object -- safe to store (e.g. in
    /// a struct field), unlike a raw `const T&`, since it doesn't dangle when reassigned.
    inline Ref<T, const CPtr> as_ref() const {
        return Ref<T, const CPtr>::FromFFI(AsFFI());
    }
    /// As `as_ref`, but mutable.
    inline Ref<T, CPtr> as_mut_ref() {
        return Ref<T, CPtr>::FromFFI(AsFFI());
    }
protected:
    OpaquePointer() = default;
private:
    CPtr* ptr_{nullptr};
    friend T;
};

/// A non-owning, trivially-copyable reference to a `T` (an `OpaquePointer`-derived opaque
/// wrapper). Used where a real `T&`/`const T&` can't be: struct fields, and anywhere else that
/// needs to store and reassign a borrow rather than just pass it through a call.
///
/// Mutability is encoded by `CPtr` itself rather than by a separate class: `Ref<Foo, const
/// capi::Foo>` (aliased as `FooRef`) is immutable, `Ref<Foo, capi::Foo>` (aliased as `FooRefMut`)
/// is mutable. Always go through those generated aliases rather than spelling this out directly
/// -- a bare `Ref<Foo, capi::Foo>` means *mutable* here, since it's `CPtr`'s own qualification
/// being read, not a hardcoded choice.
///
/// Safe for the same reason `OpaquePointer::as_ref`/`as_mut_ref` are: `T`'s entire layout is one
/// `CPtr*`, bit-identical to this class's own storage, so reinterpreting `&ptr_` as a
/// `ValueType*` never constructs or destroys a `T` -- it just reuses the layout to call methods
/// through the existing bits.
template <typename T, typename CPtr>
class Ref {
public:
    using CType = CPtr;
    using ValueType = std::conditional_t<std::is_const_v<CPtr>, const T, T>;

    inline CPtr* AsFFI() const { return ptr_; }
    inline static Ref FromFFI(CPtr* ptr) {
        Ref r;
        r.ptr_ = ptr;
        return r;
    }

    inline ValueType& get() { return *reinterpret_cast<ValueType*>(&ptr_); }
    inline ValueType& operator*() { return get(); }
    inline ValueType* operator->() { return &get(); }
    inline operator ValueType&() { return get(); }
private:
    Ref() = default;
    CPtr* ptr_{nullptr};
};

/// Detects "T behaves like an ABI pointer wrapper" (a generated opaque class `Foo`, or `Ref<T,
/// CPtr>`) -- i.e. anything whose `AsFFI()` returns a raw C pointer, as opposed to a
/// primitive/struct/enum whose `AsFFI()` (if it has one at all) returns a value type. This is
/// what lets `Optional<T>` niche-optimize to a single pointer (`nullptr` = empty) for such `T`,
/// the C++ analogue of Rust's `Option<&T>`/`Option<Box<T>>` niche optimizations, instead of
/// falling back to a `std::optional<T>`-shaped "value plus presence flag" representation.
template <typename T, typename = void>
struct is_pointer_like : std::false_type {};
template <typename T>
struct is_pointer_like<T, std::void_t<decltype(sizeof(T)),
                          std::enable_if_t<std::is_pointer_v<decltype(std::declval<T>().AsFFI())>>>>
  : std::true_type {};
template <typename T>
constexpr bool is_pointer_like_v = is_pointer_like<T>::value;

/// The `Ref`/opaque-aware equivalent of `std::optional<T>`, used everywhere `std::optional<T>`
/// would otherwise appear in generated signatures (return types, struct fields, callback
/// params/returns). Dispatches on `T`'s shape via the two partial specializations below:
///
/// - Pointer-like `T` (a generated opaque wrapper `Foo`, or `Ref<Foo, CPtr>`/`FooRef`/`FooRefMut`)
///   niche-optimizes to a single pointer, `nullptr` meaning empty -- the C++ analogue of Rust's
///   `Option<Box<T>>`/`Option<&T>`. This matters beyond just avoiding a wasted flag byte: a plain
///   `std::optional<Ref<T, CPtr>>` isn't guaranteed to be the same size as a raw `CPtr*` (the
///   discriminant flag typically doubles it), which would break the reinterpret_cast trick slices
///   (and struct fields, and callback signatures) rely on.
/// - Everything else (primitives, structs, enums, strings, ...) falls back to wrapping a real
///   `std::optional<T>`, since the C ABI needs an explicit presence flag for these regardless.
///
/// This primary template is that fallback; see the specializations below it for the pointer-like
/// cases.
template <typename T, typename Enable>
class Optional {
    std::optional<T> inner_;
public:
    using value_type = T;

    // All converting constructors are `explicit`, deliberately unlike `std::optional`: we want
    // constructing an `Optional<T>` (present or absent) to always be a visible, intentional act at
    // the call site, closer to Rust's `Some(x)`/`None` than to `std::optional`'s pervasive
    // implicit conversions.
    inline explicit Optional() = default;
    inline explicit Optional(std::nullopt_t) noexcept {}
    // Templated (rather than a plain `Optional(T value)`) for the same reason
    // `std::optional<T>`'s own converting constructor is templated: it lets a single
    // user-defined conversion (e.g. an enum-wrapper class's own converting constructor from its
    // raw enum literal type) still apply when initializing an `Optional<T>` -- a non-templated
    // `Optional(T)` parameter would need a *second*, disallowed user-defined conversion on top of
    // that to reach `T` first. Still explicit -- templated-ness and explicit-ness are orthogonal;
    // this only affects whether *this* constructor can fire without being spelled out, not what
    // conversions can happen to reach `T` once it's invoked.
    template <typename U = T, typename = std::enable_if_t<
        std::is_constructible_v<T, U&&> &&
        !std::is_same_v<std::decay_t<U>, Optional> &&
        !std::is_same_v<std::decay_t<U>, std::nullopt_t>>>
    inline explicit Optional(U&& value) : inner_(std::forward<U>(value)) {}

    inline explicit operator bool() const { return inner_.has_value(); }
    inline bool has_value() const { return inner_.has_value(); }

    inline void reset() { inner_.reset(); }
    inline void emplace(T value) { inner_.emplace(std::move(value)); }

    inline T& get() { return *inner_; }
    inline const T& get() const { return *inner_; }
    inline T& operator*() { return get(); }
    inline const T& operator*() const { return get(); }
    inline T* operator->() { return &get(); }
    inline const T* operator->() const { return &get(); }
    // Ref-qualified like `std::optional::value()`, so `Result::ok().value()`-style chains (`ok()`
    // returns an `Optional<T>` temporary) move out of the temporary instead of trying to copy
    // from it -- important since `T` here can be move-only.
    inline T& value() & { return get(); }
    inline const T& value() const & { return get(); }
    inline T&& value() && { return std::move(get()); }
};

// Lets `Optional<T> == nullptr`-style presence checks work for any specialization (e.g. the
// generated `__getitem__` bindings do this for `Optional<Ref<...>>`) -- `get()` isn't comparable
// to `nullptr` there, so this only ever checks presence, same as `std::optional == nullopt`.
template <typename T, typename Enable>
inline bool operator==(const Optional<T, Enable>& opt, std::nullptr_t) {
    return !opt.has_value();
}
template <typename T, typename Enable>
inline bool operator==(std::nullptr_t, const Optional<T, Enable>& opt) {
    return !opt.has_value();
}

// Lets `Optional<T> == someT`-style comparisons work, matching `std::optional`'s own comparison
// support -- used e.g. when comparing a struct field of `Optional<PrimitiveOrEnum>` against a
// raw value in test code.
template <typename T, typename Enable, typename U>
inline bool operator==(const Optional<T, Enable>& opt, const U& value) {
    return opt.has_value() && opt.get() == value;
}
template <typename T, typename Enable, typename U>
inline bool operator==(const U& value, const Optional<T, Enable>& opt) {
    return opt == value;
}

/// Non-owning, pointer-like, copyable `T` (`Ref<Foo, CPtr>`, i.e. `FooRef`/`FooRefMut`) -- stores
/// only the one pointer (reinterpret_cast, same trick as `Ref` itself), `nullptr` meaning empty.
/// No destructor/move-only machinery needed since `T` itself never owns anything.
///
/// `value_type` is `T::ValueType` (i.e. the wrapped opaque type, `Foo`/`const Foo`), not `T`
/// itself, so that language bindings which already know how to convert `Foo`/`std::optional<Foo>`
/// (e.g. nanobind's `optional_caster`, generic over anything shaped like `std::optional`) can
/// reuse that existing support instead of needing separate knowledge of `Ref`/`Optional<Ref<>>`
/// -- see the nanobind `type_caster` for this case in `common.h.jinja`.
template <typename T>
class Optional<T, std::enable_if_t<is_pointer_like_v<T> && std::is_copy_constructible_v<T>>> {
    using CPtr = typename T::CType;
    using ValueType = typename T::ValueType;
public:
    using value_type = ValueType;

    // All converting constructors are `explicit` -- see the primary template's constructors for
    // why. For this specialization specifically: a bare `FooRef`/`FooRefMut` shouldn't be able to
    // silently become "always present" wherever an `Optional<FooRef>` is expected -- callers
    // should say `Optional(ref)`/`Optional<FooRef>(ref)` and mean it.
    inline explicit Optional() = default;
    inline explicit Optional(std::nullopt_t) noexcept {}
    inline explicit Optional(T ref) noexcept : ptr_(ref.AsFFI()) {}

    inline explicit operator bool() const { return ptr_ != nullptr; }
    inline bool has_value() const { return ptr_ != nullptr; }

    inline void reset() { ptr_ = nullptr; }
    inline void emplace(T ref) { ptr_ = ref.AsFFI(); }
    // Overload used by nanobind's optional_caster, which emplaces directly from a `ValueType&`
    // (the already-registered `Foo` class), not a `Ref` -- see `value_type` above.
    inline void emplace(ValueType& value) { ptr_ = value.AsFFI(); }

    inline ValueType& get() { return *reinterpret_cast<ValueType*>(&ptr_); }
    inline ValueType& get() const { return *reinterpret_cast<ValueType*>(const_cast<CPtr**>(&ptr_)); }
    inline ValueType& operator*() { return get(); }
    inline ValueType& operator*() const { return get(); }
    inline ValueType* operator->() { return &get(); }
    inline ValueType* operator->() const { return &get(); }
    // Ref-qualified like `std::optional::value()` -- see the primary template's `value()` for why.
    inline ValueType& value() & { return get(); }
    inline ValueType&& value() && { return std::move(get()); }

    inline CPtr* AsFFI() const { return ptr_; }
    inline static Optional FromFFI(CPtr* ptr) {
        Optional o;
        o.ptr_ = ptr;
        return o;
    }
private:
    CPtr* ptr_{nullptr};
};

/// Owning, pointer-like, move-only `T` (a generated opaque wrapper class `Foo`) -- the ABI
/// representation (a single nullable `CPtr*`) already matches Rust's `Option<Box<T>>` niche
/// optimization, so this stores just that one pointer and constructs/destroys the wrapped `Foo`
/// on demand (via `T::FromFFI`/`T`'s own destructor), exactly as `OpaquePointer` itself does,
/// rather than adding a separate `std::optional`-style presence flag.
template <typename T>
class Optional<T, std::enable_if_t<is_pointer_like_v<T> && !std::is_copy_constructible_v<T>>> {
    using CPtr = std::remove_pointer_t<decltype(std::declval<T>().AsFFI())>;
public:
    using value_type = T;

    // All converting constructors are `explicit` -- see the primary template's constructors for
    // why.
    inline explicit Optional() = default;
    inline explicit Optional(std::nullopt_t) noexcept {}
    inline explicit Optional(T value) noexcept : ptr_(value.release()) {}

    inline ~Optional() { reset(); }
    Optional(const Optional&) = delete;
    Optional& operator=(const Optional&) = delete;
    inline Optional(Optional&& old) noexcept : ptr_(old.ptr_) { old.ptr_ = nullptr; }
    inline Optional& operator=(Optional&& old) noexcept {
        if (this == &old) {
            return *this;
        }
        reset();
        ptr_ = old.ptr_;
        old.ptr_ = nullptr;
        return *this;
    }

    inline explicit operator bool() const { return ptr_ != nullptr; }
    inline bool has_value() const { return ptr_ != nullptr; }

    inline void reset() {
        if (ptr_) {
            CPtr* p = ptr_;
            ptr_ = nullptr;
            T tmp = T::FromFFI(p);
        }
    }
    inline void emplace(T value) {
        reset();
        ptr_ = value.release();
    }

    inline T& get() { return *reinterpret_cast<T*>(&ptr_); }
    inline const T& get() const { return *reinterpret_cast<const T*>(&ptr_); }
    inline T& operator*() { return get(); }
    inline const T& operator*() const { return get(); }
    inline T* operator->() { return &get(); }
    inline const T* operator->() const { return &get(); }
    // Ref-qualified like `std::optional::value()` -- important here specifically, since `T` is
    // move-only: `Result::ok().value()`-style chains need to move the wrapped `T` out of the
    // `Optional<T>` temporary `ok()` returns, not try to copy it.
    inline T& value() & { return get(); }
    inline T&& value() && { return std::move(get()); }

    inline CPtr* AsFFI() const { return ptr_; }
    inline static Optional FromFFI(CPtr* ptr) {
        Optional o;
        o.ptr_ = ptr;
        return o;
    }
    // Mirrors `OpaquePointer::release()`: relinquishes ownership without destroying, so callers
    // rebuilding a fresh owning value from this one's pointer (e.g. the nanobind ZST fix-up) don't
    // also trigger this `Optional`'s own destructor on the same pointer.
    inline CPtr* release() {
        CPtr* p = ptr_;
        ptr_ = nullptr;
        return p;
    }
private:
    CPtr* ptr_{nullptr};
};

// Deduction guide so `Optional(val)` (used in generated code the same way `std::optional(val)`
// was before) resolves to the right `Optional<T>` regardless of which specialization backs it.
template<typename T>
Optional(T) -> Optional<T>;

// Trait for extracting inner types from T*, std::optional, std::unique_ptr, Ref, or Optional.
// These are the potential types returned by next() functions.
template<typename T> struct inner { /* only T*, std::optional, std::unique_ptr, Ref, and Optional are supported */ };
template<typename T> struct inner<T*> { using type = T; };
template<typename T> struct inner<std::unique_ptr<T>> { using type = T; };
template<typename T> struct inner<std::optional<T>>{ using type = T; };
template<typename T, typename CPtr> struct inner<Ref<T, CPtr>> { using type = T; };
template<typename T> struct inner<Optional<T>> { using type = T; };
template<typename T, typename CPtr> struct inner<Optional<Ref<T, CPtr>>> { using type = T; };

template<class T> struct Ok {
  T inner;

  // Move constructor always allowed
  Ok(T&& i): inner(std::forward<T>(i)) {}

  //  copy constructor allowed only for trivially copyable types
  template<typename X = T, typename = typename std::enable_if<std::is_trivially_copyable<X>::value>::type>
  Ok(const T& i) : inner(i) {}

  Ok() = default;
  Ok(Ok&&) noexcept = default;
  Ok(const Ok &) = default;
  Ok& operator=(const Ok&) = default;
  Ok& operator=(Ok&&) noexcept = default;
};


template<class T> struct Err {
  T inner;

  // Move constructor always allowed
  Err(T&& i): inner(std::forward<T>(i)) {}

  //  copy constructor allowed only for trivially copyable types
  template<typename X = T, typename = typename std::enable_if<std::is_trivially_copyable<X>::value>::type>
  Err(const T& i) : inner(i) {}

  Err() = default;
  Err(Err&&) noexcept = default;
  Err(const Err &) = default;
  Err& operator=(const Err&) = default;
  Err& operator=(Err&&) noexcept = default;
};

template <typename T> struct fn_traits;

template<class T, class E>
class result {
protected:
    std::variant<Ok<T>, Err<E>> val;
public:
  template <typename T_>
  friend struct fn_traits;

  result(Ok<T>&& v): val(std::move(v)) {}
  result(Err<E>&& v): val(std::move(v)) {}
  result() = default;
  result(const result &) = default;
  result& operator=(const result&) = default;
  result& operator=(result&&) noexcept = default;
  result(result &&) noexcept = default;
  ~result() = default;
  bool is_ok() const {
    return std::holds_alternative<Ok<T>>(this->val);
  }
  bool is_err() const {
    return std::holds_alternative<Err<E>>(this->val);
  }

  template<typename U = T, typename std::enable_if_t<!std::is_reference_v<U>, std::nullptr_t> = nullptr>
  Optional<T> ok() && {
    if (!this->is_ok()) {
      return Optional<T>(std::nullopt);
    }
    return Optional<T>(std::move(std::get<Ok<T>>(std::move(this->val)).inner));
  }

  template<typename U = E, typename std::enable_if_t<!std::is_reference_v<U>, std::nullptr_t> = nullptr>
  Optional<E> err() && {
    if (!this->is_err()) {
      return Optional<E>(std::nullopt);
    }
    return Optional<E>(std::move(std::get<Err<E>>(std::move(this->val)).inner));
  }

  // std::optional does not work with reference types directly, so wrap them if present
  template<typename U = T, typename std::enable_if_t<std::is_reference_v<U>, std::nullptr_t> = nullptr>
  std::optional<std::reference_wrapper<std::remove_reference_t<T>>> ok() && {
    if (!this->is_ok()) {
      return std::nullopt;
    }
    return std::make_optional(std::reference_wrapper(std::forward<T>(std::get<Ok<T>>(std::move(this->val)).inner)));
  }

  template<typename U = E, typename std::enable_if_t<std::is_reference_v<U>, std::nullptr_t> = nullptr>
  std::optional<std::reference_wrapper<std::remove_reference_t<E>>> err() && {
    if (!this->is_err()) {
      return std::nullopt;
    }
    return std::make_optional(std::reference_wrapper(std::forward<E>(std::get<Err<E>>(std::move(this->val)).inner)));
  }

  void set_ok(T&& t) {
    this->val = Ok<T>(std::move(t));
  }

  void set_err(E&& e) {
    this->val = Err<E>(std::move(e));
  }

  template<typename T2>
  result<T2, E> replace_ok(T2&& t) {
    if (this->is_err()) {
      return result<T2, E>(Err<E>(std::get<Err<E>>(std::move(this->val))));
    } else {
      return result<T2, E>(Ok<T2>(std::move(t)));
    }
  }
};

class Utf8Error {};

// Use custom std::span on C++17, otherwise use std::span
#if __cplusplus >= 202002L

constexpr std::size_t dynamic_extent = std::dynamic_extent;
template<class T, std::size_t E = dynamic_extent> using span = std::span<T, E>;

#else // __cplusplus < 202002L

// C++-17-compatible-ish std::span
constexpr size_t dynamic_extent = std::numeric_limits<std::size_t>::max();
template <class T, std::size_t Extent = dynamic_extent>
class span {
public:
  constexpr span(T *data = nullptr, size_t size = Extent)
    : data_(data), size_(size) {}

  constexpr span(const span<T> &o)
    : data_(o.data_), size_(o.size_) {}
  template <size_t N>
  constexpr span(std::array<typename std::remove_const_t<T>, N> &arr)
    : data_(const_cast<T *>(arr.data())), size_(N) {}

  constexpr T* data() const noexcept {
    return this->data_;
  }
  constexpr size_t size() const noexcept {
    return this->size_;
  }

  constexpr T *begin() const noexcept { return data(); }
  constexpr T *end() const noexcept { return data() + size(); }

  void operator=(span<T> o) {
    data_ = o.data_;
    size_ = o.size_;
  }

private:
  T* data_;
  size_t size_;
};

#endif // __cplusplus >= 202002L

// An ABI stable std::basic_string_view equivalent for the case of string
// views in slices
template <class CharT, class Traits = std::char_traits<CharT>>
class basic_string_view_for_slice {
public:
  using std_string_view           = std::basic_string_view<CharT, Traits>;
  using traits_type               = typename std_string_view::traits_type;
  using value_type                = typename std_string_view::value_type;
  using pointer                   = typename std_string_view::pointer;
  using const_pointer             = typename std_string_view::const_pointer;
  using size_type                 = typename std_string_view::size_type;
  using difference_type           = typename std_string_view::difference_type;

  constexpr basic_string_view_for_slice() noexcept
    : basic_string_view_for_slice{std_string_view{}} {}

  constexpr basic_string_view_for_slice(const basic_string_view_for_slice& other) noexcept = default;

  constexpr basic_string_view_for_slice(const const_pointer s, const size_type count)
    : basic_string_view_for_slice{std_string_view{s, count}} {}

  constexpr basic_string_view_for_slice(const const_pointer s)
    : basic_string_view_for_slice{std_string_view{s}} {}

  constexpr basic_string_view_for_slice& operator=(const basic_string_view_for_slice& view) noexcept = default;

  constexpr basic_string_view_for_slice(const std_string_view& s) noexcept
    : data_{s.data(), s.size()} {}

  constexpr basic_string_view_for_slice& operator=(const std_string_view& s) noexcept {
    data_ = {s.data(), s.size()};
    return *this;
  }

  constexpr operator std_string_view() const noexcept { return {data(), size()}; }
  constexpr std_string_view as_sv() const noexcept { return *this; }

  constexpr const_pointer data() const noexcept { return data_.data; }
  constexpr size_type size() const noexcept { return data_.len; }

private:
  using capi_type =
    std::conditional_t<std::is_same_v<value_type, char>,
      capi::DiplomatStringView,
    std::conditional_t<std::is_same_v<value_type, char16_t>,
      capi::DiplomatString16View,
      void>>;

  static_assert(!std::is_void_v<capi_type>,
    "ABI compatible string_views are only supported for char and char16_t");

  capi_type data_;
};

// We only implement these specialisations as diplomat doesn't provide c abi
// types for others
using string_view_for_slice = basic_string_view_for_slice<char>;
using u16string_view_for_slice = basic_string_view_for_slice<char16_t>;

using string_view_span = span<const string_view_for_slice>;
using u16string_view_span = span<const u16string_view_for_slice>;

// Interop between std::function & our C Callback wrapper type

template <typename T, typename = void>
struct as_ffi {
  using type = T;
};

// The `sizeof` check here is a completeness guard: a raw `capi::Foo*` (used for a borrowed
// opaque callback arg/return) has an intentionally-incomplete pointee, and without this guard
// `decltype(declval<capi::Foo>().AsFFI())` would be a hard compile error rather than a SFINAE
// failure, since substitution only cleanly fails on ill-formed *dependent* expressions.
template <typename T>
struct as_ffi<T, std::void_t<decltype(sizeof(std::remove_pointer_t<T>)),
                              decltype(std::declval<std::remove_pointer_t<T>>().AsFFI())>> {
  using type = decltype(std::declval<std::remove_pointer_t<T>>().AsFFI());
};

template <typename T>
struct as_ffi<std::unique_ptr<T>> {
  using type = decltype(std::declval<T>().AsFFI());
};

// `Optional<Foo>`/`Optional<Ref<Foo, CPtr>>` (the pointer-like specializations) already have
// their own `AsFFI()` returning a raw pointer, so the generic detector above picks them up
// automatically -- no separate specialization needed here (unlike `std::optional<T>`, which never
// has `AsFFI()` at all).

template<typename T>
using as_ffi_t = typename as_ffi<T>::type;

template<typename T>
using replace_string_view_t = std::conditional_t<std::is_same_v<T, std::string_view>, capi::DiplomatStringView, T>;

template<typename T, typename = void>
struct diplomat_c_span_convert {
  using type = T;
};

#define MAKE_SLICE_CONVERTERS(name, c_ty) \
  template<typename T> \
  struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const c_ty>>>> { \
    using type = diplomat::capi::Diplomat##name##View; \
  }; \
  template<typename T> \
  struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<c_ty>>>> { \
    using type = diplomat::capi::Diplomat##name##ViewMut; \
  }; \

#if !defined(__sun) || !defined(_CHAR_IS_SIGNED)
// int8_t and char are the same type on Solaris. Guard this definition to avoid
// conflicts.
MAKE_SLICE_CONVERTERS(I8, int8_t)
#endif
MAKE_SLICE_CONVERTERS(U8, uint8_t)
MAKE_SLICE_CONVERTERS(I16, int16_t)
MAKE_SLICE_CONVERTERS(U16, uint16_t)
MAKE_SLICE_CONVERTERS(I32, int32_t)
MAKE_SLICE_CONVERTERS(U32, uint32_t)
MAKE_SLICE_CONVERTERS(I64, int64_t)
MAKE_SLICE_CONVERTERS(U64, uint64_t)
MAKE_SLICE_CONVERTERS(F32, float)
MAKE_SLICE_CONVERTERS(F64, double)
MAKE_SLICE_CONVERTERS(Bool, bool)
MAKE_SLICE_CONVERTERS(Char, char32_t)
MAKE_SLICE_CONVERTERS(String, char)
MAKE_SLICE_CONVERTERS(String16, char16_t)

template<typename T>
using diplomat_c_span_convert_t = typename diplomat_c_span_convert<T>::type;

template<typename T>
struct is_unique_ptr {
  static constexpr bool value = false;
  using type = T;
};

template<typename T>
struct is_unique_ptr<std::unique_ptr<T>> {
  static constexpr bool value = true;
  using type = T;
};

template<typename T>
constexpr bool is_unique_ptr_v = is_unique_ptr<T>::value;

template<typename T>
struct is_optional {
  static constexpr bool value = false;
};

template<typename T>
struct is_optional<std::optional<T>> {
  static constexpr bool value = true;
};

template<typename T>
struct is_optional<Optional<T>> {
  static constexpr bool value = true;
};

template<typename T>
constexpr bool is_optional_v = is_optional<T>::value;

/// Replace the argument types from the std::function with the argument types for th function pointer
template<typename T>
using replace_fn_t = diplomat_c_span_convert_t<replace_string_view_t<as_ffi_t<T>>>;

template <typename Ret, typename... Args> struct fn_traits<std::function<Ret(Args...)>> {
    using fn_ptr_t = Ret(Args...);
    using function_t = std::function<fn_ptr_t>;
    using ret = Ret;

    // For a given T, creates a function that take in the C ABI version & return the C++ type.
    template<typename T>
    static T replace(replace_fn_t<T> val) {
      if constexpr(std::is_same_v<T, std::string_view>)   {
          return std::string_view{val.data, val.len};
      } else if constexpr (!std::is_same_v<T, diplomat_c_span_convert_t<T>>) {
        return T{ val.data, val.len };
      } else if constexpr (!std::is_same_v<T, as_ffi_t<T>>) {
        if constexpr (std::is_lvalue_reference_v<T>) {
          return *std::remove_reference_t<T>::FromFFI(val);
        } else if constexpr (is_unique_ptr_v<T>) {
          return T(is_unique_ptr<T>::type::FromFFI(val));
        }
        else {
          return T::FromFFI(val);
        }
      }
      else {
          return val;
      }
    }

    template<typename T>
    static replace_fn_t<T> replace_ret(T val) {
      if constexpr(std::is_same_v<T, std::string_view>)   {
          return {val.data(), val.size()};
      } else if constexpr (!std::is_same_v<T, diplomat_c_span_convert_t<T>>) {
        // Can we convert straight away to our slice type, or (in the case of ABI compatible structs), do we have to do a reinterpret cast?
        if constexpr(std::is_same_v<decltype(std::declval<T>().data()), decltype(replace_fn_t<T>::data)>) {
          return replace_fn_t<T> { val.data(), val.size() };
        } else {
          return replace_fn_t<T> { reinterpret_cast<decltype(replace_fn_t<T>::data)>(val.data()), val.size() };
        }
      } else if constexpr(!std::is_same_v<T, as_ffi_t<T>>) {
        return val.AsFFI();
      } else {
        return val;
      }
    }

    static Ret c_run_callback(const void *cb, replace_fn_t<Args>... args) {
        return (*reinterpret_cast<const function_t *>(cb))(replace<Args>(args)...);
    }

    template<typename TOut, typename T>
    static TOut replace_optional_ret(Optional<T> optional) {
      constexpr bool has_ok = !std::is_same_v<T, std::monostate>;

      bool is_ok = optional.has_value();

      TOut out;
      out.is_ok = is_ok;

      if constexpr(has_ok) {
        if (is_ok) {
          out.ok = replace_ret<T>(optional.value());
        }
      }
      return out;
    }

    template<typename T, typename E, typename TOut>
    static TOut c_run_callback_result(const void *cb, replace_fn_t<Args>... args) {
      result<T, E> res = c_run_callback(cb, args...);

      auto is_ok = res.is_ok();

      constexpr bool has_ok = !std::is_same_v<T, std::monostate>;
      constexpr bool has_err = !std::is_same_v<E, std::monostate>;

      TOut out;
      out.is_ok = is_ok;

      if constexpr (has_ok) {
        if (is_ok) {
          if constexpr (is_optional_v<T>) {
            out.ok = replace_optional_ret<decltype(out.ok)>(std::move(std::get<Ok<T>>(res.val).inner));
          } else {
            out.ok = replace_ret<T>(std::get<Ok<T>>(res.val).inner);
          }
        }
      }

      if constexpr(has_err) {
        if (!is_ok) {
          if constexpr(is_optional_v<T>) {
            out.err = replace_optional_ret<decltype(out.err)>(std::move(std::get<Err<E>>(res.val).inner));
          } else {
            out.err = replace_ret<E>(std::get<Err<E>>(res.val).inner);
          }
        }
      }

      return out;
    }

    // For DiplomatOption<>
    template<typename T, typename TOut>
    static TOut c_run_callback_diplomat_option(const void *cb, replace_fn_t<Args>... args) {
      Optional<T> ret = c_run_callback(cb, args...);

      return replace_optional_ret<TOut>(std::move(ret));
    }

    // `Ret` (the callback's actual C++-facing return type) can be an owned `Foo` (by value), a
    // borrowed `Ref<Foo,CPtr>` (`FooRef`/`FooRefMut`), or already the raw ABI pointer --
    // `replace_ret` already knows how to convert all three to `T` (the raw ABI pointer
    // `replace_fn_t<Ret>`).
    template<typename T>
    static T c_run_callback_diplomat_opaque(const void* cb, replace_fn_t<Args>... args) {
      Ret out = c_run_callback(cb, args...);

      return replace_ret<Ret>(out);
    }

    static void c_delete(const void *cb) {
        delete reinterpret_cast<const function_t *>(cb);
    }

    fn_traits(function_t) {} // Allows less clunky construction (avoids decltype)
};

// additional deduction guide required
template<class T>
fn_traits(T) -> fn_traits<T>;

template<typename T, typename U = typename inner<T>::type>
inline const U get_inner_if_present(T v) {
  if constexpr(std::is_same_v<T,U>) {
    return std::move(v);
  } else {
    return *std::move(v);
  }
}

// Adapter for iterator types
template<typename T, typename U = void> struct has_next : std::false_type {};
template<typename T> struct has_next < T, std::void_t<decltype(std::declval<T>().next())>> : std::true_type {};
template<typename T> constexpr bool has_next_v = has_next<T>::value;

/// Helper template enabling native iteration over owned objects which implement next()
template<typename T>
struct next_to_iter_helper {
  static_assert(has_next_v<T>, "next_to_iter_helper may only be used with types implementing next()");
  using next_type = decltype(std::declval<T>().next());

  // STL Iterator trait definitions
  using value_type = typename inner<next_type>::type;
  using difference_type = void;
  using reference = std::add_lvalue_reference_t<value_type>;
  using iterator_category = std::input_iterator_tag;

  next_to_iter_helper(T&& ptr) : _ptr(std::make_shared<T>(std::move(ptr))), _curr(_ptr->next()) {}

  // https://en.cppreference.com/w/cpp/named_req/InputIterator requires that the type be copyable
  next_to_iter_helper(const next_to_iter_helper& o) : _ptr(o._ptr), _curr(o._curr) {}

  void operator++() { _curr = _ptr->next(); }
  void operator++(int) { ++(*this); }
  // Not `const`: `Ref::operator*()` (used when `next()` returns a borrowed opaque reference)
  // isn't `const` either, for the same reason -- see its definition above.
  const value_type& operator*() { return *_curr; }

  bool operator!=(std::nullopt_t) {
    return (bool)_curr;
  }

  std::shared_ptr<T> _ptr; // shared to satisfy the copyable requirement
  next_type _curr;
};

} // namespace diplomat
} // namespace somelib
#endif