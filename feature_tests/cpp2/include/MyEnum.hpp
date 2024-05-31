#ifndef MyEnum_HPP
#define MyEnum_HPP

#include "MyEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyEnum.h"


inline capi::MyEnum MyEnum::AsFFI() const {
  switch (value) {
    case A:
      return capi::MyEnum_A;
    case B:
      return capi::MyEnum_B;
    case C:
      return capi::MyEnum_C;
    case D:
      return capi::MyEnum_D;
    case E:
      return capi::MyEnum_E;
    case F:
      return capi::MyEnum_F;
    default:
      abort();
  }
}

inline MyEnum MyEnum::FromFFI(capi::MyEnum c_enum) {
    switch (c_enum) {
    case capi::MyEnum_A:
      return MyEnum::Value::A;
    case capi::MyEnum_B:
      return MyEnum::Value::B;
    case capi::MyEnum_C:
      return MyEnum::Value::C;
    case capi::MyEnum_D:
      return MyEnum::Value::D;
    case capi::MyEnum_E:
      return MyEnum::Value::E;
    case capi::MyEnum_F:
      return MyEnum::Value::F;
    default:
      abort();
  }
}

inline int8_t MyEnum::into_value() {
  auto result = capi::MyEnum_into_value(this->AsFFI());
  return result;
}

inline MyEnum MyEnum::get_a() {
  auto result = capi::MyEnum_get_a();
  return MyEnum::FromFFI(result);
}
#endif // MyEnum_HPP
