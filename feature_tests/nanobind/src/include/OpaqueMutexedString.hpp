#ifndef SOMELIB_OpaqueMutexedString_HPP
#define SOMELIB_OpaqueMutexedString_HPP

#include "OpaqueMutexedString.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Utf16Wrap.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::OpaqueMutexedString* OpaqueMutexedString_from_usize(size_t number);

    void OpaqueMutexedString_change(const somelib::capi::OpaqueMutexedString* self, size_t number);

    const somelib::capi::OpaqueMutexedString* OpaqueMutexedString_borrow(const somelib::capi::OpaqueMutexedString* self);

    const somelib::capi::OpaqueMutexedString* OpaqueMutexedString_borrow_other(const somelib::capi::OpaqueMutexedString* other);

    const somelib::capi::OpaqueMutexedString* OpaqueMutexedString_borrow_self_or_other(const somelib::capi::OpaqueMutexedString* self, const somelib::capi::OpaqueMutexedString* other);

    size_t OpaqueMutexedString_get_len_and_add(const somelib::capi::OpaqueMutexedString* self, size_t other);

    somelib::diplomat::capi::DiplomatStringView OpaqueMutexedString_dummy_str(const somelib::capi::OpaqueMutexedString* self);

    somelib::capi::Utf16Wrap* OpaqueMutexedString_wrapper(const somelib::capi::OpaqueMutexedString* self);

    uint16_t OpaqueMutexedString_to_unsigned_from_unsigned(const somelib::capi::OpaqueMutexedString* self, uint16_t input);

    void OpaqueMutexedString_destroy(OpaqueMutexedString* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::OpaqueMutexedString somelib::OpaqueMutexedString::from_usize(size_t number) {
    auto result = somelib::capi::OpaqueMutexedString_from_usize(number);
    return somelib::OpaqueMutexedString::FromFFI(result);
}

inline void somelib::OpaqueMutexedString::change(size_t number) const {
    somelib::capi::OpaqueMutexedString_change(this->AsFFI(),
        number);
}

inline somelib::OpaqueMutexedStringRef somelib::OpaqueMutexedString::borrow() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OpaqueMutexedString_borrow(this->AsFFI());
    return somelib::OpaqueMutexedStringRef::FromFFI(result);
}

inline somelib::OpaqueMutexedStringRef somelib::OpaqueMutexedString::borrow_other(const somelib::OpaqueMutexedString& other DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::OpaqueMutexedString_borrow_other(other.AsFFI());
    return somelib::OpaqueMutexedStringRef::FromFFI(result);
}

inline somelib::OpaqueMutexedStringRef somelib::OpaqueMutexedString::borrow_self_or_other(const somelib::OpaqueMutexedString& other DIPLOMAT_LIFETIME_BOUND) const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OpaqueMutexedString_borrow_self_or_other(this->AsFFI(),
        other.AsFFI());
    return somelib::OpaqueMutexedStringRef::FromFFI(result);
}

inline size_t somelib::OpaqueMutexedString::get_len_and_add(size_t other) const {
    auto result = somelib::capi::OpaqueMutexedString_get_len_and_add(this->AsFFI(),
        other);
    return result;
}

inline std::string_view somelib::OpaqueMutexedString::dummy_str() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OpaqueMutexedString_dummy_str(this->AsFFI());
    return std::string_view(result.data, result.len);
}

inline somelib::Utf16Wrap somelib::OpaqueMutexedString::wrapper() const {
    auto result = somelib::capi::OpaqueMutexedString_wrapper(this->AsFFI());
    return somelib::Utf16Wrap::FromFFI(result);
}

inline uint16_t somelib::OpaqueMutexedString::to_unsigned_from_unsigned(uint16_t input) const {
    auto result = somelib::capi::OpaqueMutexedString_to_unsigned_from_unsigned(this->AsFFI(),
        input);
    return result;
}


#endif // SOMELIB_OpaqueMutexedString_HPP
