#ifndef ns_RenamedAttrEnum_HPP
#define ns_RenamedAttrEnum_HPP

#include "RenamedAttrEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline ns::capi::RenamedAttrEnum ns::RenamedAttrEnum::AsFFI() const {
  return static_cast<ns::capi::RenamedAttrEnum>(value);
}

inline ns::RenamedAttrEnum ns::RenamedAttrEnum::FromFFI(ns::capi::RenamedAttrEnum c_enum) {
  switch (c_enum) {
    case ns::capi::RenamedAttrEnum_A:
    case ns::capi::RenamedAttrEnum_B:
    case ns::capi::RenamedAttrEnum_C:
      return static_cast<ns::RenamedAttrEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ns_RenamedAttrEnum_HPP
