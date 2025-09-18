#ifndef SOMELIB_ContiguousEnum_D_HPP
#define SOMELIB_ContiguousEnum_D_HPP

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
    enum ContiguousEnum {
      ContiguousEnum_C = 0,
      ContiguousEnum_D = 1,
      ContiguousEnum_E = 2,
      ContiguousEnum_F = 3,
    };

    typedef struct ContiguousEnum_option {union { ContiguousEnum ok; }; bool is_ok; } ContiguousEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class ContiguousEnum {
public:
    enum Value {
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    };

    ContiguousEnum(): value(Value::E) {}

    // Implicit conversions between enum and ::Value
    constexpr ContiguousEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::ContiguousEnum AsFFI() const;
    inline static somelib::ContiguousEnum FromFFI(somelib::capi::ContiguousEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_ContiguousEnum_D_HPP
