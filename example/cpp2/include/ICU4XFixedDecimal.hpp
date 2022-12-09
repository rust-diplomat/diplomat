#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimal.d.hpp"
#include "ICU4XFixedDecimal.h"




inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::new_(int32_t v) {
  // TODO
}
inline void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  // TODO
}
inline void ICU4XFixedDecimal::negate() {
  // TODO
}
inline DiplomatResult<std::string, std::monostate> ICU4XFixedDecimal::to_string() const {
  // TODO
}
inline const capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimal*>(this);
}
inline capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimal*>(this);
}
inline ICU4XFixedDecimal::~ICU4XFixedDecimal() {
  capi::ICU4XFixedDecimal_destroy(AsFFI());
}


#endif // ICU4XFixedDecimal_HPP
