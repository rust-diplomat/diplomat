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


inline ContiguousEnum::ContiguousEnum(ContiguousEnum::Value cpp_value) {
  switch (cpp_value) {
    case C:
      value = capi::ContiguousEnum_C;
      break;
    case D:
      value = capi::ContiguousEnum_D;
      break;
    case E:
      value = capi::ContiguousEnum_E;
      break;
    case F:
      value = capi::ContiguousEnum_F;
      break;
    default:
      abort();
  }
}

inline capi::ContiguousEnum ContiguousEnum::AsFFI() const {
  return value;
}

inline ContiguousEnum ContiguousEnum::FromFFI(capi::ContiguousEnum c_enum) {
  return ContiguousEnum(c_enum);
}

#endif // ContiguousEnum_HPP
