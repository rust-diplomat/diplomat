#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP

#include "ICU4XDataProvider.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.h"


inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::new_static() {
  auto result = capi::ICU4XDataProvider_new_static();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline std::optional<std::monostate> ICU4XDataProvider::returns_result() {
  auto result = capi::ICU4XDataProvider_returns_result();
  return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
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

inline void ICU4XDataProvider::operator delete(void* ptr) {
  capi::ICU4XDataProvider_destroy(reinterpret_cast<capi::ICU4XDataProvider*>(ptr));
}


#endif // ICU4XDataProvider_HPP
