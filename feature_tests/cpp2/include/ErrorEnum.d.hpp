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
public:
  enum Value {
    Foo,
    Bar,
  };

  ErrorEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr ErrorEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ErrorEnum AsFFI() const;
  inline static ErrorEnum FromFFI(capi::ErrorEnum c_enum);
private:
    Value value;
};


#endif // ErrorEnum_D_HPP
