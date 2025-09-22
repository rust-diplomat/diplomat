#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP

#include "OptionOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::OptionOpaque* OptionOpaque_new(int32_t i);

    diplomat::capi::OptionOpaque* OptionOpaque_new_none(void);

    typedef struct OptionOpaque_returns_result {union {diplomat::capi::OptionStruct ok; }; bool is_ok;} OptionOpaque_returns_result;
    OptionOpaque_returns_result OptionOpaque_returns(void);

    typedef struct OptionOpaque_option_isize_result {union {intptr_t ok; }; bool is_ok;} OptionOpaque_option_isize_result;
    OptionOpaque_option_isize_result OptionOpaque_option_isize(const diplomat::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_usize_result {union {size_t ok; }; bool is_ok;} OptionOpaque_option_usize_result;
    OptionOpaque_option_usize_result OptionOpaque_option_usize(const diplomat::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_i32_result {union {int32_t ok; }; bool is_ok;} OptionOpaque_option_i32_result;
    OptionOpaque_option_i32_result OptionOpaque_option_i32(const diplomat::capi::OptionOpaque* self);

    typedef struct OptionOpaque_option_u32_result {union {uint32_t ok; }; bool is_ok;} OptionOpaque_option_u32_result;
    OptionOpaque_option_u32_result OptionOpaque_option_u32(const diplomat::capi::OptionOpaque* self);

    diplomat::capi::OptionStruct OptionOpaque_new_struct(void);

    diplomat::capi::OptionStruct OptionOpaque_new_struct_nones(void);

    const diplomat::capi::OptionOpaque* OptionOpaque_returns_none_self(const diplomat::capi::OptionOpaque* self);

    const diplomat::capi::OptionOpaque* OptionOpaque_returns_some_self(const diplomat::capi::OptionOpaque* self);

    void OptionOpaque_assert_integer(const diplomat::capi::OptionOpaque* self, int32_t i);

    bool OptionOpaque_option_opaque_argument(const diplomat::capi::OptionOpaque* arg);

    typedef struct OptionOpaque_accepts_option_u8_result {union {uint8_t ok; }; bool is_ok;} OptionOpaque_accepts_option_u8_result;
    OptionOpaque_accepts_option_u8_result OptionOpaque_accepts_option_u8(diplomat::capi::OptionU8 arg, uint8_t sentinel);

    typedef struct OptionOpaque_accepts_option_enum_result {union {diplomat::capi::OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_option_enum_result;
    OptionOpaque_accepts_option_enum_result OptionOpaque_accepts_option_enum(diplomat::capi::OptionEnum_option arg, uint8_t sentinel);

    typedef struct OptionOpaque_accepts_multiple_option_enum_result {union {diplomat::capi::OptionEnum ok; }; bool is_ok;} OptionOpaque_accepts_multiple_option_enum_result;
    OptionOpaque_accepts_multiple_option_enum_result OptionOpaque_accepts_multiple_option_enum(uint8_t sentinel1, diplomat::capi::OptionEnum_option arg1, diplomat::capi::OptionEnum_option arg2, diplomat::capi::OptionEnum_option arg3, uint8_t sentinel2);

    typedef struct OptionOpaque_accepts_option_input_struct_result {union {diplomat::capi::OptionInputStruct ok; }; bool is_ok;} OptionOpaque_accepts_option_input_struct_result;
    OptionOpaque_accepts_option_input_struct_result OptionOpaque_accepts_option_input_struct(diplomat::capi::OptionInputStruct_option arg, uint8_t sentinel);

    diplomat::capi::OptionInputStruct OptionOpaque_returns_option_input_struct(void);

    size_t OptionOpaque_accepts_option_str(diplomat::capi::OptionStringView arg, uint8_t sentinel);

    bool OptionOpaque_accepts_option_str_slice(diplomat::capi::OptionStringsView arg, uint8_t sentinel);

    int64_t OptionOpaque_accepts_option_primitive(diplomat::capi::OptionU32View arg, uint8_t sentinel);

    void OptionOpaque_destroy(OptionOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_(int32_t i) {
    auto result = diplomat::capi::OptionOpaque_new(i);
    return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_none() {
    auto result = diplomat::capi::OptionOpaque_new_none();
    return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline std::optional<OptionStruct> OptionOpaque::returns() {
    auto result = diplomat::capi::OptionOpaque_returns();
    return result.is_ok ? std::optional<OptionStruct>(OptionStruct::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<intptr_t> OptionOpaque::option_isize() const {
    auto result = diplomat::capi::OptionOpaque_option_isize(this->AsFFI());
    return result.is_ok ? std::optional<intptr_t>(result.ok) : std::nullopt;
}

inline std::optional<size_t> OptionOpaque::option_usize() const {
    auto result = diplomat::capi::OptionOpaque_option_usize(this->AsFFI());
    return result.is_ok ? std::optional<size_t>(result.ok) : std::nullopt;
}

inline std::optional<int32_t> OptionOpaque::option_i32() const {
    auto result = diplomat::capi::OptionOpaque_option_i32(this->AsFFI());
    return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<uint32_t> OptionOpaque::option_u32() const {
    auto result = diplomat::capi::OptionOpaque_option_u32(this->AsFFI());
    return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline OptionStruct OptionOpaque::new_struct() {
    auto result = diplomat::capi::OptionOpaque_new_struct();
    return OptionStruct::FromFFI(result);
}

inline OptionStruct OptionOpaque::new_struct_nones() {
    auto result = diplomat::capi::OptionOpaque_new_struct_nones();
    return OptionStruct::FromFFI(result);
}

inline const OptionOpaque* OptionOpaque::returns_none_self() const {
    auto result = diplomat::capi::OptionOpaque_returns_none_self(this->AsFFI());
    return OptionOpaque::FromFFI(result);
}

inline const OptionOpaque* OptionOpaque::returns_some_self() const {
    auto result = diplomat::capi::OptionOpaque_returns_some_self(this->AsFFI());
    return OptionOpaque::FromFFI(result);
}

inline void OptionOpaque::assert_integer(int32_t i) const {
    diplomat::capi::OptionOpaque_assert_integer(this->AsFFI(),
        i);
}

inline bool OptionOpaque::option_opaque_argument(const OptionOpaque* arg) {
    auto result = diplomat::capi::OptionOpaque_option_opaque_argument(arg ? arg->AsFFI() : nullptr);
    return result;
}

inline std::optional<uint8_t> OptionOpaque::accepts_option_u8(std::optional<uint8_t> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_u8(arg.has_value() ? (diplomat::capi::OptionU8{ { arg.value() }, true }) : (diplomat::capi::OptionU8{ {}, false }),
        sentinel);
    return result.is_ok ? std::optional<uint8_t>(result.ok) : std::nullopt;
}

inline std::optional<OptionEnum> OptionOpaque::accepts_option_enum(std::optional<OptionEnum> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_enum(arg.has_value() ? (diplomat::capi::OptionEnum_option{ { arg.value().AsFFI() }, true }) : (diplomat::capi::OptionEnum_option{ {}, false }),
        sentinel);
    return result.is_ok ? std::optional<OptionEnum>(OptionEnum::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<OptionEnum> OptionOpaque::accepts_multiple_option_enum(uint8_t sentinel1, std::optional<OptionEnum> arg1, std::optional<OptionEnum> arg2, std::optional<OptionEnum> arg3, uint8_t sentinel2) {
    auto result = diplomat::capi::OptionOpaque_accepts_multiple_option_enum(sentinel1,
        arg1.has_value() ? (diplomat::capi::OptionEnum_option{ { arg1.value().AsFFI() }, true }) : (diplomat::capi::OptionEnum_option{ {}, false }),
        arg2.has_value() ? (diplomat::capi::OptionEnum_option{ { arg2.value().AsFFI() }, true }) : (diplomat::capi::OptionEnum_option{ {}, false }),
        arg3.has_value() ? (diplomat::capi::OptionEnum_option{ { arg3.value().AsFFI() }, true }) : (diplomat::capi::OptionEnum_option{ {}, false }),
        sentinel2);
    return result.is_ok ? std::optional<OptionEnum>(OptionEnum::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<OptionInputStruct> OptionOpaque::accepts_option_input_struct(std::optional<OptionInputStruct> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_input_struct(arg.has_value() ? (diplomat::capi::OptionInputStruct_option{ { arg.value().AsFFI() }, true }) : (diplomat::capi::OptionInputStruct_option{ {}, false }),
        sentinel);
    return result.is_ok ? std::optional<OptionInputStruct>(OptionInputStruct::FromFFI(result.ok)) : std::nullopt;
}

inline OptionInputStruct OptionOpaque::returns_option_input_struct() {
    auto result = diplomat::capi::OptionOpaque_returns_option_input_struct();
    return OptionInputStruct::FromFFI(result);
}

inline size_t OptionOpaque::accepts_option_str(std::optional<std::string_view> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_str(arg.has_value() ? (diplomat::capi::OptionStringView{ { {arg.value().data(), arg.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        sentinel);
    return result;
}

inline bool OptionOpaque::accepts_option_str_slice(std::optional<diplomat::span<const diplomat::string_view_for_slice>> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_str_slice(arg.has_value() ? (diplomat::capi::OptionStringsView{ { {reinterpret_cast<const diplomat::capi::DiplomatStringView*>(arg.value().data()), arg.value().size()} }, true }) : (diplomat::capi::OptionStringsView{ {}, false }),
        sentinel);
    return result;
}

inline int64_t OptionOpaque::accepts_option_primitive(std::optional<diplomat::span<const uint32_t>> arg, uint8_t sentinel) {
    auto result = diplomat::capi::OptionOpaque_accepts_option_primitive(arg.has_value() ? (diplomat::capi::OptionU32View{ { {arg.value().data(), arg.value().size()} }, true }) : (diplomat::capi::OptionU32View{ {}, false }),
        sentinel);
    return result;
}

inline const diplomat::capi::OptionOpaque* OptionOpaque::AsFFI() const {
    return reinterpret_cast<const diplomat::capi::OptionOpaque*>(this);
}

inline diplomat::capi::OptionOpaque* OptionOpaque::AsFFI() {
    return reinterpret_cast<diplomat::capi::OptionOpaque*>(this);
}

inline const OptionOpaque* OptionOpaque::FromFFI(const diplomat::capi::OptionOpaque* ptr) {
    return reinterpret_cast<const OptionOpaque*>(ptr);
}

inline OptionOpaque* OptionOpaque::FromFFI(diplomat::capi::OptionOpaque* ptr) {
    return reinterpret_cast<OptionOpaque*>(ptr);
}

inline void OptionOpaque::operator delete(void* ptr) {
    diplomat::capi::OptionOpaque_destroy(reinterpret_cast<diplomat::capi::OptionOpaque*>(ptr));
}


#endif // OptionOpaque_HPP
