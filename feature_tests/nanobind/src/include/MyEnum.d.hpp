#ifndef MyEnum_D_HPP
#define MyEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
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

class MyEnum {
public:
  enum Value {
    A = -2,
    B = -1,
    C = 0,
    D = 1,
    E = 2,
    F = 3,
  };

  MyEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr MyEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline int8_t into_value();

  inline static MyEnum get_a();

  inline diplomat::capi::MyEnum AsFFI() const;
  inline static MyEnum FromFFI(diplomat::capi::MyEnum c_enum);
private:
    Value value;
};


#endif // MyEnum_D_HPP
