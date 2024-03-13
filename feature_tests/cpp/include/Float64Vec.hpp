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
  static Float64Vec new_bool(const diplomat::span<const bool> v);
  static Float64Vec new_i16(const diplomat::span<const int16_t> v);
  static Float64Vec new_u16(const diplomat::span<const uint16_t> v);
  static Float64Vec new_isize(const diplomat::span<const intptr_t> v);
  static Float64Vec new_usize(const diplomat::span<const size_t> v);
  static Float64Vec new_f64_be_bytes(const diplomat::span<const uint8_t> v);
  static Float64Vec new_from_owned(const diplomat::span<double> v);
  const diplomat::span<double> as_boxed_slice() const;

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  const diplomat::span<const double> as_slice() const;
  void fill_slice(const diplomat::span<double> v) const;
  void set_value(const diplomat::span<const double> new_slice);
  template<typename W> void to_string_to_writeable(W& w) const;
  std::string to_string() const;

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  const diplomat::span<const double> borrow() const;
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
inline Float64Vec Float64Vec::new_bool(const diplomat::span<const bool> v) {
  return Float64Vec(capi::Float64Vec_new_bool(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_i16(const diplomat::span<const int16_t> v) {
  return Float64Vec(capi::Float64Vec_new_i16(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_u16(const diplomat::span<const uint16_t> v) {
  return Float64Vec(capi::Float64Vec_new_u16(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_isize(const diplomat::span<const intptr_t> v) {
  return Float64Vec(capi::Float64Vec_new_isize(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_usize(const diplomat::span<const size_t> v) {
  return Float64Vec(capi::Float64Vec_new_usize(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_f64_be_bytes(const diplomat::span<const uint8_t> v) {
  return Float64Vec(capi::Float64Vec_new_f64_be_bytes(v.data(), v.size()));
}
inline Float64Vec Float64Vec::new_from_owned(const diplomat::span<double> v) {
  return Float64Vec(capi::Float64Vec_new_from_owned(v.data(), v.size()));
}
inline const diplomat::span<double> Float64Vec::as_boxed_slice() const {
  capi::DiplomatF64View diplomat_slice_raw_out_value = capi::Float64Vec_as_boxed_slice(this->inner.get());
  diplomat::span<mut double> slice(diplomat_slice_raw_out_value.data, diplomat_slice_raw_out_value.len);
  return slice;
}
inline const diplomat::span<const double> Float64Vec::as_slice() const {
  capi::DiplomatF64View diplomat_slice_raw_out_value = capi::Float64Vec_as_slice(this->inner.get());
  diplomat::span<const double> slice(diplomat_slice_raw_out_value.data, diplomat_slice_raw_out_value.len);
  return slice;
}
inline void Float64Vec::fill_slice(const diplomat::span<double> v) const {
  capi::Float64Vec_fill_slice(this->inner.get(), v.data(), v.size());
}
inline void Float64Vec::set_value(const diplomat::span<const double> new_slice) {
  capi::Float64Vec_set_value(this->inner.get(), new_slice.data(), new_slice.size());
}
template<typename W> inline void Float64Vec::to_string_to_writeable(W& w) const {
  capi::DiplomatWriteable w_writer = diplomat::WriteableTrait<W>::Construct(w);
  capi::Float64Vec_to_string(this->inner.get(), &w_writer);
}
inline std::string Float64Vec::to_string() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::Float64Vec_to_string(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
inline const diplomat::span<const double> Float64Vec::borrow() const {
  capi::DiplomatF64View diplomat_slice_raw_out_value = capi::Float64Vec_borrow(this->inner.get());
  diplomat::span<const double> slice(diplomat_slice_raw_out_value.data, diplomat_slice_raw_out_value.len);
  return slice;
}
#endif
