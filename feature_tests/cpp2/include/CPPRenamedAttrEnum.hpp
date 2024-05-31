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
#include "AttrEnum.h"


inline capi::AttrEnum ns::CPPRenamedAttrEnum::AsFFI() const {
  switch (value) {
    case A:
      return capi::AttrEnum_A;
    case B:
      return capi::AttrEnum_B;
    case CPPRenamed:
      return capi::AttrEnum_C;
    default:
      abort();
  }
}

inline ns::CPPRenamedAttrEnum ns::CPPRenamedAttrEnum::FromFFI(capi::AttrEnum c_enum) {
    switch (c_enum) {
    case capi::AttrEnum_A:
      return ns::CPPRenamedAttrEnum::Value::A;
    case capi::AttrEnum_B:
      return ns::CPPRenamedAttrEnum::Value::B;
    case capi::AttrEnum_C:
      return ns::CPPRenamedAttrEnum::Value::CPPRenamed;
    default:
      abort();
  }
}
#endif // CPPRenamedAttrEnum_HPP
