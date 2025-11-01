#ifndef SOMELIB_OptionEnum_D_HPP
#define SOMELIB_OptionEnum_D_HPP

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
    enum OptionEnum {
      OptionEnum_Foo = 0,
      OptionEnum_Bar = 1,
      OptionEnum_Baz = 2,
    };

    typedef struct OptionEnum_option {union { OptionEnum ok; }; bool is_ok; } OptionEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class OptionEnum {
public:
    enum Value {
        Foo = 0,
        Bar = 1,
        Baz = 2,
    };

    OptionEnum(): value(Value::Foo) {}

    // Implicit conversions between enum and ::Value
    constexpr OptionEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::OptionEnum AsFFI() const;
    inline static somelib::OptionEnum FromFFI(somelib::capi::OptionEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_OptionEnum_D_HPP
