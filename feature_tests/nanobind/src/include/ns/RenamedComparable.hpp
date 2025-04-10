#ifndef ns_RenamedComparable_HPP
#define ns_RenamedComparable_HPP

#include "RenamedComparable.d.hpp"

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
    
    ns::capi::RenamedComparable* namespace_Comparable_new(uint8_t int_);
    
    int8_t namespace_Comparable_cmp(const ns::capi::RenamedComparable* self, const ns::capi::RenamedComparable* other);
    
    
    void namespace_Comparable_destroy(RenamedComparable* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::RenamedComparable> ns::RenamedComparable::new_(uint8_t int_) {
  auto result = ns::capi::namespace_Comparable_new(int_);
  return std::unique_ptr<ns::RenamedComparable>(ns::RenamedComparable::FromFFI(result));
}

inline int8_t ns::RenamedComparable::cmp(const ns::RenamedComparable& other) const {
  auto result = ns::capi::namespace_Comparable_cmp(this->AsFFI(),
    other.AsFFI());
  return result;
}
inline bool ns::RenamedComparable::operator==(const ns::RenamedComparable& other) const {
  return this->cmp(other) == 0;
}

inline bool ns::RenamedComparable::operator!=(const ns::RenamedComparable& other) const {
  return this->cmp(other) != 0;
}

inline bool ns::RenamedComparable::operator<=(const ns::RenamedComparable& other) const {
  return this->cmp(other) <= 0;
}

inline bool ns::RenamedComparable::operator>=(const ns::RenamedComparable& other) const {
  return this->cmp(other) >= 0;
}

inline bool ns::RenamedComparable::operator<(const ns::RenamedComparable& other) const {
  return this->cmp(other) < 0;
}

inline bool ns::RenamedComparable::operator>(const ns::RenamedComparable& other) const {
  return this->cmp(other) > 0;
}

inline const ns::capi::RenamedComparable* ns::RenamedComparable::AsFFI() const {
  return reinterpret_cast<const ns::capi::RenamedComparable*>(this);
}

inline ns::capi::RenamedComparable* ns::RenamedComparable::AsFFI() {
  return reinterpret_cast<ns::capi::RenamedComparable*>(this);
}

inline const ns::RenamedComparable* ns::RenamedComparable::FromFFI(const ns::capi::RenamedComparable* ptr) {
  return reinterpret_cast<const ns::RenamedComparable*>(ptr);
}

inline ns::RenamedComparable* ns::RenamedComparable::FromFFI(ns::capi::RenamedComparable* ptr) {
  return reinterpret_cast<ns::RenamedComparable*>(ptr);
}

inline void ns::RenamedComparable::operator delete(void* ptr) {
  ns::capi::namespace_Comparable_destroy(reinterpret_cast<ns::capi::RenamedComparable*>(ptr));
}


#endif // ns_RenamedComparable_HPP
