#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.hpp"
#include "ICU4XDataProvider.h"





inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::new_static() {
  // TODO
}

inline DiplomatResult<std::monostate, std::monostate> ICU4XDataProvider::returns_result() {
  // TODO
}

inline const capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDataProvider*>(this);
}
inline capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() {
  return reinterpret_cast<capi::ICU4XDataProvider*>(this);
}
inline ICU4XDataProvider::~ICU4XDataProvider() {
  capi::ICU4XDataProvider_destroy(AsFFI());
}


#endif // ICU4XDataProvider_HPP
