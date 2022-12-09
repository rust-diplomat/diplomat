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
  capi::OptionOpaque_new(i);
  // TODO
}

inline std::unique_ptr<OptionOpaque> OptionOpaque::new_none() {
  capi::OptionOpaque_new_none();
  // TODO
}

inline OptionStruct OptionOpaque::new_struct() {
  capi::OptionOpaque_new_struct();
  // TODO
}

inline OptionStruct OptionOpaque::new_struct_nones() {
  capi::OptionOpaque_new_struct_nones();
  // TODO
}

inline void OptionOpaque::assert_integer(int32_t i) const {
  capi::OptionOpaque_assert_integer(this->AsFFI(),
    i);
  // TODO
}

inline bool OptionOpaque::option_opaque_argument(std::optional<const std::reference_wrapper<OptionOpaque>> arg) {
  capi::OptionOpaque_option_opaque_argument(arg ? arg.value().get().AsFFI() : nullptr);
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
