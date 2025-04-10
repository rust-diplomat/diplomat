#ifndef Unnamespaced_HPP
#define Unnamespaced_HPP

#include "Unnamespaced.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::Unnamespaced* namespace_Unnamespaced_make(ns::capi::RenamedAttrEnum _e);
    
    void namespace_Unnamespaced_use_namespaced(const diplomat::capi::Unnamespaced* self, const ns::capi::AttrOpaque1Renamed* _n);
    
    
    void namespace_Unnamespaced_destroy(Unnamespaced* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Unnamespaced> Unnamespaced::make(ns::RenamedAttrEnum _e) {
  auto result = diplomat::capi::namespace_Unnamespaced_make(_e.AsFFI());
  return std::unique_ptr<Unnamespaced>(Unnamespaced::FromFFI(result));
}

inline void Unnamespaced::use_namespaced(const ns::AttrOpaque1Renamed& _n) const {
  diplomat::capi::namespace_Unnamespaced_use_namespaced(this->AsFFI(),
    _n.AsFFI());
}

inline const diplomat::capi::Unnamespaced* Unnamespaced::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Unnamespaced*>(this);
}

inline diplomat::capi::Unnamespaced* Unnamespaced::AsFFI() {
  return reinterpret_cast<diplomat::capi::Unnamespaced*>(this);
}

inline const Unnamespaced* Unnamespaced::FromFFI(const diplomat::capi::Unnamespaced* ptr) {
  return reinterpret_cast<const Unnamespaced*>(ptr);
}

inline Unnamespaced* Unnamespaced::FromFFI(diplomat::capi::Unnamespaced* ptr) {
  return reinterpret_cast<Unnamespaced*>(ptr);
}

inline void Unnamespaced::operator delete(void* ptr) {
  diplomat::capi::namespace_Unnamespaced_destroy(reinterpret_cast<diplomat::capi::Unnamespaced*>(ptr));
}


#endif // Unnamespaced_HPP
