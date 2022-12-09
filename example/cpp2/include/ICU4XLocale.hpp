#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XLocale.d.hpp"
#include "ICU4XLocale.h"




inline std::unique_ptr<ICU4XLocale> ICU4XLocale::new_(std::string_view name) {
  // TODO
}
inline std::unique_ptr<ICU4XLocale> ICU4XLocale::new_from_bytes(std::span<const uint8_t> bytes) {
  // TODO
}
inline const capi::ICU4XLocale* ICU4XLocale::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocale*>(this);
}
inline capi::ICU4XLocale* ICU4XLocale::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocale*>(this);
}
inline ICU4XLocale::~ICU4XLocale() {
  capi::ICU4XLocale_destroy(AsFFI());
}


#endif // ICU4XLocale_HPP
