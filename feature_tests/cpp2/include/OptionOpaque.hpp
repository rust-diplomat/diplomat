#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "OptionOpaque.h"

#include "OptionOpaque.d.hpp"


inline std::unique_ptr<OptionOpaque> OptionOpaque::new_(int32_t i) {
  auto result = capi::OptionOpaque_new(i);
  return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_none() {
  auto result = capi::OptionOpaque_new_none();
  return std::unique_ptr<OptionOpaque>(OptionOpaque::FromFFI(result));
}

inline OptionStruct OptionOpaque::new_struct() {
  auto result = capi::OptionOpaque_new_struct();
  return OptionStruct::FromFFI(result);
}

inline OptionStruct OptionOpaque::new_struct_nones() {
  auto result = capi::OptionOpaque_new_struct_nones();
  return OptionStruct::FromFFI(result);
}

inline void OptionOpaque::assert_integer(int32_t i) const {
  capi::OptionOpaque_assert_integer(this->AsFFI(),
    i);
}

inline bool OptionOpaque::option_opaque_argument(const OptionOpaque* arg) {
  auto result = capi::OptionOpaque_option_opaque_argument(arg ? arg->AsFFI() : nullptr);
  return result;
}

inline const capi::OptionOpaque* OptionOpaque::AsFFI() const {
  return reinterpret_cast<const capi::OptionOpaque*>(this);
}

inline capi::OptionOpaque* OptionOpaque::AsFFI() {
  return reinterpret_cast<capi::OptionOpaque*>(this);
}

inline const OptionOpaque* OptionOpaque::FromFFI(const capi::OptionOpaque* ptr) {
  return reinterpret_cast<const OptionOpaque*>(ptr);
}

inline OptionOpaque* OptionOpaque::FromFFI(capi::OptionOpaque* ptr) {
  return reinterpret_cast<OptionOpaque*>(ptr);
}

inline OptionOpaque::~OptionOpaque() {
  capi::OptionOpaque_destroy(AsFFI());
}


#endif // OptionOpaque_HPP
