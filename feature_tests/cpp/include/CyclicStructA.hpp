#ifndef CyclicStructA_HPP
#define CyclicStructA_HPP

#include "CyclicStructA.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CyclicStructB.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::CyclicStructB CyclicStructA_get_b();
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline CyclicStructB CyclicStructA::get_b() {
  auto result = diplomat::capi::CyclicStructA_get_b();
  return CyclicStructB::FromFFI(result);
}


inline diplomat::capi::CyclicStructA CyclicStructA::AsFFI() const {
  return diplomat::capi::CyclicStructA {
    .a = a.AsFFI(),
  };
}

inline CyclicStructA CyclicStructA::FromFFI(diplomat::capi::CyclicStructA c_struct) {
  return CyclicStructA {
    .a = CyclicStructB::FromFFI(c_struct.a),
  };
}


#endif // CyclicStructA_HPP
