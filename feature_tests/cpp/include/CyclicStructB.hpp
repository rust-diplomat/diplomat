#ifndef CyclicStructB_HPP
#define CyclicStructB_HPP

#include "CyclicStructB.d.hpp"

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
    
    diplomat::capi::CyclicStructA CyclicStructB_get_a(void);
    
    typedef struct CyclicStructB_get_a_option_result {union {diplomat::capi::CyclicStructA ok; }; bool is_ok;} CyclicStructB_get_a_option_result;
    CyclicStructB_get_a_option_result CyclicStructB_get_a_option(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline CyclicStructA CyclicStructB::get_a() {
  auto result = diplomat::capi::CyclicStructB_get_a();
  return CyclicStructA::FromFFI(result);
}

inline std::optional<CyclicStructA> CyclicStructB::get_a_option() {
  auto result = diplomat::capi::CyclicStructB_get_a_option();
  return result.is_ok ? std::optional<CyclicStructA>(CyclicStructA::FromFFI(result.ok)) : std::nullopt;
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
