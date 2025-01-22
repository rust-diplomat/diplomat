#ifndef MyEnum_HPP
#define MyEnum_HPP

#include "MyEnum.d.hpp"

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
    extern "C" {
    
    int8_t MyEnum_into_value(diplomat::capi::MyEnum self);
    
    diplomat::capi::MyEnum MyEnum_get_a(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::MyEnum MyEnum::AsFFI() const {
  return static_cast<diplomat::capi::MyEnum>(value);
}

inline MyEnum MyEnum::FromFFI(diplomat::capi::MyEnum c_enum) {
  switch (c_enum) {
    case diplomat::capi::MyEnum_A:
    case diplomat::capi::MyEnum_B:
    case diplomat::capi::MyEnum_C:
    case diplomat::capi::MyEnum_D:
    case diplomat::capi::MyEnum_E:
    case diplomat::capi::MyEnum_F:
      return static_cast<MyEnum::Value>(c_enum);
    default:
      abort();
  }
}

inline int8_t MyEnum::into_value() {
  auto result = diplomat::capi::MyEnum_into_value(this->AsFFI());
  return result;
}

inline MyEnum MyEnum::get_a() {
  auto result = diplomat::capi::MyEnum_get_a();
  return MyEnum::FromFFI(result);
}
#endif // MyEnum_HPP
