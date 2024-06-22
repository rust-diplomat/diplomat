#ifndef ErrorEnum_HPP
#define ErrorEnum_HPP

#include "ErrorEnum.d.hpp"

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


inline capi::ErrorEnum ErrorEnum::AsFFI() const {
  return static_cast<capi::ErrorEnum>(value);
}

inline ErrorEnum ErrorEnum::FromFFI(capi::ErrorEnum c_enum) {
  switch (c_enum) {
    case capi::ErrorEnum_Foo:
    case capi::ErrorEnum_Bar:
      return static_cast<ErrorEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ErrorEnum_HPP
