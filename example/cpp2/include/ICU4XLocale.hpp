#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocale.d.hpp"
#include "ICU4XLocale.h"


inline std::unique_ptr<ICU4XLocale> ICU4XLocale::new_(std::string_view name) {
  auto result = capi::ICU4XLocale_new(name.data(),
    name.size());
  return std::unique_ptr(ICU4XLocale::FromFFI(result));
}

inline std::unique_ptr<ICU4XLocale> ICU4XLocale::new_from_bytes(std::span<const uint8_t> bytes) {
  auto result = capi::ICU4XLocale_new_from_bytes(bytes.data(),
    bytes.size());
  return std::unique_ptr(ICU4XLocale::FromFFI(result));
}

inline const capi::ICU4XLocale* ICU4XLocale::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocale*>(this);
}

inline capi::ICU4XLocale* ICU4XLocale::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocale*>(this);
}

inline const ICU4XLocale* ICU4XLocale::FromFFI(const capi::ICU4XLocale* ptr) {
  return reinterpret_cast<const ICU4XLocale*>(ptr);
}

inline ICU4XLocale* ICU4XLocale::FromFFI(capi::ICU4XLocale* ptr) {
  return reinterpret_cast<ICU4XLocale*>(ptr);
}

inline ICU4XLocale::~ICU4XLocale() {
  capi::ICU4XLocale_destroy(AsFFI());
}


#endif // ICU4XLocale_HPP
