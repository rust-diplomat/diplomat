#ifndef icu4x_DataProvider_HPP
#define icu4x_DataProvider_HPP

#include "DataProvider.d.hpp"

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
    
    icu4x::capi::DataProvider* icu4x_DataProvider_new_static_mv1(void);
    
    typedef struct icu4x_DataProvider_returns_result_mv1_result { bool is_ok;} icu4x_DataProvider_returns_result_mv1_result;
    icu4x_DataProvider_returns_result_mv1_result icu4x_DataProvider_returns_result_mv1(void);
    
    
    void icu4x_DataProvider_destroy_mv1(DataProvider* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::DataProvider> icu4x::DataProvider::new_static() {
  auto result = icu4x::capi::icu4x_DataProvider_new_static_mv1();
  return std::unique_ptr<icu4x::DataProvider>(icu4x::DataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, std::monostate> icu4x::DataProvider::returns_result() {
  auto result = icu4x::capi::icu4x_DataProvider_returns_result_mv1();
  return result.is_ok ? diplomat::result<std::monostate, std::monostate>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::monostate>(diplomat::Err<std::monostate>());
}

inline const icu4x::capi::DataProvider* icu4x::DataProvider::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::DataProvider*>(this);
}

inline icu4x::capi::DataProvider* icu4x::DataProvider::AsFFI() {
  return reinterpret_cast<icu4x::capi::DataProvider*>(this);
}

inline const icu4x::DataProvider* icu4x::DataProvider::FromFFI(const icu4x::capi::DataProvider* ptr) {
  return reinterpret_cast<const icu4x::DataProvider*>(ptr);
}

inline icu4x::DataProvider* icu4x::DataProvider::FromFFI(icu4x::capi::DataProvider* ptr) {
  return reinterpret_cast<icu4x::DataProvider*>(ptr);
}

inline void icu4x::DataProvider::operator delete(void* ptr) {
  icu4x::capi::icu4x_DataProvider_destroy_mv1(reinterpret_cast<icu4x::capi::DataProvider*>(ptr));
}


#endif // icu4x_DataProvider_HPP
