#ifndef SOMELIB_ErrorEnum_D_HPP
#define SOMELIB_ErrorEnum_D_HPP

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
    enum ErrorEnum {
      ErrorEnum_Foo = 0,
      ErrorEnum_Bar = 1,
    };

    typedef struct ErrorEnum_option {union { ErrorEnum ok; }; bool is_ok; } ErrorEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class ErrorEnum {
public:
    enum Value {
        Foo = 0,
        Bar = 1,
    };

    ErrorEnum(): value(Value::Foo) {}

    // Implicit conversions between enum and ::Value
    constexpr ErrorEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::ErrorEnum AsFFI() const;
    inline static somelib::ErrorEnum FromFFI(somelib::capi::ErrorEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_ErrorEnum_D_HPP
