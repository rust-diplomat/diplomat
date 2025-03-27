#ifndef StructArithmetic_HPP
#define StructArithmetic_HPP

#include "StructArithmetic.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::StructArithmetic StructArithmetic_add(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_sub(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_mul(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_div(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline StructArithmetic StructArithmetic::operator+(StructArithmetic o) const {
  auto result = diplomat::capi::StructArithmetic_add(this->AsFFI(),
    o.AsFFI());
  return StructArithmetic::FromFFI(result);
}
inline StructArithmetic& StructArithmetic::operator+=(StructArithmetic o) {
  *this = *this + o;
  return *this;
}

inline StructArithmetic StructArithmetic::operator-(StructArithmetic o) const {
  auto result = diplomat::capi::StructArithmetic_sub(this->AsFFI(),
    o.AsFFI());
  return StructArithmetic::FromFFI(result);
}
inline StructArithmetic& StructArithmetic::operator-=(StructArithmetic o) {
  *this = *this - o;
  return *this;
}

inline StructArithmetic StructArithmetic::operator*(StructArithmetic o) const {
  auto result = diplomat::capi::StructArithmetic_mul(this->AsFFI(),
    o.AsFFI());
  return StructArithmetic::FromFFI(result);
}
inline StructArithmetic& StructArithmetic::operator*=(StructArithmetic o) {
  *this = *this * o;
  return *this;
}

inline StructArithmetic StructArithmetic::operator/(StructArithmetic o) const {
  auto result = diplomat::capi::StructArithmetic_div(this->AsFFI(),
    o.AsFFI());
  return StructArithmetic::FromFFI(result);
}
inline StructArithmetic& StructArithmetic::operator/=(StructArithmetic o) {
  *this = *this / o;
  return *this;
}


inline diplomat::capi::StructArithmetic StructArithmetic::AsFFI() const {
  return diplomat::capi::StructArithmetic {
    /* .x = */ x,
    /* .y = */ y,
  };
}

inline StructArithmetic StructArithmetic::FromFFI(diplomat::capi::StructArithmetic c_struct) {
  return StructArithmetic {
    /* .x = */ c_struct.x,
    /* .y = */ c_struct.y,
  };
}


#endif // StructArithmetic_HPP
