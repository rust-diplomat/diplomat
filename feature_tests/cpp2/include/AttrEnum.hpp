#ifndef AttrEnum_HPP
#define AttrEnum_HPP

#include "AttrEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrEnum.h"


inline AttrEnum::AttrEnum(AttrEnum::Value cpp_value) {
  switch (cpp_value) {
    case A:
      value = capi::AttrEnum_A;
      break;
    case B:
      value = capi::AttrEnum_B;
      break;
    case CRenamed:
      value = capi::AttrEnum_C;
      break;
    default:
      abort();
  }
}

inline capi::AttrEnum AttrEnum::AsFFI() const {
  return value;
}

inline AttrEnum AttrEnum::FromFFI(capi::AttrEnum c_enum) {
  return AttrEnum(c_enum);
}

#endif // AttrEnum_HPP
