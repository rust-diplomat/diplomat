#ifndef OptionInputStruct_HPP
#define OptionInputStruct_HPP

#include "OptionInputStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "OptionEnum.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::OptionInputStruct OptionInputStruct::AsFFI() const {
  return diplomat::capi::OptionInputStruct {
    /* .a = */ a.has_value() ? (diplomat::capi::OptionU8{ { a.value() }, true }) : (diplomat::capi::OptionU8{ {}, false }),
    /* .b = */ b.has_value() ? (diplomat::capi::OptionChar{ { b.value() }, true }) : (diplomat::capi::OptionChar{ {}, false }),
    /* .c = */ c.has_value() ? (diplomat::capi::OptionEnum_option{ { c.value().AsFFI() }, true }) : (diplomat::capi::OptionEnum_option{ {}, false }),
  };
}

inline OptionInputStruct OptionInputStruct::FromFFI(diplomat::capi::OptionInputStruct c_struct) {
  return OptionInputStruct {
    /* .a = */ c_struct.a.is_ok ? std::optional(c_struct.a.ok) : std::nullopt,
    /* .b = */ c_struct.b.is_ok ? std::optional(c_struct.b.ok) : std::nullopt,
    /* .c = */ c_struct.c.is_ok ? std::optional(OptionEnum::FromFFI(c_struct.c.ok)) : std::nullopt,
  };
}


#endif // OptionInputStruct_HPP
