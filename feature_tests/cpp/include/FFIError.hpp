#ifndef SOMELIB_FFIError_HPP
#define SOMELIB_FFIError_HPP

#include "FFIError.d.hpp"

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

} // namespace capi
} // namespace

inline somelib::capi::FFIError somelib::FFIError::AsFFI() const {
    return static_cast<somelib::capi::FFIError>(value);
}

inline somelib::FFIError somelib::FFIError::FromFFI(somelib::capi::FFIError c_enum) {
    switch (c_enum) {
        case somelib::capi::FFIError_FFI:
        case somelib::capi::FFIError_User:
            return static_cast<somelib::FFIError::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // SOMELIB_FFIError_HPP
