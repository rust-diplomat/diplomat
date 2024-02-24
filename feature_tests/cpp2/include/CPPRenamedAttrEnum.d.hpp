#ifndef CPPRenamedAttrEnum_D_HPP
#define CPPRenamedAttrEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrEnum.d.h"


namespace ns {
class CPPRenamedAttrEnum {
  capi::AttrEnum value;

public:
  enum Value {
    A,
    B,
    CPPRenamed,
  };

  inline CPPRenamedAttrEnum(ns::CPPRenamedAttrEnum::Value cpp_value);
  inline CPPRenamedAttrEnum(capi::AttrEnum c_enum) : value(c_enum) {};

  inline capi::AttrEnum AsFFI() const;
  inline static ns::CPPRenamedAttrEnum FromFFI(capi::AttrEnum c_enum);
};

}
#endif // CPPRenamedAttrEnum_D_HPP
