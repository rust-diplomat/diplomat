#ifndef RefListParameter_HPP
#define RefListParameter_HPP

#include "RefListParameter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    void RefListParameter_destroy(RefListParameter* self);
    
    } // extern "C"
}
inline const ::capi::RefListParameter* RefListParameter::AsFFI() const {
  return reinterpret_cast<const ::capi::RefListParameter*>(this);
}

inline ::capi::RefListParameter* RefListParameter::AsFFI() {
  return reinterpret_cast<::capi::RefListParameter*>(this);
}

inline const RefListParameter* RefListParameter::FromFFI(const ::capi::RefListParameter* ptr) {
  return reinterpret_cast<const RefListParameter*>(ptr);
}

inline RefListParameter* RefListParameter::FromFFI(::capi::RefListParameter* ptr) {
  return reinterpret_cast<RefListParameter*>(ptr);
}

inline void RefListParameter::operator delete(void* ptr) {
  capi::RefListParameter_destroy(reinterpret_cast<::capi::RefListParameter*>(ptr));
}


#endif // RefListParameter_HPP
