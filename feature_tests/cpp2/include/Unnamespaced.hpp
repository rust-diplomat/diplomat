#ifndef Unnamespaced_HPP
#define Unnamespaced_HPP

#include "Unnamespaced.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque1Renamed.hpp"
#include "CPPRenamedAttrEnum.hpp"


namespace capi {
    extern "C" {
    
    ::capi::Unnamespaced* namespace_Unnamespaced_make(ns::capi::CPPRenamedAttrEnum _e);
    
    void namespace_Unnamespaced_use_namespaced(const ::capi::Unnamespaced* self, const ns::capi::AttrOpaque1Renamed* _n);
    
    
    void namespace_Unnamespaced_destroy(Unnamespaced* self);
    
    } // extern "C"
}
inline std::unique_ptr<Unnamespaced> Unnamespaced::make(ns::CPPRenamedAttrEnum _e) {
  auto result = capi::namespace_Unnamespaced_make(_e.AsFFI());
  return std::unique_ptr<Unnamespaced>(Unnamespaced::FromFFI(result));
}

inline void Unnamespaced::use_namespaced(const ns::AttrOpaque1Renamed& _n) const {
  capi::namespace_Unnamespaced_use_namespaced(this->AsFFI(),
    _n.AsFFI());
}

inline const ::capi::Unnamespaced* Unnamespaced::AsFFI() const {
  return reinterpret_cast<const ::capi::Unnamespaced*>(this);
}

inline ::capi::Unnamespaced* Unnamespaced::AsFFI() {
  return reinterpret_cast<::capi::Unnamespaced*>(this);
}

inline const Unnamespaced* Unnamespaced::FromFFI(const ::capi::Unnamespaced* ptr) {
  return reinterpret_cast<const Unnamespaced*>(ptr);
}

inline Unnamespaced* Unnamespaced::FromFFI(::capi::Unnamespaced* ptr) {
  return reinterpret_cast<Unnamespaced*>(ptr);
}

inline void Unnamespaced::operator delete(void* ptr) {
  capi::namespace_Unnamespaced_destroy(reinterpret_cast<::capi::Unnamespaced*>(ptr));
}


#endif // Unnamespaced_HPP
