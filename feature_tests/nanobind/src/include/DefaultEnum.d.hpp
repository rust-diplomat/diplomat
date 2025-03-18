#ifndef DefaultEnum_D_HPP
#define DefaultEnum_D_HPP

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
    enum DefaultEnum {
      DefaultEnum_A = 0,
      DefaultEnum_B = 1,
    };
    
    typedef struct DefaultEnum_option {union { DefaultEnum ok; }; bool is_ok; } DefaultEnum_option;
} // namespace capi
} // namespace

class DefaultEnum {
public:
  enum Value {
    A = 0,
    B = 1,
  };

  DefaultEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr DefaultEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline static DefaultEnum new_();

  inline diplomat::capi::DefaultEnum AsFFI() const;
  inline static DefaultEnum FromFFI(diplomat::capi::DefaultEnum c_enum);
private:
    Value value;
};


#endif // DefaultEnum_D_HPP
