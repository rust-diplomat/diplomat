#ifndef CPPRenamedAttrOpaque2_HPP
#define CPPRenamedAttrOpaque2_HPP

#include "CPPRenamedAttrOpaque2.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    void namespace_AttrOpaque2_destroy(AttrOpaque2* self);
    
    } // extern "C"
}

inline const capi::AttrOpaque2* ns::CPPRenamedAttrOpaque2::AsFFI() const {
  return reinterpret_cast<const capi::AttrOpaque2*>(this);
}

inline capi::AttrOpaque2* ns::CPPRenamedAttrOpaque2::AsFFI() {
  return reinterpret_cast<capi::AttrOpaque2*>(this);
}

inline const ns::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::FromFFI(const capi::AttrOpaque2* ptr) {
  return reinterpret_cast<const ns::CPPRenamedAttrOpaque2*>(ptr);
}

inline ns::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::FromFFI(capi::AttrOpaque2* ptr) {
  return reinterpret_cast<ns::CPPRenamedAttrOpaque2*>(ptr);
}

inline void ns::CPPRenamedAttrOpaque2::operator delete(void* ptr) {
  capi::namespace_AttrOpaque2_destroy(reinterpret_cast<capi::AttrOpaque2*>(ptr));
}


#endif // CPPRenamedAttrOpaque2_HPP
