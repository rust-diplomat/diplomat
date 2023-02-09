#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP

#include "ICU4XFixedDecimal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimal.h"


inline std::unique_ptr<ICU4XFixedDecimal> ICU4XFixedDecimal::new_(int32_t v) {
  auto result = capi::ICU4XFixedDecimal_new(v);
  return std::unique_ptr<ICU4XFixedDecimal>(ICU4XFixedDecimal::FromFFI(result));
}

inline void ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::ICU4XFixedDecimal_multiply_pow10(this->AsFFI(),
    power);
}

inline diplomat::result<std::string, std::monostate> ICU4XFixedDecimal::to_string() const {
  std::string output;
  capi::DiplomatWriteable writeable = diplomat::WriteableFromString(output);
  auto result = capi::ICU4XFixedDecimal_to_string(this->AsFFI(),
    &writeable);
  return result.is_ok ? diplomat::result<std::string, std::monostate>(diplomat::Ok<std::string>(/* TODO: Writeable conversion */)) : diplomat::result<std::string, std::monostate>(diplomat::Err<std::monostate>());
}

inline const capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimal*>(this);
}

inline capi::ICU4XFixedDecimal* ICU4XFixedDecimal::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimal*>(this);
}

inline const ICU4XFixedDecimal* ICU4XFixedDecimal::FromFFI(const capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<const ICU4XFixedDecimal*>(ptr);
}

inline ICU4XFixedDecimal* ICU4XFixedDecimal::FromFFI(capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<ICU4XFixedDecimal*>(ptr);
}

inline void ICU4XFixedDecimal::operator delete(void* ptr) {
  capi::ICU4XFixedDecimal_destroy(reinterpret_cast<capi::ICU4XFixedDecimal*>(ptr));
}


#endif // ICU4XFixedDecimal_HPP
