#ifndef CyclicStructC_HPP
#define CyclicStructC_HPP

#include "CyclicStructC.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "CyclicStructA.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::CyclicStructC CyclicStructC_takes_nested_parameters(diplomat::capi::CyclicStructC c);
    
    void CyclicStructC_cyclic_out(diplomat::capi::CyclicStructC self, diplomat::capi::DiplomatWrite* write);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline CyclicStructC CyclicStructC::takes_nested_parameters(CyclicStructC c) {
  auto result = diplomat::capi::CyclicStructC_takes_nested_parameters(c.AsFFI());
  return CyclicStructC::FromFFI(result);
}

inline std::string CyclicStructC::cyclic_out() {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::CyclicStructC_cyclic_out(this->AsFFI(),
    &write);
  return output;
}


inline diplomat::capi::CyclicStructC CyclicStructC::AsFFI() const {
  return diplomat::capi::CyclicStructC {
    /* .a = */ a.AsFFI(),
  };
}

inline CyclicStructC CyclicStructC::FromFFI(diplomat::capi::CyclicStructC c_struct) {
  return CyclicStructC {
    /* .a = */ CyclicStructA::FromFFI(c_struct.a),
  };
}


#endif // CyclicStructC_HPP
