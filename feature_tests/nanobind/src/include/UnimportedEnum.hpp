#ifndef UnimportedEnum_HPP
#define UnimportedEnum_HPP

#include "UnimportedEnum.d.hpp"

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

inline diplomat::capi::UnimportedEnum UnimportedEnum::AsFFI() const {
  return static_cast<diplomat::capi::UnimportedEnum>(value);
}

inline UnimportedEnum UnimportedEnum::FromFFI(diplomat::capi::UnimportedEnum c_enum) {
  switch (c_enum) {
    case diplomat::capi::UnimportedEnum_A:
    case diplomat::capi::UnimportedEnum_B:
    case diplomat::capi::UnimportedEnum_C:
      return static_cast<UnimportedEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // UnimportedEnum_HPP
