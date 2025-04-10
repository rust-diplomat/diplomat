#ifndef RefListParameter_HPP
#define RefListParameter_HPP

#include "RefListParameter.d.hpp"

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
    
    
    void RefListParameter_destroy(RefListParameter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const diplomat::capi::RefListParameter* RefListParameter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::RefListParameter*>(this);
}

inline diplomat::capi::RefListParameter* RefListParameter::AsFFI() {
  return reinterpret_cast<diplomat::capi::RefListParameter*>(this);
}

inline const RefListParameter* RefListParameter::FromFFI(const diplomat::capi::RefListParameter* ptr) {
  return reinterpret_cast<const RefListParameter*>(ptr);
}

inline RefListParameter* RefListParameter::FromFFI(diplomat::capi::RefListParameter* ptr) {
  return reinterpret_cast<RefListParameter*>(ptr);
}

inline void RefListParameter::operator delete(void* ptr) {
  diplomat::capi::RefListParameter_destroy(reinterpret_cast<diplomat::capi::RefListParameter*>(ptr));
}


#endif // RefListParameter_HPP
