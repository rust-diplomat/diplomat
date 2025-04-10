#ifndef OptionEnum_D_HPP
#define OptionEnum_D_HPP

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
    enum OptionEnum {
      OptionEnum_Foo = 0,
      OptionEnum_Bar = 1,
    };
    
    typedef struct OptionEnum_option {union { OptionEnum ok; }; bool is_ok; } OptionEnum_option;
} // namespace capi
} // namespace

class OptionEnum {
public:
  enum Value {
    Foo = 0,
    Bar = 1,
  };

  OptionEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr OptionEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::OptionEnum AsFFI() const;
  inline static OptionEnum FromFFI(diplomat::capi::OptionEnum c_enum);
private:
    Value value;
};


#endif // OptionEnum_D_HPP
