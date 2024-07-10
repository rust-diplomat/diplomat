#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP

#include "ICU4XFixedDecimal.d.hpp"

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
    
    icu4x::capi::ICU4XFixedDecimal* icu4x_ICU4XFixedDecimal_new_mv1(int32_t v);
    
    void icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(icu4x::capi::ICU4XFixedDecimal* self, int16_t power);
    
    typedef struct icu4x_ICU4XFixedDecimal_to_string_mv1_result { bool is_ok;} icu4x_ICU4XFixedDecimal_to_string_mv1_result;
    icu4x_ICU4XFixedDecimal_to_string_mv1_result icu4x_ICU4XFixedDecimal_to_string_mv1(const icu4x::capi::ICU4XFixedDecimal* self, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_ICU4XFixedDecimal_mv1_destroy(ICU4XFixedDecimal* self);
    
    } // extern "C"
}
}
inline std::unique_ptr<icu4x::ICU4XFixedDecimal> icu4x::ICU4XFixedDecimal::new_(int32_t v) {
  auto result = capi::icu4x_ICU4XFixedDecimal_new_mv1(v);
  return std::unique_ptr<icu4x::ICU4XFixedDecimal>(icu4x::ICU4XFixedDecimal::FromFFI(result));
}

inline void icu4x::ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  capi::icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(this->AsFFI(),
    power);
}

inline diplomat::result<std::string, std::monostate> icu4x::ICU4XFixedDecimal::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::icu4x_ICU4XFixedDecimal_to_string_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::monostate>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::monostate>(diplomat::Err<std::monostate>());
}

inline const icu4x::capi::ICU4XFixedDecimal* icu4x::ICU4XFixedDecimal::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ICU4XFixedDecimal*>(this);
}

inline icu4x::capi::ICU4XFixedDecimal* icu4x::ICU4XFixedDecimal::AsFFI() {
  return reinterpret_cast<icu4x::capi::ICU4XFixedDecimal*>(this);
}

inline const icu4x::ICU4XFixedDecimal* icu4x::ICU4XFixedDecimal::FromFFI(const icu4x::capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<const icu4x::ICU4XFixedDecimal*>(ptr);
}

inline icu4x::ICU4XFixedDecimal* icu4x::ICU4XFixedDecimal::FromFFI(icu4x::capi::ICU4XFixedDecimal* ptr) {
  return reinterpret_cast<icu4x::ICU4XFixedDecimal*>(ptr);
}

inline void icu4x::ICU4XFixedDecimal::operator delete(void* ptr) {
  capi::icu4x_ICU4XFixedDecimal_mv1_destroy(reinterpret_cast<icu4x::capi::ICU4XFixedDecimal*>(ptr));
}


#endif // ICU4XFixedDecimal_HPP
