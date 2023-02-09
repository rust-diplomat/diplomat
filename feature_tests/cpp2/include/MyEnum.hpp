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


inline MyEnum::MyEnum(MyEnum::Value cpp_value) {
  switch (cpp_value) {
    case A:
      value = capi::MyEnum_A;
      break;
    case B:
      value = capi::MyEnum_B;
      break;
    case C:
      value = capi::MyEnum_C;
      break;
    case D:
      value = capi::MyEnum_D;
      break;
    case E:
      value = capi::MyEnum_E;
      break;
    case F:
      value = capi::MyEnum_F;
      break;
    default:
      abort();
  }
}

inline capi::MyEnum MyEnum::AsFFI() const {
  return value;
}

inline MyEnum MyEnum::FromFFI(capi::MyEnum c_enum) {
  return MyEnum(c_enum);
}

#endif // MyEnum_HPP
