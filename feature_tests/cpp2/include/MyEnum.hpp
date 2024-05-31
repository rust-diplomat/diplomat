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
  return static_cast<capi::MyEnum>(value);
}

inline MyEnum MyEnum::FromFFI(capi::MyEnum c_enum) {
  switch (c_enum) {
    case capi::MyEnum_A:
    case capi::MyEnum_B:
    case capi::MyEnum_C:
    case capi::MyEnum_D:
    case capi::MyEnum_E:
    case capi::MyEnum_F:
      return static_cast<MyEnum::Value>(c_enum);
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
