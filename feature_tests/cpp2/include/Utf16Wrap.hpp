#ifndef Utf16Wrap_HPP
#define Utf16Wrap_HPP

#include "Utf16Wrap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Utf16Wrap.h"


inline std::u16string_view Utf16Wrap::borrow_cont() const {
  auto result = capi::Utf16Wrap_borrow_cont(this->AsFFI());
  return std::u16string_view(result_data, result_size);
}

inline std::u16string_view Utf16Wrap::owned() const {
  auto result = capi::Utf16Wrap_owned(this->AsFFI());
  return std::u16string_view(result_data, result_size);
}

inline const capi::Utf16Wrap* Utf16Wrap::AsFFI() const {
  return reinterpret_cast<const capi::Utf16Wrap*>(this);
}

inline capi::Utf16Wrap* Utf16Wrap::AsFFI() {
  return reinterpret_cast<capi::Utf16Wrap*>(this);
}

inline const Utf16Wrap* Utf16Wrap::FromFFI(const capi::Utf16Wrap* ptr) {
  return reinterpret_cast<const Utf16Wrap*>(ptr);
}

inline Utf16Wrap* Utf16Wrap::FromFFI(capi::Utf16Wrap* ptr) {
  return reinterpret_cast<Utf16Wrap*>(ptr);
}

inline void Utf16Wrap::operator delete(void* ptr) {
  capi::Utf16Wrap_destroy(reinterpret_cast<capi::Utf16Wrap*>(ptr));
}


#endif // Utf16Wrap_HPP
