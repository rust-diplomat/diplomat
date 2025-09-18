#ifndef SOMELIB_DefaultEnum_D_HPP
#define SOMELIB_DefaultEnum_D_HPP

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
class DefaultEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    enum DefaultEnum {
      DefaultEnum_A = 0,
      DefaultEnum_B = 1,
    };

    typedef struct DefaultEnum_option {union { DefaultEnum ok; }; bool is_ok; } DefaultEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class DefaultEnum {
public:
    enum Value {
        A = 0,
        B = 1,
    };

    DefaultEnum(): value(Value::A) {}

    // Implicit conversions between enum and ::Value
    constexpr DefaultEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

  inline static somelib::DefaultEnum new_();

    inline somelib::capi::DefaultEnum AsFFI() const;
    inline static somelib::DefaultEnum FromFFI(somelib::capi::DefaultEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_DefaultEnum_D_HPP
