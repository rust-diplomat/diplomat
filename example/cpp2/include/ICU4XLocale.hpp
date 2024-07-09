#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP

#include "ICU4XLocale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);
    
    
    void ICU4XLocale_destroy(ICU4XLocale* self);
    
    } // extern "C"
}
}
inline std::unique_ptr<icu4x::ICU4XLocale> icu4x::ICU4XLocale::new_(std::string_view name) {
  auto result = capi::ICU4XLocale_new(name.data(),
    name.size());
  return std::unique_ptr<icu4x::ICU4XLocale>(icu4x::ICU4XLocale::FromFFI(result));
}

inline const icu4x::capi::ICU4XLocale* icu4x::ICU4XLocale::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ICU4XLocale*>(this);
}

inline icu4x::capi::ICU4XLocale* icu4x::ICU4XLocale::AsFFI() {
  return reinterpret_cast<icu4x::capi::ICU4XLocale*>(this);
}

inline const icu4x::ICU4XLocale* icu4x::ICU4XLocale::FromFFI(const icu4x::capi::ICU4XLocale* ptr) {
  return reinterpret_cast<const icu4x::ICU4XLocale*>(ptr);
}

inline icu4x::ICU4XLocale* icu4x::ICU4XLocale::FromFFI(icu4x::capi::ICU4XLocale* ptr) {
  return reinterpret_cast<icu4x::ICU4XLocale*>(ptr);
}

inline void icu4x::ICU4XLocale::operator delete(void* ptr) {
  capi::ICU4XLocale_destroy(reinterpret_cast<icu4x::capi::ICU4XLocale*>(ptr));
}


#endif // ICU4XLocale_HPP
