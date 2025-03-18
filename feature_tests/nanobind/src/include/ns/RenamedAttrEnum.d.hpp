#ifndef ns_RenamedAttrEnum_D_HPP
#define ns_RenamedAttrEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    enum RenamedAttrEnum {
      RenamedAttrEnum_A = 0,
      RenamedAttrEnum_B = 1,
      RenamedAttrEnum_C = 2,
    };
    
    typedef struct RenamedAttrEnum_option {union { RenamedAttrEnum ok; }; bool is_ok; } RenamedAttrEnum_option;
} // namespace capi
} // namespace

namespace ns {
class RenamedAttrEnum {
public:
  enum Value {
    A = 0,
    B = 1,
    Renamed = 2,
  };

  RenamedAttrEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr RenamedAttrEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline ns::capi::RenamedAttrEnum AsFFI() const;
  inline static ns::RenamedAttrEnum FromFFI(ns::capi::RenamedAttrEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // ns_RenamedAttrEnum_D_HPP
