#ifndef AttrOpaque1Renamed_HPP
#define AttrOpaque1Renamed_HPP

#include "AttrOpaque1Renamed.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AttrOpaque1.h"


inline std::unique_ptr<AttrOpaque1Renamed> AttrOpaque1Renamed::totally_not_new() {
  auto result = capi::namespace_AttrOpaque1_new();
  return std::unique_ptr<AttrOpaque1Renamed>(AttrOpaque1Renamed::FromFFI(result));
}

inline uint8_t AttrOpaque1Renamed::method_renamed() const {
  auto result = capi::namespace_AttrOpaque1_method(this->AsFFI());
  return result;
}

inline uint8_t AttrOpaque1Renamed::abirenamed() const {
  auto result = capi::renamed_on_abi_only(this->AsFFI());
  return result;
}

inline const capi::AttrOpaque1* AttrOpaque1Renamed::AsFFI() const {
  return reinterpret_cast<const capi::AttrOpaque1*>(this);
}

inline capi::AttrOpaque1* AttrOpaque1Renamed::AsFFI() {
  return reinterpret_cast<capi::AttrOpaque1*>(this);
}

inline const AttrOpaque1Renamed* AttrOpaque1Renamed::FromFFI(const capi::AttrOpaque1* ptr) {
  return reinterpret_cast<const AttrOpaque1Renamed*>(ptr);
}

inline AttrOpaque1Renamed* AttrOpaque1Renamed::FromFFI(capi::AttrOpaque1* ptr) {
  return reinterpret_cast<AttrOpaque1Renamed*>(ptr);
}

inline void AttrOpaque1Renamed::operator delete(void* ptr) {
  capi::namespace_AttrOpaque1_destroy(reinterpret_cast<capi::AttrOpaque1*>(ptr));
}


#endif // AttrOpaque1Renamed_HPP
