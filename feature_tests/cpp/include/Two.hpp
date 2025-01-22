#ifndef Two_HPP
#define Two_HPP

#include "Two.d.hpp"

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
    
    
    void Two_destroy(Two* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const diplomat::capi::Two* Two::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Two*>(this);
}

inline diplomat::capi::Two* Two::AsFFI() {
  return reinterpret_cast<diplomat::capi::Two*>(this);
}

inline const Two* Two::FromFFI(const diplomat::capi::Two* ptr) {
  return reinterpret_cast<const Two*>(ptr);
}

inline Two* Two::FromFFI(diplomat::capi::Two* ptr) {
  return reinterpret_cast<Two*>(ptr);
}

inline void Two::operator delete(void* ptr) {
  diplomat::capi::Two_destroy(reinterpret_cast<diplomat::capi::Two*>(ptr));
}


#endif // Two_HPP
