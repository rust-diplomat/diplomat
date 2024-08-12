#ifndef CyclicStructB_HPP
#define CyclicStructB_HPP

#include "CyclicStructB.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CyclicStructA.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    diplomat::capi::CyclicStructA CyclicStructB_get_a(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline CyclicStructA CyclicStructB::get_a() {
  auto result = diplomat::capi::CyclicStructB_get_a();
  return CyclicStructA::FromFFI(result);
}


inline diplomat::capi::CyclicStructB CyclicStructB::AsFFI() const {
  return diplomat::capi::CyclicStructB {
    /* .field = */ field,
  };
}

inline CyclicStructB CyclicStructB::FromFFI(diplomat::capi::CyclicStructB c_struct) {
  return CyclicStructB {
    /* .field = */ c_struct.field,
  };
}


#endif // CyclicStructB_HPP
