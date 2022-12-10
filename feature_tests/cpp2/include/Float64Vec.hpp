#ifndef Float64Vec_HPP
#define Float64Vec_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Float64Vec.d.hpp"
#include "Float64Vec.h"





inline std::unique_ptr<Float64Vec> Float64Vec::new_(std::span<const double> v) {
  auto result = capi::Float64Vec_new(v.data(),
    v.size());
  return std::unique_ptr(Float64Vec::FromFFI(result));
}

inline void Float64Vec::fill_slice(std::span<double> v) const {
  capi::Float64Vec_fill_slice(this->AsFFI(),
    v.data(),
    v.size());
}

inline void Float64Vec::set_value(std::span<const double> new_slice) {
  capi::Float64Vec_set_value(this->AsFFI(),
    new_slice.data(),
    new_slice.size());
}

inline const capi::Float64Vec* Float64Vec::AsFFI() const {
  return reinterpret_cast<const capi::Float64Vec*>(this);
}
inline capi::Float64Vec* Float64Vec::AsFFI() {
  return reinterpret_cast<capi::Float64Vec*>(this);
}
inline const Float64Vec* Float64Vec::FromFFI(const capi::Float64Vec* ptr) {
  return reinterpret_cast<const Float64Vec*>(ptr);
}
inline Float64Vec* Float64Vec::FromFFI(capi::Float64Vec* ptr) {
  return reinterpret_cast<Float64Vec*>(ptr);
}
inline Float64Vec::~Float64Vec() {
  capi::Float64Vec_destroy(AsFFI());
}


#endif // Float64Vec_HPP
