#ifndef SOMELIB_FFIError_D_HPP
#define SOMELIB_FFIError_D_HPP

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
    enum FFIError {
      FFIError_FFI = 0,
      FFIError_User = 1,
    };

    typedef struct FFIError_option {union { FFIError ok; }; bool is_ok; } FFIError_option;
} // namespace capi
} // namespace

namespace somelib {
class FFIError {
public:
    enum Value {
        FFI = 0,
        User = 1,
    };

    FFIError(): value(Value::FFI) {}

    // Implicit conversions between enum and ::Value
    constexpr FFIError(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::FFIError AsFFI() const;
    inline static somelib::FFIError FromFFI(somelib::capi::FFIError c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_FFIError_D_HPP
