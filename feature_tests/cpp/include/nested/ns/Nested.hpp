#ifndef nested_ns_Nested_HPP
#define nested_ns_Nested_HPP

#include "Nested.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../../diplomat_runtime.hpp"


namespace nested::ns {
namespace capi {
    extern "C" {
    
    
    void namespace_Nested_destroy(Nested* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const nested::ns::capi::Nested* nested::ns::Nested::AsFFI() const {
  return reinterpret_cast<const nested::ns::capi::Nested*>(this);
}

inline nested::ns::capi::Nested* nested::ns::Nested::AsFFI() {
  return reinterpret_cast<nested::ns::capi::Nested*>(this);
}

inline const nested::ns::Nested* nested::ns::Nested::FromFFI(const nested::ns::capi::Nested* ptr) {
  return reinterpret_cast<const nested::ns::Nested*>(ptr);
}

inline nested::ns::Nested* nested::ns::Nested::FromFFI(nested::ns::capi::Nested* ptr) {
  return reinterpret_cast<nested::ns::Nested*>(ptr);
}

inline void nested::ns::Nested::operator delete(void* ptr) {
  nested::ns::capi::namespace_Nested_destroy(reinterpret_cast<nested::ns::capi::Nested*>(ptr));
}


#endif // nested_ns_Nested_HPP
