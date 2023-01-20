#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.h"

#include "ICU4XDataProvider.d.hpp"


inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::new_static() {
  auto result = capi::ICU4XDataProvider_new_static();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline DiplomatResult<std::monostate, std::monostate> ICU4XDataProvider::returns_result() {
  capi::ICU4XDataProvider_returns_result();
}

inline const capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDataProvider*>(this);
}

inline capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() {
  return reinterpret_cast<capi::ICU4XDataProvider*>(this);
}

inline const ICU4XDataProvider* ICU4XDataProvider::FromFFI(const capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<const ICU4XDataProvider*>(ptr);
}

inline ICU4XDataProvider* ICU4XDataProvider::FromFFI(capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<ICU4XDataProvider*>(ptr);
}

inline ICU4XDataProvider::~ICU4XDataProvider() {
  capi::ICU4XDataProvider_destroy(AsFFI());
}


#endif // ICU4XDataProvider_HPP
