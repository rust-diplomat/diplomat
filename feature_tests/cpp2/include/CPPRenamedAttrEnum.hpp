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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::AttrEnum ns::CPPRenamedAttrEnum::AsFFI() const {
  return static_cast<capi::AttrEnum>(value);
}

inline ns::CPPRenamedAttrEnum ns::CPPRenamedAttrEnum::FromFFI(capi::AttrEnum c_enum) {
  switch (c_enum) {
    case capi::AttrEnum_A:
    case capi::AttrEnum_B:
    case capi::AttrEnum_C:
      return static_cast<ns::CPPRenamedAttrEnum::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CPPRenamedAttrEnum_HPP
