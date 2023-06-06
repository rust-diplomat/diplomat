#ifndef AttrEnum_D_HPP
#define AttrEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrEnum.d.h"


class AttrEnum {
  capi::AttrEnum value;

public:
  enum Value {
    A,
    B,
    CRenamed,
  };

  inline AttrEnum(AttrEnum::Value cpp_value);
  inline AttrEnum(capi::AttrEnum c_enum) : value(c_enum) {};

  inline capi::AttrEnum AsFFI() const;
  inline static AttrEnum FromFFI(capi::AttrEnum c_enum);
};


#endif // AttrEnum_D_HPP
