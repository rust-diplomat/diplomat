#ifndef CyclicStructB_HPP
#define CyclicStructB_HPP

#include "CyclicStructB.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CyclicStructA.hpp"


namespace capi {
    extern "C" {
    
    ::capi::CyclicStructA CyclicStructB_get_a();
    
    
    } // extern "C"
}
inline CyclicStructA CyclicStructB::get_a() {
  auto result = capi::CyclicStructB_get_a();
  return CyclicStructA::FromFFI(result);
}


inline ::capi::CyclicStructB CyclicStructB::AsFFI() const {
  return ::capi::CyclicStructB {
    .field = field,
  };
}

inline CyclicStructB CyclicStructB::FromFFI(::capi::CyclicStructB c_struct) {
  return CyclicStructB {
    .field = c_struct.field,
  };
}


#endif // CyclicStructB_HPP
