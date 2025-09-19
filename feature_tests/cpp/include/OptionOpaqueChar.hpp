#ifndef SOMELIB_OptionOpaqueChar_HPP
#define SOMELIB_OptionOpaqueChar_HPP

#include "OptionOpaqueChar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void OptionOpaqueChar_assert_char(const somelib::capi::OptionOpaqueChar* self, char32_t ch);

    void OptionOpaqueChar_destroy(OptionOpaqueChar* self);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::OptionOpaqueChar::assert_char(char32_t ch) const {
    somelib::capi::OptionOpaqueChar_assert_char(this->AsFFI(),
        ch);
}

inline const somelib::capi::OptionOpaqueChar* somelib::OptionOpaqueChar::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OptionOpaqueChar*>(this);
}

inline somelib::capi::OptionOpaqueChar* somelib::OptionOpaqueChar::AsFFI() {
    return reinterpret_cast<somelib::capi::OptionOpaqueChar*>(this);
}

inline const somelib::OptionOpaqueChar* somelib::OptionOpaqueChar::FromFFI(const somelib::capi::OptionOpaqueChar* ptr) {
    return reinterpret_cast<const somelib::OptionOpaqueChar*>(ptr);
}

inline somelib::OptionOpaqueChar* somelib::OptionOpaqueChar::FromFFI(somelib::capi::OptionOpaqueChar* ptr) {
    return reinterpret_cast<somelib::OptionOpaqueChar*>(ptr);
}

inline void somelib::OptionOpaqueChar::operator delete(void* ptr) {
    somelib::capi::OptionOpaqueChar_destroy(reinterpret_cast<somelib::capi::OptionOpaqueChar*>(ptr));
}


#endif // SOMELIB_OptionOpaqueChar_HPP
