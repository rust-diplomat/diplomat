#ifndef icu4x_FixedDecimal_HPP
#define icu4x_FixedDecimal_HPP

#include "FixedDecimal.d.hpp"

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
    
    icu4x::capi::FixedDecimal* icu4x_FixedDecimal_new_mv1(int32_t v);
    
    void icu4x_FixedDecimal_multiply_pow10_mv1(icu4x::capi::FixedDecimal* self, int16_t power);
    
    typedef struct icu4x_FixedDecimal_to_string_mv1_result { bool is_ok;} icu4x_FixedDecimal_to_string_mv1_result;
    icu4x_FixedDecimal_to_string_mv1_result icu4x_FixedDecimal_to_string_mv1(const icu4x::capi::FixedDecimal* self, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_FixedDecimal_destroy_mv1(FixedDecimal* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::FixedDecimal> icu4x::FixedDecimal::new_(int32_t v) {
  auto result = icu4x::capi::icu4x_FixedDecimal_new_mv1(v);
  return std::unique_ptr<icu4x::FixedDecimal>(icu4x::FixedDecimal::FromFFI(result));
}

inline void icu4x::FixedDecimal::multiply_pow10(int16_t power) {
  icu4x::capi::icu4x_FixedDecimal_multiply_pow10_mv1(this->AsFFI(),
    power);
}

inline diplomat::result<std::string, std::monostate> icu4x::FixedDecimal::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_FixedDecimal_to_string_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::monostate>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::monostate>(diplomat::Err<std::monostate>());
}

inline const icu4x::capi::FixedDecimal* icu4x::FixedDecimal::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::FixedDecimal*>(this);
}

inline icu4x::capi::FixedDecimal* icu4x::FixedDecimal::AsFFI() {
  return reinterpret_cast<icu4x::capi::FixedDecimal*>(this);
}

inline const icu4x::FixedDecimal* icu4x::FixedDecimal::FromFFI(const icu4x::capi::FixedDecimal* ptr) {
  return reinterpret_cast<const icu4x::FixedDecimal*>(ptr);
}

inline icu4x::FixedDecimal* icu4x::FixedDecimal::FromFFI(icu4x::capi::FixedDecimal* ptr) {
  return reinterpret_cast<icu4x::FixedDecimal*>(ptr);
}

inline void icu4x::FixedDecimal::operator delete(void* ptr) {
  icu4x::capi::icu4x_FixedDecimal_destroy_mv1(reinterpret_cast<icu4x::capi::FixedDecimal*>(ptr));
}


#endif // icu4x_FixedDecimal_HPP
