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
public:
  enum Value {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
  };

  ContiguousEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr ContiguousEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ContiguousEnum AsFFI() const;
  inline static ContiguousEnum FromFFI(capi::ContiguousEnum c_enum);
private:
    Value value;
};


#endif // ContiguousEnum_D_HPP
