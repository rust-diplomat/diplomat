#ifndef ns_RenamedOpaqueArithmetic_HPP
#define ns_RenamedOpaqueArithmetic_HPP

#include "RenamedOpaqueArithmetic.d.hpp"

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
    
    ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_make(int32_t x, int32_t y);
    
    int32_t namespace_OpaqueArithmetic_x(const ns::capi::RenamedOpaqueArithmetic* self);
    
    int32_t namespace_OpaqueArithmetic_y(const ns::capi::RenamedOpaqueArithmetic* self);
    
    ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_add(const ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_sub(const ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_mul(const ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_div(const ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    void namespace_OpaqueArithmetic_addassign(ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    void namespace_OpaqueArithmetic_subassign(ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    void namespace_OpaqueArithmetic_mulassign(ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    void namespace_OpaqueArithmetic_divassign(ns::capi::RenamedOpaqueArithmetic* self, const ns::capi::RenamedOpaqueArithmetic* o);
    
    
    void namespace_OpaqueArithmetic_destroy(RenamedOpaqueArithmetic* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedOpaqueArithmetic> ns::RenamedOpaqueArithmetic::make(int32_t x, int32_t y) {
  auto result = ns::capi::namespace_OpaqueArithmetic_make(x,
    y);
  return std::unique_ptr<ns::RenamedOpaqueArithmetic>(ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline int32_t ns::RenamedOpaqueArithmetic::x() const {
  auto result = ns::capi::namespace_OpaqueArithmetic_x(this->AsFFI());
  return result;
}

inline int32_t ns::RenamedOpaqueArithmetic::y() const {
  auto result = ns::capi::namespace_OpaqueArithmetic_y(this->AsFFI());
  return result;
}

inline std::unique_ptr<ns::RenamedOpaqueArithmetic> ns::RenamedOpaqueArithmetic::operator+(const ns::RenamedOpaqueArithmetic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmetic_add(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmetic>(ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmetic> ns::RenamedOpaqueArithmetic::operator-(const ns::RenamedOpaqueArithmetic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmetic_sub(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmetic>(ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmetic> ns::RenamedOpaqueArithmetic::operator*(const ns::RenamedOpaqueArithmetic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmetic_mul(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmetic>(ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<ns::RenamedOpaqueArithmetic> ns::RenamedOpaqueArithmetic::operator/(const ns::RenamedOpaqueArithmetic& o) const {
  auto result = ns::capi::namespace_OpaqueArithmetic_div(this->AsFFI(),
    o.AsFFI());
  return std::unique_ptr<ns::RenamedOpaqueArithmetic>(ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline void ns::RenamedOpaqueArithmetic::operator+=(const ns::RenamedOpaqueArithmetic& o) {
  ns::capi::namespace_OpaqueArithmetic_addassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmetic::operator-=(const ns::RenamedOpaqueArithmetic& o) {
  ns::capi::namespace_OpaqueArithmetic_subassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmetic::operator*=(const ns::RenamedOpaqueArithmetic& o) {
  ns::capi::namespace_OpaqueArithmetic_mulassign(this->AsFFI(),
    o.AsFFI());
}

inline void ns::RenamedOpaqueArithmetic::operator/=(const ns::RenamedOpaqueArithmetic& o) {
  ns::capi::namespace_OpaqueArithmetic_divassign(this->AsFFI(),
    o.AsFFI());
}

inline const ns::capi::RenamedOpaqueArithmetic* ns::RenamedOpaqueArithmetic::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedOpaqueArithmetic*>(this);
}

inline ns::capi::RenamedOpaqueArithmetic* ns::RenamedOpaqueArithmetic::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedOpaqueArithmetic*>(this);
}

inline const ns::RenamedOpaqueArithmetic* ns::RenamedOpaqueArithmetic::FromFFI(const ns::capi::RenamedOpaqueArithmetic* ptr) {
  return reinterpret_cast<const ns::RenamedOpaqueArithmetic*>(ptr);
}

inline ns::RenamedOpaqueArithmetic* ns::RenamedOpaqueArithmetic::FromFFI(ns::capi::RenamedOpaqueArithmetic* ptr) {
  return reinterpret_cast<ns::RenamedOpaqueArithmetic*>(ptr);
}

inline void ns::RenamedOpaqueArithmetic::operator delete(void* ptr) {
  ns::capi::namespace_OpaqueArithmetic_destroy(reinterpret_cast<ns::capi::RenamedOpaqueArithmetic*>(ptr));
}


#endif // ns_RenamedOpaqueArithmetic_HPP
