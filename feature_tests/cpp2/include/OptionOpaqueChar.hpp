#ifndef OptionOpaqueChar_HPP
#define OptionOpaqueChar_HPP

#include "OptionOpaqueChar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    void OptionOpaqueChar_assert_char(const OptionOpaqueChar* self, char32_t ch);
    
    
    void OptionOpaqueChar_destroy(OptionOpaqueChar* self);
    
    } // extern "C"
}

inline void OptionOpaqueChar::assert_char(char32_t ch) const {
  capi::OptionOpaqueChar_assert_char(this->AsFFI(),
    ch);
}

inline const capi::OptionOpaqueChar* OptionOpaqueChar::AsFFI() const {
  return reinterpret_cast<const capi::OptionOpaqueChar*>(this);
}

inline capi::OptionOpaqueChar* OptionOpaqueChar::AsFFI() {
  return reinterpret_cast<capi::OptionOpaqueChar*>(this);
}

inline const OptionOpaqueChar* OptionOpaqueChar::FromFFI(const capi::OptionOpaqueChar* ptr) {
  return reinterpret_cast<const OptionOpaqueChar*>(ptr);
}

inline OptionOpaqueChar* OptionOpaqueChar::FromFFI(capi::OptionOpaqueChar* ptr) {
  return reinterpret_cast<OptionOpaqueChar*>(ptr);
}

inline void OptionOpaqueChar::operator delete(void* ptr) {
  capi::OptionOpaqueChar_destroy(reinterpret_cast<capi::OptionOpaqueChar*>(ptr));
}


#endif // OptionOpaqueChar_HPP
