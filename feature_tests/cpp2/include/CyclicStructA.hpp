#ifndef CyclicStructA_HPP
#define CyclicStructA_HPP

#include "CyclicStructA.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CyclicStructB.hpp"


namespace capi {
    extern "C" {
    
    ::capi::CyclicStructB CyclicStructA_get_b();
    
    
    } // extern "C"
}
inline CyclicStructB CyclicStructA::get_b() {
  auto result = capi::CyclicStructA_get_b();
  return CyclicStructB::FromFFI(result);
}


inline ::capi::CyclicStructA CyclicStructA::AsFFI() const {
  return ::capi::CyclicStructA {
    .a = a.AsFFI(),
  };
}

inline CyclicStructA CyclicStructA::FromFFI(::capi::CyclicStructA c_struct) {
  return CyclicStructA {
    .a = CyclicStructB::FromFFI(c_struct.a),
  };
}


#endif // CyclicStructA_HPP
