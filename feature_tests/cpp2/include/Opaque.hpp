#ifndef Opaque_HPP
#define Opaque_HPP

#include "Opaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ImportedStruct.hpp"
#include "MyStruct.hpp"
#include "Opaque.h"


inline std::unique_ptr<Opaque> Opaque::new_() {
  auto result = capi::Opaque_new();
  return std::unique_ptr<Opaque>(Opaque::FromFFI(result));
}

inline std::unique_ptr<Opaque> Opaque::try_from_utf8(std::string_view input) {
  auto result = capi::Opaque_try_from_utf8(input.data(),
    input.size());
  return std::unique_ptr<Opaque>(Opaque::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<Opaque>, diplomat::Utf8Error> Opaque::from_str(std::string_view input) {
  if (!capi::diplomat_is_str(input.data(), input.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::Opaque_from_str(input.data(),
    input.size());
  return diplomat::Ok<std::unique_ptr<Opaque>>(std::move(std::unique_ptr<Opaque>(Opaque::FromFFI(result))));
}

inline std::string Opaque::get_debug_str() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::Opaque_get_debug_str(this->AsFFI(),
    &write);
  return output;
}

inline void Opaque::assert_struct(MyStruct s) const {
  capi::Opaque_assert_struct(this->AsFFI(),
    s.AsFFI());
}

inline size_t Opaque::returns_usize() {
  auto result = capi::Opaque_returns_usize();
  return result;
}

inline ImportedStruct Opaque::returns_imported() {
  auto result = capi::Opaque_returns_imported();
  return ImportedStruct::FromFFI(result);
}

inline int8_t Opaque::cmp() {
  auto result = capi::Opaque_cmp();
  return result;
}

inline const capi::Opaque* Opaque::AsFFI() const {
  return reinterpret_cast<const capi::Opaque*>(this);
}

inline capi::Opaque* Opaque::AsFFI() {
  return reinterpret_cast<capi::Opaque*>(this);
}

inline const Opaque* Opaque::FromFFI(const capi::Opaque* ptr) {
  return reinterpret_cast<const Opaque*>(ptr);
}

inline Opaque* Opaque::FromFFI(capi::Opaque* ptr) {
  return reinterpret_cast<Opaque*>(ptr);
}

inline void Opaque::operator delete(void* ptr) {
  capi::Opaque_destroy(reinterpret_cast<capi::Opaque*>(ptr));
}


#endif // Opaque_HPP
