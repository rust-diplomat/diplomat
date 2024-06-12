#ifndef CPPRenamedAttrEnum_D_HPP
#define CPPRenamedAttrEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum AttrEnum {
      AttrEnum_A = 0,
      AttrEnum_B = 1,
      AttrEnum_C = 2,
    } AttrEnum;
}namespace ns {
class CPPRenamedAttrEnum {
public:
  enum Value {
    A = 0,
    B = 1,
    CPPRenamed = 2,
  };

  CPPRenamedAttrEnum() = default;
  // Implicit conversions between enum and ::Value
  constexpr CPPRenamedAttrEnum(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::AttrEnum AsFFI() const;
  inline static ns::CPPRenamedAttrEnum FromFFI(capi::AttrEnum c_enum);
private:
    Value value;
};

}
#endif // CPPRenamedAttrEnum_D_HPP
