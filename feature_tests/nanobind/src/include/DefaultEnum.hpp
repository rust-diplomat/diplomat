#ifndef DefaultEnum_HPP
#define DefaultEnum_HPP

#include "DefaultEnum.d.hpp"

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
    
    diplomat::capi::DefaultEnum DefaultEnum_new(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::DefaultEnum DefaultEnum::AsFFI() const {
  return static_cast<diplomat::capi::DefaultEnum>(value);
}

inline DefaultEnum DefaultEnum::FromFFI(diplomat::capi::DefaultEnum c_enum) {
  switch (c_enum) {
    case diplomat::capi::DefaultEnum_A:
    case diplomat::capi::DefaultEnum_B:
      return static_cast<DefaultEnum::Value>(c_enum);
    default:
      abort();
  }
}

inline DefaultEnum DefaultEnum::new_() {
  auto result = diplomat::capi::DefaultEnum_new();
  return DefaultEnum::FromFFI(result);
}
#endif // DefaultEnum_HPP
