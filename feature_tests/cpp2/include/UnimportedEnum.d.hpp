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
public:
  enum Value {
    A,
    B,
    C,
  };

  UnimportedEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr UnimportedEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::UnimportedEnum AsFFI() const;
  inline static UnimportedEnum FromFFI(capi::UnimportedEnum c_enum);
private:
    Value value;
};


#endif // UnimportedEnum_D_HPP
