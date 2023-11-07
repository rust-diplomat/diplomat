#ifndef ContiguousEnum_D_HPP
#define ContiguousEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ContiguousEnum.d.h"


class ContiguousEnum {
  capi::ContiguousEnum value;

public:
  enum Value {
    C,
    D,
    E,
    F,
  };

  inline ContiguousEnum(ContiguousEnum::Value cpp_value);
  inline ContiguousEnum(capi::ContiguousEnum c_enum) : value(c_enum) {};

  inline capi::ContiguousEnum AsFFI() const;
  inline static ContiguousEnum FromFFI(capi::ContiguousEnum c_enum);
};


#endif // ContiguousEnum_D_HPP
