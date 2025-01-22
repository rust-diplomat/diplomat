#ifndef nested_ns2_Nested_HPP
#define nested_ns2_Nested_HPP

#include "Nested.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../../diplomat_runtime.hpp"


namespace nested::ns2 {
namespace capi {
    extern "C" {
    
    
    void namespace_Nested2_destroy(Nested* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const nested::ns2::capi::Nested* nested::ns2::Nested::AsFFI() const {
  return reinterpret_cast<const nested::ns2::capi::Nested*>(this);
}

inline nested::ns2::capi::Nested* nested::ns2::Nested::AsFFI() {
  return reinterpret_cast<nested::ns2::capi::Nested*>(this);
}

inline const nested::ns2::Nested* nested::ns2::Nested::FromFFI(const nested::ns2::capi::Nested* ptr) {
  return reinterpret_cast<const nested::ns2::Nested*>(ptr);
}

inline nested::ns2::Nested* nested::ns2::Nested::FromFFI(nested::ns2::capi::Nested* ptr) {
  return reinterpret_cast<nested::ns2::Nested*>(ptr);
}

inline void nested::ns2::Nested::operator delete(void* ptr) {
  nested::ns2::capi::namespace_Nested2_destroy(reinterpret_cast<nested::ns2::capi::Nested*>(ptr));
}


#endif // nested_ns2_Nested_HPP
