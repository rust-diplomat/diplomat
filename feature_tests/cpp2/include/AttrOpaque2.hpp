#ifndef AttrOpaque2_HPP
#define AttrOpaque2_HPP

#include "AttrOpaque2.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque2.h"


inline const capi::AttrOpaque2* AttrOpaque2::AsFFI() const {
  return reinterpret_cast<const capi::AttrOpaque2*>(this);
}

inline capi::AttrOpaque2* AttrOpaque2::AsFFI() {
  return reinterpret_cast<capi::AttrOpaque2*>(this);
}

inline const AttrOpaque2* AttrOpaque2::FromFFI(const capi::AttrOpaque2* ptr) {
  return reinterpret_cast<const AttrOpaque2*>(ptr);
}

inline AttrOpaque2* AttrOpaque2::FromFFI(capi::AttrOpaque2* ptr) {
  return reinterpret_cast<AttrOpaque2*>(ptr);
}

inline void AttrOpaque2::operator delete(void* ptr) {
  capi::AttrOpaque2_destroy(reinterpret_cast<capi::AttrOpaque2*>(ptr));
}


#endif // AttrOpaque2_HPP
