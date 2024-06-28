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


namespace capi {
    extern "C" {
    
    Utf16Wrap* Utf16Wrap_from_utf16(const char16_t* input_data, size_t input_len);
    
    void Utf16Wrap_get_debug_str(const Utf16Wrap* self, DiplomatWrite* write);
    
    DiplomatString16View Utf16Wrap_borrow_cont(const Utf16Wrap* self);
    
    DiplomatString16View Utf16Wrap_owned(const Utf16Wrap* self);
    
    
    void Utf16Wrap_destroy(Utf16Wrap* self);
    
    } // extern "C"
}

inline std::unique_ptr<Utf16Wrap> Utf16Wrap::from_utf16(std::u16string_view input) {
  auto result = capi::Utf16Wrap_from_utf16(input.data(),
    input.size());
  return std::unique_ptr<Utf16Wrap>(Utf16Wrap::FromFFI(result));
}

inline std::string Utf16Wrap::get_debug_str() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::Utf16Wrap_get_debug_str(this->AsFFI(),
    &write);
  return output;
}

inline std::u16string_view Utf16Wrap::borrow_cont() const {
  auto result = capi::Utf16Wrap_borrow_cont(this->AsFFI());
  return std::u16string_view(result.data, result.len);
}

inline std::u16string_view Utf16Wrap::owned() const {
  auto result = capi::Utf16Wrap_owned(this->AsFFI());
  return std::u16string_view(result.data, result.len);
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
