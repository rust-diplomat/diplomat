#ifndef ErrorEnum_HPP
#define ErrorEnum_HPP

#include "ErrorEnum.d.hpp"

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
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::ErrorEnum ErrorEnum::AsFFI() const {
  return static_cast<diplomat::capi::ErrorEnum>(value);
}

inline ErrorEnum ErrorEnum::FromFFI(diplomat::capi::ErrorEnum c_enum) {
  switch (c_enum) {
    case diplomat::capi::ErrorEnum_Foo:
    case diplomat::capi::ErrorEnum_Bar:
      return static_cast<ErrorEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ErrorEnum_HPP
