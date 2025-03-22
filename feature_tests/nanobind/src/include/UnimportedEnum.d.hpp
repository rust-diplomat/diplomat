#ifndef UnimportedEnum_D_HPP
#define UnimportedEnum_D_HPP

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
    enum UnimportedEnum {
      UnimportedEnum_A = 0,
      UnimportedEnum_B = 1,
      UnimportedEnum_C = 2,
    };
    
    typedef struct UnimportedEnum_option {union { UnimportedEnum ok; }; bool is_ok; } UnimportedEnum_option;
} // namespace capi
} // namespace

class UnimportedEnum {
public:
  enum Value {
    A = 0,
    B = 1,
    C = 2,
  };

  UnimportedEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr UnimportedEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::UnimportedEnum AsFFI() const;
  inline static UnimportedEnum FromFFI(diplomat::capi::UnimportedEnum c_enum);
private:
    Value value;
};


#endif // UnimportedEnum_D_HPP
