#ifndef MyEnum_D_HPP
#define MyEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyEnum.d.h"


class MyEnum {
  capi::MyEnum value;

public:
  enum Value {
    A,
    B,
    C,
    D,
    E,
    F,
  };

  inline int8_t into_value();

  inline static MyEnum get_a();

  inline MyEnum(MyEnum::Value cpp_value);
  inline MyEnum(capi::MyEnum c_enum) : value(c_enum) {};

  inline capi::MyEnum AsFFI() const;
  inline static MyEnum FromFFI(capi::MyEnum c_enum);
};


#endif // MyEnum_D_HPP
