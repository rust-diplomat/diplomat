#ifndef CPPRenamedAttrEnum_HPP
#define CPPRenamedAttrEnum_HPP

#include "CPPRenamedAttrEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline ns::capi::CPPRenamedAttrEnum ns::CPPRenamedAttrEnum::AsFFI() const {
  return static_cast<ns::capi::CPPRenamedAttrEnum>(value);
}

inline ns::CPPRenamedAttrEnum ns::CPPRenamedAttrEnum::FromFFI(ns::capi::CPPRenamedAttrEnum c_enum) {
  switch (c_enum) {
    case ns::capi::CPPRenamedAttrEnum_A:
    case ns::capi::CPPRenamedAttrEnum_B:
    case ns::capi::CPPRenamedAttrEnum_C:
      return static_cast<ns::CPPRenamedAttrEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CPPRenamedAttrEnum_HPP
