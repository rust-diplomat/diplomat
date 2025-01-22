#ifndef OptionStruct_HPP
#define OptionStruct_HPP

#include "OptionStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::OptionStruct OptionStruct::AsFFI() const {
  return diplomat::capi::OptionStruct {
    /* .a = */ a ? a->AsFFI() : nullptr,
    /* .b = */ b ? b->AsFFI() : nullptr,
    /* .c = */ c,
    /* .d = */ d->AsFFI(),
  };
}

inline OptionStruct OptionStruct::FromFFI(diplomat::capi::OptionStruct c_struct) {
  return OptionStruct {
    /* .a = */ std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(c_struct.a)),
    /* .b = */ std::unique_ptr<OptionOpaqueChar>(OptionOpaqueChar::FromFFI(c_struct.b)),
    /* .c = */ c_struct.c,
    /* .d = */ std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(c_struct.d)),
  };
}


#endif // OptionStruct_HPP
