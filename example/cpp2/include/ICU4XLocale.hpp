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


namespace capi {
    extern "C" {
    
    ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);
    
    
    void ICU4XLocale_destroy(ICU4XLocale* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XLocale> ICU4XLocale::new_(std::string_view name) {
  auto result = capi::ICU4XLocale_new(name.data(),
    name.size());
  return std::unique_ptr<ICU4XLocale>(ICU4XLocale::FromFFI(result));
}

inline const capi::ICU4XLocale* ICU4XLocale::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocale*>(this);
}

inline capi::ICU4XLocale* ICU4XLocale::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocale*>(this);
}

inline const ICU4XLocale* ICU4XLocale::FromFFI(const capi::ICU4XLocale* ptr) {
  return reinterpret_cast<const ICU4XLocale*>(ptr);
}

inline ICU4XLocale* ICU4XLocale::FromFFI(capi::ICU4XLocale* ptr) {
  return reinterpret_cast<ICU4XLocale*>(ptr);
}

inline void ICU4XLocale::operator delete(void* ptr) {
  capi::ICU4XLocale_destroy(reinterpret_cast<capi::ICU4XLocale*>(ptr));
}


#endif // ICU4XLocale_HPP
