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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::UnimportedEnum UnimportedEnum::AsFFI() const {
  return static_cast<capi::UnimportedEnum>(value);
}

inline UnimportedEnum UnimportedEnum::FromFFI(capi::UnimportedEnum c_enum) {
  switch (c_enum) {
    case capi::UnimportedEnum_A:
    case capi::UnimportedEnum_B:
    case capi::UnimportedEnum_C:
      return static_cast<UnimportedEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // UnimportedEnum_HPP
