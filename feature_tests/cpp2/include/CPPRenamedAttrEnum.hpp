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
}
}
inline ns::capi::AttrEnum ns::CPPRenamedAttrEnum::AsFFI() const {
  return static_cast<ns::capi::AttrEnum>(value);
}

inline ns::CPPRenamedAttrEnum ns::CPPRenamedAttrEnum::FromFFI(ns::capi::AttrEnum c_enum) {
  switch (c_enum) {
    case ns::capi::AttrEnum_A:
    case ns::capi::AttrEnum_B:
    case ns::capi::AttrEnum_C:
      return static_cast<ns::CPPRenamedAttrEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CPPRenamedAttrEnum_HPP
