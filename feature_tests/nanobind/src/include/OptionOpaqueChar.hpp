#ifndef OptionOpaqueChar_HPP
#define OptionOpaqueChar_HPP

#include "OptionOpaqueChar.d.hpp"

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
    
    void OptionOpaqueChar_assert_char(const diplomat::capi::OptionOpaqueChar* self, char32_t ch);
    
    
    void OptionOpaqueChar_destroy(OptionOpaqueChar* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline void OptionOpaqueChar::assert_char(char32_t ch) const {
  diplomat::capi::OptionOpaqueChar_assert_char(this->AsFFI(),
    ch);
}

inline const diplomat::capi::OptionOpaqueChar* OptionOpaqueChar::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OptionOpaqueChar*>(this);
}

inline diplomat::capi::OptionOpaqueChar* OptionOpaqueChar::AsFFI() {
  return reinterpret_cast<diplomat::capi::OptionOpaqueChar*>(this);
}

inline const OptionOpaqueChar* OptionOpaqueChar::FromFFI(const diplomat::capi::OptionOpaqueChar* ptr) {
  return reinterpret_cast<const OptionOpaqueChar*>(ptr);
}

inline OptionOpaqueChar* OptionOpaqueChar::FromFFI(diplomat::capi::OptionOpaqueChar* ptr) {
  return reinterpret_cast<OptionOpaqueChar*>(ptr);
}

inline void OptionOpaqueChar::operator delete(void* ptr) {
  diplomat::capi::OptionOpaqueChar_destroy(reinterpret_cast<diplomat::capi::OptionOpaqueChar*>(ptr));
}


#endif // OptionOpaqueChar_HPP
