#ifndef ContiguousEnum_HPP
#define ContiguousEnum_HPP

#include "ContiguousEnum.d.hpp"

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

inline diplomat::capi::ContiguousEnum ContiguousEnum::AsFFI() const {
  return static_cast<diplomat::capi::ContiguousEnum>(value);
}

inline ContiguousEnum ContiguousEnum::FromFFI(diplomat::capi::ContiguousEnum c_enum) {
  switch (c_enum) {
    case diplomat::capi::ContiguousEnum_C:
    case diplomat::capi::ContiguousEnum_D:
    case diplomat::capi::ContiguousEnum_E:
    case diplomat::capi::ContiguousEnum_F:
      return static_cast<ContiguousEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ContiguousEnum_HPP
