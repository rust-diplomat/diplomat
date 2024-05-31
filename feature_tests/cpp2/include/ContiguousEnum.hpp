#ifndef ContiguousEnum_HPP
#define ContiguousEnum_HPP

#include "ContiguousEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ContiguousEnum.h"


inline capi::ContiguousEnum ContiguousEnum::AsFFI() const {
  return static_cast<capi::ContiguousEnum>(value);
}

inline ContiguousEnum ContiguousEnum::FromFFI(capi::ContiguousEnum c_enum) {
  switch (c_enum) {
    case capi::ContiguousEnum_C:
    case capi::ContiguousEnum_D:
    case capi::ContiguousEnum_E:
    case capi::ContiguousEnum_F:
      return static_cast<ContiguousEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ContiguousEnum_HPP
