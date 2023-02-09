#ifndef UnimportedEnum_HPP
#define UnimportedEnum_HPP

#include "UnimportedEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "UnimportedEnum.h"


inline UnimportedEnum::UnimportedEnum(UnimportedEnum::Value cpp_value) {
  switch (cpp_value) {
    case A:
      value = capi::UnimportedEnum_A;
      break;
    case B:
      value = capi::UnimportedEnum_B;
      break;
    case C:
      value = capi::UnimportedEnum_C;
      break;
    default:
      abort();
  }
}

inline capi::UnimportedEnum UnimportedEnum::AsFFI() const {
  return value;
}

inline UnimportedEnum UnimportedEnum::FromFFI(capi::UnimportedEnum c_enum) {
  return UnimportedEnum(c_enum);
}

#endif // UnimportedEnum_HPP
