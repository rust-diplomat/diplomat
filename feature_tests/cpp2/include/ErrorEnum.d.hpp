#ifndef ErrorEnum_D_HPP
#define ErrorEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ErrorEnum.d.h"


class ErrorEnum {
  capi::ErrorEnum value;

public:
  enum Value {
    Foo,
    Bar,
  };

  inline ErrorEnum(ErrorEnum::Value cpp_value);
  inline ErrorEnum(capi::ErrorEnum c_enum) : value(c_enum) {};

  inline capi::ErrorEnum AsFFI() const;
  inline static ErrorEnum FromFFI(capi::ErrorEnum c_enum);
};


#endif // ErrorEnum_D_HPP
