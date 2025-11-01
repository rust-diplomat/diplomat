#ifndef SOMELIB_UnimportedEnum_D_HPP
#define SOMELIB_UnimportedEnum_D_HPP

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
    enum UnimportedEnum {
      UnimportedEnum_A = 0,
      UnimportedEnum_B = 1,
      UnimportedEnum_C = 2,
    };

    typedef struct UnimportedEnum_option {union { UnimportedEnum ok; }; bool is_ok; } UnimportedEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class UnimportedEnum {
public:
    enum Value {
        A = 0,
        B = 1,
        C = 2,
    };

    UnimportedEnum(): value(Value::A) {}

    // Implicit conversions between enum and ::Value
    constexpr UnimportedEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::UnimportedEnum AsFFI() const;
    inline static somelib::UnimportedEnum FromFFI(somelib::capi::UnimportedEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_UnimportedEnum_D_HPP
