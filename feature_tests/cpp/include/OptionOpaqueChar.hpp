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


#endif // SOMELIB_OptionOpaqueChar_HPP
