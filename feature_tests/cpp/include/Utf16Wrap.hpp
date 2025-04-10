#ifndef Utf16Wrap_HPP
#define Utf16Wrap_HPP

#include "Utf16Wrap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::Utf16Wrap* Utf16Wrap_from_utf16(diplomat::capi::DiplomatString16View input);
    
    void Utf16Wrap_get_debug_str(const diplomat::capi::Utf16Wrap* self, diplomat::capi::DiplomatWrite* write);
    
    diplomat::capi::DiplomatString16View Utf16Wrap_borrow_cont(const diplomat::capi::Utf16Wrap* self);
    
    
    void Utf16Wrap_destroy(Utf16Wrap* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Utf16Wrap> Utf16Wrap::from_utf16(std::u16string_view input) {
  auto result = diplomat::capi::Utf16Wrap_from_utf16({input.data(), input.size()});
  return std::unique_ptr<Utf16Wrap>(Utf16Wrap::FromFFI(result));
}

inline std::string Utf16Wrap::get_debug_str() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Utf16Wrap_get_debug_str(this->AsFFI(),
    &write);
  return output;
}

inline std::u16string_view Utf16Wrap::borrow_cont() const {
  auto result = diplomat::capi::Utf16Wrap_borrow_cont(this->AsFFI());
  return std::u16string_view(result.data, result.len);
}

inline const diplomat::capi::Utf16Wrap* Utf16Wrap::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Utf16Wrap*>(this);
}

inline diplomat::capi::Utf16Wrap* Utf16Wrap::AsFFI() {
  return reinterpret_cast<diplomat::capi::Utf16Wrap*>(this);
}

inline const Utf16Wrap* Utf16Wrap::FromFFI(const diplomat::capi::Utf16Wrap* ptr) {
  return reinterpret_cast<const Utf16Wrap*>(ptr);
}

inline Utf16Wrap* Utf16Wrap::FromFFI(diplomat::capi::Utf16Wrap* ptr) {
  return reinterpret_cast<Utf16Wrap*>(ptr);
}

inline void Utf16Wrap::operator delete(void* ptr) {
  diplomat::capi::Utf16Wrap_destroy(reinterpret_cast<diplomat::capi::Utf16Wrap*>(ptr));
}


#endif // Utf16Wrap_HPP
