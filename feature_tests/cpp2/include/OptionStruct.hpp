#ifndef OptionStruct_HPP
#define OptionStruct_HPP

#include "OptionStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionStruct.h"



inline capi::OptionStruct OptionStruct::AsFFI() const {
  return capi::OptionStruct {
    .a = a.AsFFI(),
    .b = b.AsFFI(),
    .c = c,
    .d = d.AsFFI(),
  };
}

inline OptionStruct OptionStruct::FromFFI(capi::OptionStruct c_struct) {
  return OptionStruct {
    .a = std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(c_struct.a)),
    .b = std::unique_ptr<OptionOpaqueChar>(OptionOpaqueChar::FromFFI(c_struct.b)),
    .c = c_struct.c,
    .d = std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(c_struct.d)),
  };
}


#endif // OptionStruct_HPP
