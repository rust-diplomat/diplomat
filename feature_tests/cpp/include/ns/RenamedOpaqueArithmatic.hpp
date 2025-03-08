#ifndef ns_RenamedOpaqueArithmatic_HPP
#define ns_RenamedOpaqueArithmatic_HPP

#include "RenamedOpaqueArithmatic.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {
    extern "C" {
    
    ns::capi::RenamedOpaqueArithmatic* namespace_OpaqueArithmatic_make(int32_t x, int32_t y);
    
    int32_t namespace_OpaqueArithmatic_x(const ns::capi::RenamedOpaqueArithmatic* self);
    
    int32_t namespace_OpaqueArithmatic_y(const ns::capi::RenamedOpaqueArithmatic* self);
    
    ns::capi::RenamedOpaqueArithmatic* namespace_OpaqueArithmatic_add(const ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    ns::capi::RenamedOpaqueArithmatic* namespace_OpaqueArithmatic_sub(const ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    ns::capi::RenamedOpaqueArithmatic* namespace_OpaqueArithmatic_mul(const ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    ns::capi::RenamedOpaqueArithmatic* namespace_OpaqueArithmatic_div(const ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    void namespace_OpaqueArithmatic_addassign(ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    void namespace_OpaqueArithmatic_subassign(ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    void namespace_OpaqueArithmatic_mulassign(ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    void namespace_OpaqueArithmatic_divassign(ns::capi::RenamedOpaqueArithmatic* self, const ns::capi::RenamedOpaqueArithmatic* o);
    
    
    void namespace_OpaqueArithmatic_destroy(RenamedOpaqueArithmatic* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedOpaqueArithmatic> ns::RenamedOpaqueArithmatic::make(int32_t x, int32_t y) {
  auto result = ns::capi::namespace_OpaqueArithmatic_make(x,
    y);
  return std::unique_ptr<ns::RenamedOpaqueArithmatic>(ns::RenamedOpaqueArithmatic::FromFFI(result));
}

inline int32_t ns::RenamedOpaqueArithmatic::x() const {
  auto result = ns::capi::namespace_OpaqueArithmatic_x(this->AsFFI());
  return result;
}

inline int32_t ns::RenamedOpaqueArithmatic::y() const {
  auto result = ns::capi::namespace_OpaqueArithmatic_y(this->AsFFI());
  return result;
}

inline std::unique_ptr<ns::RenamedOpaqueArithmatic> ns::RenamedOpaqueArithmatic::operator+(const ns::RenamedOpaqueArithmatic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmatic_add(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmatic>(ns::RenamedOpaqueArithmatic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmatic> ns::RenamedOpaqueArithmatic::operator-(const ns::RenamedOpaqueArithmatic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmatic_sub(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmatic>(ns::RenamedOpaqueArithmatic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmatic> ns::RenamedOpaqueArithmatic::operator*(const ns::RenamedOpaqueArithmatic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmatic_mul(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmatic>(ns::RenamedOpaqueArithmatic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmatic> ns::RenamedOpaqueArithmatic::operator/(const ns::RenamedOpaqueArithmatic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmatic_div(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmatic>(ns::RenamedOpaqueArithmatic::FromFFI(result));
}

inline void ns::RenamedOpaqueArithmatic::operator+=(const ns::RenamedOpaqueArithmatic& o) {
  ns::capi::namespace_OpaqueArithmatic_addassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmatic::operator-=(const ns::RenamedOpaqueArithmatic& o) {
  ns::capi::namespace_OpaqueArithmatic_subassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmatic::operator*=(const ns::RenamedOpaqueArithmatic& o) {
  ns::capi::namespace_OpaqueArithmatic_mulassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmatic::operator/=(const ns::RenamedOpaqueArithmatic& o) {
  ns::capi::namespace_OpaqueArithmatic_divassign(this->AsFFI(),
    o.AsFFI());
}

inline const ns::capi::RenamedOpaqueArithmatic* ns::RenamedOpaqueArithmatic::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedOpaqueArithmatic*>(this);
}

inline ns::capi::RenamedOpaqueArithmatic* ns::RenamedOpaqueArithmatic::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedOpaqueArithmatic*>(this);
}

inline const ns::RenamedOpaqueArithmatic* ns::RenamedOpaqueArithmatic::FromFFI(const ns::capi::RenamedOpaqueArithmatic* ptr) {
  return reinterpret_cast<const ns::RenamedOpaqueArithmatic*>(ptr);
}

inline ns::RenamedOpaqueArithmatic* ns::RenamedOpaqueArithmatic::FromFFI(ns::capi::RenamedOpaqueArithmatic* ptr) {
  return reinterpret_cast<ns::RenamedOpaqueArithmatic*>(ptr);
}

inline void ns::RenamedOpaqueArithmatic::operator delete(void* ptr) {
  ns::capi::namespace_OpaqueArithmatic_destroy(reinterpret_cast<ns::capi::RenamedOpaqueArithmatic*>(ptr));
}


#endif // ns_RenamedOpaqueArithmatic_HPP
