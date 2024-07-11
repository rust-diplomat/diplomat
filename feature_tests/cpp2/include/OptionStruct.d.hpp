#ifndef OptionStruct_D_HPP
#define OptionStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct OptionOpaque; }
class OptionOpaque;
namespace diplomat::capi { struct OptionOpaqueChar; }
class OptionOpaqueChar;


namespace diplomat {
namespace capi {
    struct OptionStruct {
      diplomat::capi::OptionOpaque* a;
      diplomat::capi::OptionOpaqueChar* b;
      uint32_t c;
      diplomat::capi::OptionOpaque* d;
    };
} // namespace capi
} // namespace


struct OptionStruct {
  std::unique_ptr<OptionOpaque> a;
  std::unique_ptr<OptionOpaqueChar> b;
  uint32_t c;
  std::unique_ptr<OptionOpaque> d;

  inline diplomat::capi::OptionStruct AsFFI() const;
  inline static OptionStruct FromFFI(diplomat::capi::OptionStruct c_struct);
};


#endif // OptionStruct_D_HPP
