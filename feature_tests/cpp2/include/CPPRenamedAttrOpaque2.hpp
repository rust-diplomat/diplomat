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


namespace ns {
namespace capi {
    extern "C" {
    
    
    void namespace_AttrOpaque2_destroy(CPPRenamedAttrOpaque2* self);
    
    } // extern "C"
}
}
inline const ns::capi::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::AsFFI() const {
  return reinterpret_cast<const ns::capi::CPPRenamedAttrOpaque2*>(this);
}

inline ns::capi::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::AsFFI() {
  return reinterpret_cast<ns::capi::CPPRenamedAttrOpaque2*>(this);
}

inline const ns::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::FromFFI(const ns::capi::CPPRenamedAttrOpaque2* ptr) {
  return reinterpret_cast<const ns::CPPRenamedAttrOpaque2*>(ptr);
}

inline ns::CPPRenamedAttrOpaque2* ns::CPPRenamedAttrOpaque2::FromFFI(ns::capi::CPPRenamedAttrOpaque2* ptr) {
  return reinterpret_cast<ns::CPPRenamedAttrOpaque2*>(ptr);
}

inline void ns::CPPRenamedAttrOpaque2::operator delete(void* ptr) {
  capi::namespace_AttrOpaque2_destroy(reinterpret_cast<ns::capi::CPPRenamedAttrOpaque2*>(ptr));
}


#endif // CPPRenamedAttrOpaque2_HPP
