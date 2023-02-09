#ifndef UnimportedEnum_D_HPP
#define UnimportedEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "UnimportedEnum.d.h"


class UnimportedEnum {
  capi::UnimportedEnum value;

public:
  enum Value {
    A,
    B,
    C,
  };

  inline UnimportedEnum(UnimportedEnum::Value cpp_value);
  inline UnimportedEnum(capi::UnimportedEnum c_enum) : value(c_enum) {};

  inline capi::UnimportedEnum AsFFI() const;
  inline static UnimportedEnum FromFFI(capi::UnimportedEnum c_enum);
};


#endif // UnimportedEnum_D_HPP
