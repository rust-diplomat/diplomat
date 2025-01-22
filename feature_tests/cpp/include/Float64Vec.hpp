#ifndef Float64Vec_HPP
#define Float64Vec_HPP

#include "Float64Vec.d.hpp"

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
    
    diplomat::capi::Float64Vec* Float64Vec_new(diplomat::capi::DiplomatF64View v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_bool(diplomat::capi::DiplomatBoolView v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_i16(diplomat::capi::DiplomatI16View v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_u16(diplomat::capi::DiplomatU16View v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_isize(diplomat::capi::DiplomatIsizeView v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_usize(diplomat::capi::DiplomatUsizeView v);
    
    diplomat::capi::Float64Vec* Float64Vec_new_f64_be_bytes(diplomat::capi::DiplomatU8View v);
    
    diplomat::capi::DiplomatF64View Float64Vec_as_slice(const diplomat::capi::Float64Vec* self);
    
    void Float64Vec_fill_slice(const diplomat::capi::Float64Vec* self, diplomat::capi::DiplomatF64ViewMut v);
    
    void Float64Vec_set_value(diplomat::capi::Float64Vec* self, diplomat::capi::DiplomatF64View new_slice);
    
    void Float64Vec_to_string(const diplomat::capi::Float64Vec* self, diplomat::capi::DiplomatWrite* write);
    
    diplomat::capi::DiplomatF64View Float64Vec_borrow(const diplomat::capi::Float64Vec* self);
    
    typedef struct Float64Vec_get_result {union {double ok; }; bool is_ok;} Float64Vec_get_result;
    Float64Vec_get_result Float64Vec_get(const diplomat::capi::Float64Vec* self, size_t i);
    
    
    void Float64Vec_destroy(Float64Vec* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Float64Vec> Float64Vec::new_(diplomat::span<const double> v) {
  auto result = diplomat::capi::Float64Vec_new({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_bool(diplomat::span<const bool> v) {
  auto result = diplomat::capi::Float64Vec_new_bool({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_i16(diplomat::span<const int16_t> v) {
  auto result = diplomat::capi::Float64Vec_new_i16({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_u16(diplomat::span<const uint16_t> v) {
  auto result = diplomat::capi::Float64Vec_new_u16({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_isize(diplomat::span<const intptr_t> v) {
  auto result = diplomat::capi::Float64Vec_new_isize({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_usize(diplomat::span<const size_t> v) {
  auto result = diplomat::capi::Float64Vec_new_usize({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline std::unique_ptr<Float64Vec> Float64Vec::new_f64_be_bytes(diplomat::span<const uint8_t> v) {
  auto result = diplomat::capi::Float64Vec_new_f64_be_bytes({v.data(), v.size()});
  return std::unique_ptr<Float64Vec>(Float64Vec::FromFFI(result));
}

inline diplomat::span<const double> Float64Vec::as_slice() const {
  auto result = diplomat::capi::Float64Vec_as_slice(this->AsFFI());
  return diplomat::span<const double>(result.data, result.len);
}

inline void Float64Vec::fill_slice(diplomat::span<double> v) const {
  diplomat::capi::Float64Vec_fill_slice(this->AsFFI(),
    {v.data(), v.size()});
}

inline void Float64Vec::set_value(diplomat::span<const double> new_slice) {
  diplomat::capi::Float64Vec_set_value(this->AsFFI(),
    {new_slice.data(), new_slice.size()});
}

inline std::string Float64Vec::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Float64Vec_to_string(this->AsFFI(),
    &write);
  return output;
}

inline diplomat::span<const double> Float64Vec::borrow() const {
  auto result = diplomat::capi::Float64Vec_borrow(this->AsFFI());
  return diplomat::span<const double>(result.data, result.len);
}

inline std::optional<double> Float64Vec::get(size_t i) const {
  auto result = diplomat::capi::Float64Vec_get(this->AsFFI(),
    i);
  return result.is_ok ? std::optional<double>(result.ok) : std::nullopt;
}

inline const diplomat::capi::Float64Vec* Float64Vec::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Float64Vec*>(this);
}

inline diplomat::capi::Float64Vec* Float64Vec::AsFFI() {
  return reinterpret_cast<diplomat::capi::Float64Vec*>(this);
}

inline const Float64Vec* Float64Vec::FromFFI(const diplomat::capi::Float64Vec* ptr) {
  return reinterpret_cast<const Float64Vec*>(ptr);
}

inline Float64Vec* Float64Vec::FromFFI(diplomat::capi::Float64Vec* ptr) {
  return reinterpret_cast<Float64Vec*>(ptr);
}

inline void Float64Vec::operator delete(void* ptr) {
  diplomat::capi::Float64Vec_destroy(reinterpret_cast<diplomat::capi::Float64Vec*>(ptr));
}


#endif // Float64Vec_HPP
