#ifndef ContiguousEnum_D_HPP
#define ContiguousEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum ContiguousEnum {
      ContiguousEnum_C = 0,
      ContiguousEnum_D = 1,
      ContiguousEnum_E = 2,
      ContiguousEnum_F = 3,
    };
    
    typedef struct ContiguousEnum_option {union { ContiguousEnum ok; }; bool is_ok; } ContiguousEnum_option;
} // namespace capi
} // namespace

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

  inline diplomat::capi::ContiguousEnum AsFFI() const;
  inline static ContiguousEnum FromFFI(diplomat::capi::ContiguousEnum c_enum);
private:
    Value value;
};


#endif // ContiguousEnum_D_HPP
