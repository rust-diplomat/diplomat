#ifndef Opaque_HPP
#define Opaque_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Opaque.h"

#include "Opaque.d.hpp"


inline std::unique_ptr<Opaque> Opaque::new_() {
  auto result = capi::Opaque_new();
  return std::unique_ptr<Opaque>(Opaque::FromFFI(result));
}

inline void Opaque::assert_struct(MyStruct s) const {
  capi::Opaque_assert_struct(this->AsFFI(),
    s.AsFFI());
}

inline size_t Opaque::returns_usize() {
  auto result = capi::Opaque_returns_usize();
  return result;
}

inline ImportedStruct Opaque::returns_imported() {
  auto result = capi::Opaque_returns_imported();
  return ImportedStruct::FromFFI(result);
}

inline const capi::Opaque* Opaque::AsFFI() const {
  return reinterpret_cast<const capi::Opaque*>(this);
}

inline capi::Opaque* Opaque::AsFFI() {
  return reinterpret_cast<capi::Opaque*>(this);
}

inline const Opaque* Opaque::FromFFI(const capi::Opaque* ptr) {
  return reinterpret_cast<const Opaque*>(ptr);
}

inline Opaque* Opaque::FromFFI(capi::Opaque* ptr) {
  return reinterpret_cast<Opaque*>(ptr);
}

inline Opaque::~Opaque() {
  capi::Opaque_destroy(AsFFI());
}


#endif // Opaque_HPP
