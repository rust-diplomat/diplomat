#ifndef SOMELIB_MyEnum_D_HPP
#define SOMELIB_MyEnum_D_HPP

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
class MyEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    enum MyEnum {
      MyEnum_A = -2,
      MyEnum_B = -1,
      MyEnum_C = 0,
      MyEnum_D = 1,
      MyEnum_E = 2,
      MyEnum_F = 3,
    };

    typedef struct MyEnum_option {union { MyEnum ok; }; bool is_ok; } MyEnum_option;
} // namespace capi
} // namespace

namespace somelib {
class MyEnum {
public:
    enum Value {
        A = -2,
        /**
         * \deprecated C is the new B
         */
        B [[deprecated("C is the new B")]] = -1,
        C = 0,
        D = 1,
        /**
         * EEEEEEE
         */
        E = 2,
        F = 3,
    };

    MyEnum(): value(Value::D) {}

    // Implicit conversions between enum and ::Value
    constexpr MyEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

  inline int8_t into_value() const;

  inline static somelib::MyEnum get_a();

    inline somelib::capi::MyEnum AsFFI() const;
    inline static somelib::MyEnum FromFFI(somelib::capi::MyEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_MyEnum_D_HPP
