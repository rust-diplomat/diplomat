#ifndef OptionOpaque_HPP
#define OptionOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "OptionOpaque.d.hpp"
#include "OptionOpaque.h"





inline std::unique_ptr<OptionOpaque> OptionOpaque::new_(int32_t i) {
  // TODO
}

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_none() {
  // TODO
}

inline OptionStruct OptionOpaque::new_struct() {
  // TODO
}

inline OptionStruct OptionOpaque::new_struct_nones() {
  // TODO
}

inline void OptionOpaque::assert_integer(int32_t i) const {
  // TODO
}

inline bool OptionOpaque::option_opaque_argument(std::optional<const OptionOpaque&> arg) {
  // TODO
}

inline const capi::OptionOpaque* OptionOpaque::AsFFI() const {
  return reinterpret_cast<const capi::OptionOpaque*>(this);
}
inline capi::OptionOpaque* OptionOpaque::AsFFI() {
  return reinterpret_cast<capi::OptionOpaque*>(this);
}
inline OptionOpaque::~OptionOpaque() {
  capi::OptionOpaque_destroy(AsFFI());
}


#endif // OptionOpaque_HPP
