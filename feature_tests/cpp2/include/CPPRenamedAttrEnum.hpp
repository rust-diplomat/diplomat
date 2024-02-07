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


inline CPPRenamedAttrEnum::CPPRenamedAttrEnum(CPPRenamedAttrEnum::Value cpp_value) {
  switch (cpp_value) {
    case A:
      value = capi::AttrEnum_A;
      break;
    case B:
      value = capi::AttrEnum_B;
      break;
    case CPPRenamed:
      value = capi::AttrEnum_C;
      break;
    default:
      abort();
  }
}

inline capi::AttrEnum CPPRenamedAttrEnum::AsFFI() const {
  return value;
}

inline CPPRenamedAttrEnum CPPRenamedAttrEnum::FromFFI(capi::AttrEnum c_enum) {
  return CPPRenamedAttrEnum(c_enum);
}

#endif // CPPRenamedAttrEnum_HPP
