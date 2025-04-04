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
    
    diplomat::capi::StructArithmetic StructArithmetic_ORIGIN(void);
    
    void StructArithmetic_set_origin(diplomat::capi::StructArithmetic _new_origin);
    
    diplomat::capi::StructArithmetic StructArithmetic_new(int32_t x, int32_t y);
    
    diplomat::capi::StructArithmetic StructArithmetic_add(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_sub(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_mul(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    diplomat::capi::StructArithmetic StructArithmetic_div(diplomat::capi::StructArithmetic self, diplomat::capi::StructArithmetic o);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline StructArithmetic StructArithmetic::ORIGIN() {
  auto result = diplomat::capi::StructArithmetic_ORIGIN();
  return StructArithmetic::FromFFI(result);
}

inline void StructArithmetic::set_origin(StructArithmetic _new_origin) {
  diplomat::capi::StructArithmetic_set_origin(_new_origin.AsFFI());
}

inline StructArithmetic StructArithmetic::new_(int32_t x, int32_t y) {
  auto result = diplomat::capi::StructArithmetic_new(x,
    y);
  return StructArithmetic::FromFFI(result);
}

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
