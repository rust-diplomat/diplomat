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


inline capi::UnimportedEnum UnimportedEnum::AsFFI() const {
  switch (value) {
    case A:
      return capi::UnimportedEnum_A;
    case B:
      return capi::UnimportedEnum_B;
    case C:
      return capi::UnimportedEnum_C;
    default:
      abort();
  }
}

inline UnimportedEnum UnimportedEnum::FromFFI(capi::UnimportedEnum c_enum) {
    switch (c_enum) {
    case capi::UnimportedEnum_A:
      return UnimportedEnum::Value::A;
    case capi::UnimportedEnum_B:
      return UnimportedEnum::Value::B;
    case capi::UnimportedEnum_C:
      return UnimportedEnum::Value::C;
    default:
      abort();
  }
}
#endif // UnimportedEnum_HPP
