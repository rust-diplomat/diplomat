#ifndef Opaque_HPP
#define Opaque_HPP

#include "Opaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "ImportedStruct.hpp"
#include "MyStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::Opaque* Opaque_new(void);
    
    diplomat::capi::Opaque* Opaque_try_from_utf8(diplomat::capi::DiplomatStringView input);
    
    diplomat::capi::Opaque* Opaque_from_str(diplomat::capi::DiplomatStringView input);
    
    void Opaque_get_debug_str(const diplomat::capi::Opaque* self, diplomat::capi::DiplomatWrite* write);
    
    void Opaque_assert_struct(const diplomat::capi::Opaque* self, diplomat::capi::MyStruct s);
    
    size_t Opaque_returns_usize(void);
    
    diplomat::capi::ImportedStruct Opaque_returns_imported(void);
    
    int8_t Opaque_cmp(void);
    
    
    void Opaque_destroy(Opaque* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Opaque> Opaque::new_() {
  auto result = diplomat::capi::Opaque_new();
  return std::unique_ptr<Opaque>(Opaque::FromFFI(result));
}

inline std::unique_ptr<Opaque> Opaque::try_from_utf8(std::string_view input) {
  auto result = diplomat::capi::Opaque_try_from_utf8({input.data(), input.size()});
  return std::unique_ptr<Opaque>(Opaque::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<Opaque>, diplomat::Utf8Error> Opaque::from_str(std::string_view input) {
  if (!diplomat::capi::diplomat_is_str(input.data(), input.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::Opaque_from_str({input.data(), input.size()});
  return diplomat::Ok<std::unique_ptr<Opaque>>(std::unique_ptr<Opaque>(Opaque::FromFFI(result)));
}

inline std::string Opaque::get_debug_str() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Opaque_get_debug_str(this->AsFFI(),
    &write);
  return output;
}

inline void Opaque::assert_struct(MyStruct s) const {
  diplomat::capi::Opaque_assert_struct(this->AsFFI(),
    s.AsFFI());
}

inline size_t Opaque::returns_usize() {
  auto result = diplomat::capi::Opaque_returns_usize();
  return result;
}

inline ImportedStruct Opaque::returns_imported() {
  auto result = diplomat::capi::Opaque_returns_imported();
  return ImportedStruct::FromFFI(result);
}

inline int8_t Opaque::cmp() {
  auto result = diplomat::capi::Opaque_cmp();
  return result;
}

inline const diplomat::capi::Opaque* Opaque::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Opaque*>(this);
}

inline diplomat::capi::Opaque* Opaque::AsFFI() {
  return reinterpret_cast<diplomat::capi::Opaque*>(this);
}

inline const Opaque* Opaque::FromFFI(const diplomat::capi::Opaque* ptr) {
  return reinterpret_cast<const Opaque*>(ptr);
}

inline Opaque* Opaque::FromFFI(diplomat::capi::Opaque* ptr) {
  return reinterpret_cast<Opaque*>(ptr);
}

inline void Opaque::operator delete(void* ptr) {
  diplomat::capi::Opaque_destroy(reinterpret_cast<diplomat::capi::Opaque*>(ptr));
}


#endif // Opaque_HPP
