#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP

#include "ICU4XDataProvider.d.hpp"

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
    
    icu4x::capi::ICU4XDataProvider* ICU4XDataProvider_new_static();
    
    typedef struct ICU4XDataProvider_returns_result_result { bool is_ok;} ICU4XDataProvider_returns_result_result;
    ICU4XDataProvider_returns_result_result ICU4XDataProvider_returns_result();
    
    
    void ICU4XDataProvider_destroy(ICU4XDataProvider* self);
    
    } // extern "C"
}
}
inline std::unique_ptr<icu4x::ICU4XDataProvider> icu4x::ICU4XDataProvider::new_static() {
  auto result = capi::ICU4XDataProvider_new_static();
  return std::unique_ptr<icu4x::ICU4XDataProvider>(icu4x::ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, std::monostate> icu4x::ICU4XDataProvider::returns_result() {
  auto result = capi::ICU4XDataProvider_returns_result();
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline const icu4x::capi::ICU4XDataProvider* icu4x::ICU4XDataProvider::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ICU4XDataProvider*>(this);
}

inline icu4x::capi::ICU4XDataProvider* icu4x::ICU4XDataProvider::AsFFI() {
  return reinterpret_cast<icu4x::capi::ICU4XDataProvider*>(this);
}

inline const icu4x::ICU4XDataProvider* icu4x::ICU4XDataProvider::FromFFI(const icu4x::capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<const icu4x::ICU4XDataProvider*>(ptr);
}

inline icu4x::ICU4XDataProvider* icu4x::ICU4XDataProvider::FromFFI(icu4x::capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<icu4x::ICU4XDataProvider*>(ptr);
}

inline void icu4x::ICU4XDataProvider::operator delete(void* ptr) {
  capi::ICU4XDataProvider_destroy(reinterpret_cast<icu4x::capi::ICU4XDataProvider*>(ptr));
}


#endif // ICU4XDataProvider_HPP
