#ifndef OptionStruct_D_HPP
#define OptionStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class OptionOpaque;
class OptionOpaqueChar;


struct OptionStruct {
  std::unique_ptr<OptionOpaque> a;
  std::unique_ptr<OptionOpaqueChar> b;
  uint32_t c;
  std::unique_ptr<OptionOpaque> d;

  inline capi::OptionStruct AsFFI() const;
  inline static OptionStruct FromFFI(capi::OptionStruct ptr);
};


#endif // OptionStruct_D_HPP
