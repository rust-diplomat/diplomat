#ifndef icu4x_Locale_HPP
#define icu4x_Locale_HPP

#include "Locale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::Locale* icu4x_Locale_new_mv1(diplomat::capi::DiplomatStringView name);
    
    
    void icu4x_Locale_destroy_mv1(Locale* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::Locale> icu4x::Locale::new_(std::string_view name) {
  auto result = icu4x::capi::icu4x_Locale_new_mv1({name.data(), name.size()});
  return std::unique_ptr<icu4x::Locale>(icu4x::Locale::FromFFI(result));
}

inline const icu4x::capi::Locale* icu4x::Locale::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::Locale*>(this);
}

inline icu4x::capi::Locale* icu4x::Locale::AsFFI() {
  return reinterpret_cast<icu4x::capi::Locale*>(this);
}

inline const icu4x::Locale* icu4x::Locale::FromFFI(const icu4x::capi::Locale* ptr) {
  return reinterpret_cast<const icu4x::Locale*>(ptr);
}

inline icu4x::Locale* icu4x::Locale::FromFFI(icu4x::capi::Locale* ptr) {
  return reinterpret_cast<icu4x::Locale*>(ptr);
}

inline void icu4x::Locale::operator delete(void* ptr) {
  icu4x::capi::icu4x_Locale_destroy_mv1(reinterpret_cast<icu4x::capi::Locale*>(ptr));
}


#endif // icu4x_Locale_HPP
