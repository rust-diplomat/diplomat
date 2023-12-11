#ifndef Float64Vec_HPP
#define Float64Vec_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Float64Vec.h"

class Float64Vec;

/**
 * A destruction policy for using Float64Vec with std::unique_ptr.
 */
struct Float64VecDeleter {
  void operator()(capi::Float64Vec* l) const noexcept {
    capi::Float64Vec_destroy(l);
  }
};
class Float64Vec {
 public:
  static Float64Vec new_(const diplomat::span<const double> v);
  void fill_slice(diplomat::span<const double> v) const;
  void set_value(const diplomat::span<const double> new_slice);
  inline const capi::Float64Vec* AsFFI() const { return this->inner.get(); }
  inline capi::Float64Vec* AsFFIMut() { return this->inner.get(); }
  inline explicit Float64Vec(capi::Float64Vec* i) : inner(i) {}
  Float64Vec() = default;
  Float64Vec(Float64Vec&&) noexcept = default;
  Float64Vec& operator=(Float64Vec&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Float64Vec, Float64VecDeleter> inner;
};


inline Float64Vec Float64Vec::new_(const diplomat::span<const double> v) {
  return Float64Vec(capi::Float64Vec_new(v.data(), v.size()));
}
inline void Float64Vec::fill_slice(diplomat::span<const double> v) const {
  capi::Float64Vec_fill_slice(this->inner.get(), v.data(), v.size());
}
inline void Float64Vec::set_value(const diplomat::span<const double> new_slice) {
  capi::Float64Vec_set_value(this->inner.get(), new_slice.data(), new_slice.size());
}
#endif
