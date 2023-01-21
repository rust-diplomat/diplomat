#ifndef ErrorEnum_HPP
#define ErrorEnum_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ErrorEnum.h"

#include "ErrorEnum.d.hpp"


inline ErrorEnum::ErrorEnum(ErrorEnum::Value cpp_value) {
  switch (cpp_value) {
    case Foo:
      value = capi::ErrorEnum_Foo;
      break;
    case Bar:
      value = capi::ErrorEnum_Bar;
      break;
    default:
      abort();
  }
}

inline capi::ErrorEnum ErrorEnum::AsFFI() const {
  return value;
}

inline ErrorEnum ErrorEnum::FromFFI(capi::ErrorEnum c_enum) {
  return ErrorEnum(c_enum);
}

#endif // ErrorEnum_HPP
