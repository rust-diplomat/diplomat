#ifndef AttrOpaque1_HPP
#define AttrOpaque1_HPP

#include "AttrOpaque1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque1.h"


inline void AttrOpaque1::method() const {
  capi::AttrOpaque1_method(this->AsFFI());
}

inline const capi::AttrOpaque1* AttrOpaque1::AsFFI() const {
  return reinterpret_cast<const capi::AttrOpaque1*>(this);
}

inline capi::AttrOpaque1* AttrOpaque1::AsFFI() {
  return reinterpret_cast<capi::AttrOpaque1*>(this);
}

inline const AttrOpaque1* AttrOpaque1::FromFFI(const capi::AttrOpaque1* ptr) {
  return reinterpret_cast<const AttrOpaque1*>(ptr);
}

inline AttrOpaque1* AttrOpaque1::FromFFI(capi::AttrOpaque1* ptr) {
  return reinterpret_cast<AttrOpaque1*>(ptr);
}

inline void AttrOpaque1::operator delete(void* ptr) {
  capi::AttrOpaque1_destroy(reinterpret_cast<capi::AttrOpaque1*>(ptr));
}


#endif // AttrOpaque1_HPP
