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
  switch (value) {
    case C:
      return capi::ContiguousEnum_C;
    case D:
      return capi::ContiguousEnum_D;
    case E:
      return capi::ContiguousEnum_E;
    case F:
      return capi::ContiguousEnum_F;
    default:
      abort();
  }
}

inline ContiguousEnum ContiguousEnum::FromFFI(capi::ContiguousEnum c_enum) {
    switch (c_enum) {
    case capi::ContiguousEnum_C:
      return ContiguousEnum::Value::C;
    case capi::ContiguousEnum_D:
      return ContiguousEnum::Value::D;
    case capi::ContiguousEnum_E:
      return ContiguousEnum::Value::E;
    case capi::ContiguousEnum_F:
      return ContiguousEnum::Value::F;
    default:
      abort();
  }
}
#endif // ContiguousEnum_HPP
