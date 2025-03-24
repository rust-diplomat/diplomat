#ifndef CallbackTestingStruct_HPP
#define CallbackTestingStruct_HPP

#include "CallbackTestingStruct.d.hpp"

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
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::CallbackTestingStruct CallbackTestingStruct::AsFFI() const {
  return diplomat::capi::CallbackTestingStruct {
    /* .x = */ x,
    /* .y = */ y,
  };
}

inline CallbackTestingStruct CallbackTestingStruct::FromFFI(diplomat::capi::CallbackTestingStruct c_struct) {
  return CallbackTestingStruct {
    /* .x = */ c_struct.x,
    /* .y = */ c_struct.y,
  };
}


#endif // CallbackTestingStruct_HPP
