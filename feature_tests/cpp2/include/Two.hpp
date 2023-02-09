#ifndef Two_HPP
#define Two_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Two.h"

#include "Two.d.hpp"


inline const capi::Two* Two::AsFFI() const {
  return reinterpret_cast<const capi::Two*>(this);
}

inline capi::Two* Two::AsFFI() {
  return reinterpret_cast<capi::Two*>(this);
}

inline const Two* Two::FromFFI(const capi::Two* ptr) {
  return reinterpret_cast<const Two*>(ptr);
}

inline Two* Two::FromFFI(capi::Two* ptr) {
  return reinterpret_cast<Two*>(ptr);
}

inline void Two::operator delete(void* ptr) {
  capi::Two_destroy(reinterpret_cast<capi::Two*>(ptr));
}


#endif // Two_HPP
