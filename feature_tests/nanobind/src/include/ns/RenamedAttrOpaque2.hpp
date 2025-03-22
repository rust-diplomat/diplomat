#ifndef ns_RenamedAttrOpaque2_HPP
#define ns_RenamedAttrOpaque2_HPP

#include "RenamedAttrOpaque2.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    
    
    void namespace_AttrOpaque2_destroy(RenamedAttrOpaque2* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const ns::capi::RenamedAttrOpaque2* ns::RenamedAttrOpaque2::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedAttrOpaque2*>(this);
}

inline ns::capi::RenamedAttrOpaque2* ns::RenamedAttrOpaque2::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedAttrOpaque2*>(this);
}

inline const ns::RenamedAttrOpaque2* ns::RenamedAttrOpaque2::FromFFI(const ns::capi::RenamedAttrOpaque2* ptr) {
  return reinterpret_cast<const ns::RenamedAttrOpaque2*>(ptr);
}

inline ns::RenamedAttrOpaque2* ns::RenamedAttrOpaque2::FromFFI(ns::capi::RenamedAttrOpaque2* ptr) {
  return reinterpret_cast<ns::RenamedAttrOpaque2*>(ptr);
}

inline void ns::RenamedAttrOpaque2::operator delete(void* ptr) {
  ns::capi::namespace_AttrOpaque2_destroy(reinterpret_cast<ns::capi::RenamedAttrOpaque2*>(ptr));
}


#endif // ns_RenamedAttrOpaque2_HPP
