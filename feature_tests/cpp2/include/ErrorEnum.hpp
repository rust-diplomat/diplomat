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
#include "ErrorEnum.h"


inline capi::ErrorEnum ErrorEnum::AsFFI() const {
  switch (value) {
    case Foo:
      return capi::ErrorEnum_Foo;
    case Bar:
      return capi::ErrorEnum_Bar;
    default:
      abort();
  }
}

inline ErrorEnum ErrorEnum::FromFFI(capi::ErrorEnum c_enum) {
    switch (c_enum) {
    case capi::ErrorEnum_Foo:
      return ErrorEnum::Value::Foo;
    case capi::ErrorEnum_Bar:
      return ErrorEnum::Value::Bar;
    default:
      abort();
  }
}
#endif // ErrorEnum_HPP
