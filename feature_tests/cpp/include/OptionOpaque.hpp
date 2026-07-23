#ifndef SOMELIB_OptionOpaque_HPP
#define SOMELIB_OptionOpaque_HPP

#include "OptionOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BorrowingOptionStruct.hpp"
#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::OptionOpaque* OptionOpaque_new(int32_t i);

    somelib::capi::OptionOpaque* OptionOpaque_new_none(void);

    typedef struct OptionOpaque_returns_result {union {somelib::capi::OptionStruct ok; }; bool is_ok;} OptionOpaque_returns_result;
    OptionOpaque_returns_result OptionOpaque_returns(void);

    typedef struct OptionOpaque_option_isize_result {union {intptr_t ok; }; bool is_ok;} OptionOpaque_option_isize_result;
    OptionOpaque_option_isize_result OptionOpaque_option_isize(const somelib::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_usize_result {union {size_t ok; }; bool is_ok;} OptionOpaque_option_usize_result;
    OptionOpaque_option_usize_result OptionOpaque_option_usize(const somelib::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_i32_result {union {int32_t ok; }; bool is_ok;} OptionOpaque_option_i32_result;
    OptionOpaque_option_i32_result OptionOpaque_option_i32(const somelib::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_u32_result {union {uint32_t ok; }; bool is_ok;} OptionOpaque_option_u32_result;
    OptionOpaque_option_u32_result OptionOpaque_option_u32(const somelib::capi::OptionOpaque* self);

    somelib::capi::OptionStruct OptionOpaque_new_struct(void);

    somelib::capi::OptionStruct OptionOpaque_new_struct_nones(void);

    const somelib::capi::OptionOpaque* OptionOpaque_returns_none_self(const somelib::capi::OptionOpaque* self);

    const somelib::capi::OptionOpaque* OptionOpaque_returns_some_self(const somelib::capi::OptionOpaque* self);

    void OptionOpaque_assert_integer(const somelib::capi::OptionOpaque* self, int32_t i);

    bool OptionOpaque_option_opaque_argument(const somelib::capi::OptionOpaque* arg);

    typedef struct OptionOpaque_accepts_option_u8_result {union {uint8_t ok; }; bool is_ok;} OptionOpaque_accepts_option_u8_result;
    OptionOpaque_accepts_option_u8_result OptionOpaque_accepts_option_u8(somelib::diplomat::capi::OptionU8 arg, uint8_t sentinel);

    typedef struct OptionOpaque_accepts_option_enum_result {union {somelib::capi::OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_option_enum_result;
    OptionOpaque_accepts_option_enum_result OptionOpaque_accepts_option_enum(somelib::capi::OptionEnum_option arg, uint8_t sentinel);

    void OptionOpaque_accepts_borrowing_option_struct(somelib::capi::BorrowingOptionStruct arg);

    typedef struct OptionOpaque_accepts_multiple_option_enum_result {union {somelib::capi::OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_multiple_option_enum_result;
    OptionOpaque_accepts_multiple_option_enum_result OptionOpaque_accepts_multiple_option_enum(uint8_t sentinel1, somelib::capi::OptionEnum_option arg1, somelib::capi::OptionEnum_option arg2, somelib::capi::OptionEnum_option arg3, uint8_t sentinel2);

    typedef struct OptionOpaque_accepts_option_input_struct_result {union {somelib::capi::OptionInputStruct ok; }; bool is_ok;} OptionOpaque_accepts_option_input_struct_result;
    OptionOpaque_accepts_option_input_struct_result OptionOpaque_accepts_option_input_struct(somelib::capi::OptionInputStruct_option arg, uint8_t sentinel);

    somelib::capi::OptionInputStruct OptionOpaque_returns_option_input_struct(void);

    size_t OptionOpaque_accepts_option_str(somelib::diplomat::capi::OptionStringView arg, uint8_t sentinel);

    bool OptionOpaque_accepts_option_str_slice(somelib::diplomat::capi::OptionStringsView arg, uint8_t sentinel);

    int64_t OptionOpaque_accepts_option_primitive(somelib::diplomat::capi::OptionU32View arg, uint8_t sentinel);

    void OptionOpaque_destroy(OptionOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::Optional<somelib::OptionOpaque> somelib::OptionOpaque::new_(int32_t i) {
    auto result = somelib::capi::OptionOpaque_new(i);
    return somelib::diplomat::Optional<somelib::OptionOpaque>::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::OptionOpaque> somelib::OptionOpaque::new_none() {
    auto result = somelib::capi::OptionOpaque_new_none();
    return somelib::diplomat::Optional<somelib::OptionOpaque>::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::OptionStruct> somelib::OptionOpaque::returns() {
    auto result = somelib::capi::OptionOpaque_returns();
    return result.is_ok ? somelib::diplomat::Optional<somelib::OptionStruct>(somelib::OptionStruct::FromFFI(result.ok)) : somelib::diplomat::Optional<somelib::OptionStruct>(std::nullopt);
}

inline somelib::diplomat::Optional<intptr_t> somelib::OptionOpaque::option_isize() const {
    auto result = somelib::capi::OptionOpaque_option_isize(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<intptr_t>(result.ok) : somelib::diplomat::Optional<intptr_t>(std::nullopt);
}

inline somelib::diplomat::Optional<size_t> somelib::OptionOpaque::option_usize() const {
    auto result = somelib::capi::OptionOpaque_option_usize(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<size_t>(result.ok) : somelib::diplomat::Optional<size_t>(std::nullopt);
}

inline somelib::diplomat::Optional<int32_t> somelib::OptionOpaque::option_i32() const {
    auto result = somelib::capi::OptionOpaque_option_i32(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<int32_t>(result.ok) : somelib::diplomat::Optional<int32_t>(std::nullopt);
}

inline somelib::diplomat::Optional<uint32_t> somelib::OptionOpaque::option_u32() const {
    auto result = somelib::capi::OptionOpaque_option_u32(this->AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<uint32_t>(result.ok) : somelib::diplomat::Optional<uint32_t>(std::nullopt);
}

inline somelib::OptionStruct somelib::OptionOpaque::new_struct() {
    auto result = somelib::capi::OptionOpaque_new_struct();
    return somelib::OptionStruct::FromFFI(result);
}

inline somelib::OptionStruct somelib::OptionOpaque::new_struct_nones() {
    auto result = somelib::capi::OptionOpaque_new_struct_nones();
    return somelib::OptionStruct::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::OptionOpaqueRef> somelib::OptionOpaque::returns_none_self() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OptionOpaque_returns_none_self(this->AsFFI());
    return somelib::diplomat::Optional<somelib::OptionOpaqueRef>::FromFFI(result);
}

inline somelib::diplomat::Optional<somelib::OptionOpaqueRef> somelib::OptionOpaque::returns_some_self() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OptionOpaque_returns_some_self(this->AsFFI());
    return somelib::diplomat::Optional<somelib::OptionOpaqueRef>::FromFFI(result);
}

inline void somelib::OptionOpaque::assert_integer(int32_t i) const {
    somelib::capi::OptionOpaque_assert_integer(this->AsFFI(),
        i);
}

inline bool somelib::OptionOpaque::option_opaque_argument(somelib::diplomat::Optional<somelib::OptionOpaqueRef> arg) {
    auto result = somelib::capi::OptionOpaque_option_opaque_argument(arg.AsFFI());
    return result;
}

inline somelib::diplomat::Optional<uint8_t> somelib::OptionOpaque::accepts_option_u8(somelib::diplomat::Optional<uint8_t> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_u8(arg.has_value() ? (somelib::diplomat::capi::OptionU8{ { arg.value() }, true }) : (somelib::diplomat::capi::OptionU8{ {}, false }),
        sentinel);
    return result.is_ok ? somelib::diplomat::Optional<uint8_t>(result.ok) : somelib::diplomat::Optional<uint8_t>(std::nullopt);
}

inline somelib::diplomat::Optional<somelib::OptionEnum> somelib::OptionOpaque::accepts_option_enum(somelib::diplomat::Optional<somelib::OptionEnum> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_enum(arg.has_value() ? (somelib::capi::OptionEnum_option{ { arg.value().AsFFI() }, true }) : (somelib::capi::OptionEnum_option{ {}, false }),
        sentinel);
    return result.is_ok ? somelib::diplomat::Optional<somelib::OptionEnum>(somelib::OptionEnum::FromFFI(result.ok)) : somelib::diplomat::Optional<somelib::OptionEnum>(std::nullopt);
}

inline void somelib::OptionOpaque::accepts_borrowing_option_struct(somelib::BorrowingOptionStruct arg) {
    somelib::capi::OptionOpaque_accepts_borrowing_option_struct(arg.AsFFI());
}

inline somelib::diplomat::Optional<somelib::OptionEnum> somelib::OptionOpaque::accepts_multiple_option_enum(uint8_t sentinel1, somelib::diplomat::Optional<somelib::OptionEnum> arg1, somelib::diplomat::Optional<somelib::OptionEnum> arg2, somelib::diplomat::Optional<somelib::OptionEnum> arg3, uint8_t sentinel2) {
    auto result = somelib::capi::OptionOpaque_accepts_multiple_option_enum(sentinel1,
        arg1.has_value() ? (somelib::capi::OptionEnum_option{ { arg1.value().AsFFI() }, true }) : (somelib::capi::OptionEnum_option{ {}, false }),
        arg2.has_value() ? (somelib::capi::OptionEnum_option{ { arg2.value().AsFFI() }, true }) : (somelib::capi::OptionEnum_option{ {}, false }),
        arg3.has_value() ? (somelib::capi::OptionEnum_option{ { arg3.value().AsFFI() }, true }) : (somelib::capi::OptionEnum_option{ {}, false }),
        sentinel2);
    return result.is_ok ? somelib::diplomat::Optional<somelib::OptionEnum>(somelib::OptionEnum::FromFFI(result.ok)) : somelib::diplomat::Optional<somelib::OptionEnum>(std::nullopt);
}

inline somelib::diplomat::Optional<somelib::OptionInputStruct> somelib::OptionOpaque::accepts_option_input_struct(somelib::diplomat::Optional<somelib::OptionInputStruct> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_input_struct(arg.has_value() ? (somelib::capi::OptionInputStruct_option{ { arg.value().AsFFI() }, true }) : (somelib::capi::OptionInputStruct_option{ {}, false }),
        sentinel);
    return result.is_ok ? somelib::diplomat::Optional<somelib::OptionInputStruct>(somelib::OptionInputStruct::FromFFI(result.ok)) : somelib::diplomat::Optional<somelib::OptionInputStruct>(std::nullopt);
}

inline somelib::OptionInputStruct somelib::OptionOpaque::returns_option_input_struct() {
    auto result = somelib::capi::OptionOpaque_returns_option_input_struct();
    return somelib::OptionInputStruct::FromFFI(result);
}

inline size_t somelib::OptionOpaque::accepts_option_str(somelib::diplomat::Optional<std::string_view> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_str(arg.has_value() ? (somelib::diplomat::capi::OptionStringView{ { {arg.value().data(), arg.value().size()} }, true }) : (somelib::diplomat::capi::OptionStringView{ {}, false }),
        sentinel);
    return result;
}

inline bool somelib::OptionOpaque::accepts_option_str_slice(somelib::diplomat::Optional<somelib::diplomat::span<const diplomat::string_view_for_slice>> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_str_slice(arg.has_value() ? (somelib::diplomat::capi::OptionStringsView{ { {reinterpret_cast<const somelib::diplomat::capi::DiplomatStringView*>(arg.value().data()), arg.value().size()} }, true }) : (somelib::diplomat::capi::OptionStringsView{ {}, false }),
        sentinel);
    return result;
}

inline int64_t somelib::OptionOpaque::accepts_option_primitive(somelib::diplomat::Optional<somelib::diplomat::span<const uint32_t>> arg, uint8_t sentinel) {
    auto result = somelib::capi::OptionOpaque_accepts_option_primitive(arg.has_value() ? (somelib::diplomat::capi::OptionU32View{ { {arg.value().data(), arg.value().size()} }, true }) : (somelib::diplomat::capi::OptionU32View{ {}, false }),
        sentinel);
    return result;
}


#endif // SOMELIB_OptionOpaque_HPP
