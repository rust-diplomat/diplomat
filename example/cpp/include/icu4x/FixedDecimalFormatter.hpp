#ifndef icu4x_FixedDecimalFormatter_HPP
#define icu4x_FixedDecimalFormatter_HPP

#include "FixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalFormatterOptions.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_FixedDecimalFormatter_try_new_mv1_result {union {icu4x::capi::FixedDecimalFormatter* ok; }; bool is_ok;} icu4x_FixedDecimalFormatter_try_new_mv1_result;
    icu4x_FixedDecimalFormatter_try_new_mv1_result icu4x_FixedDecimalFormatter_try_new_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DataProvider* provider, icu4x::capi::FixedDecimalFormatterOptions options);
    
    void icu4x_FixedDecimalFormatter_format_write_mv1(const icu4x::capi::FixedDecimalFormatter* self, const icu4x::capi::FixedDecimal* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::FixedDecimalFormatter>, std::monostate> icu4x::FixedDecimalFormatter::try_new(const icu4x::Locale& locale, const icu4x::DataProvider& provider, icu4x::FixedDecimalFormatterOptions options) {
  auto result = icu4x::capi::icu4x_FixedDecimalFormatter_try_new_mv1(locale.AsFFI(),
    provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::FixedDecimalFormatter>, std::monostate>(diplomat::Ok<std::unique_ptr<icu4x::FixedDecimalFormatter>>(std::unique_ptr<icu4x::FixedDecimalFormatter>(icu4x::FixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::FixedDecimalFormatter>, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string icu4x::FixedDecimalFormatter::format_write(const icu4x::FixedDecimal& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_FixedDecimalFormatter_format_write_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::FixedDecimalFormatter* icu4x::FixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::FixedDecimalFormatter*>(this);
}

inline icu4x::capi::FixedDecimalFormatter* icu4x::FixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::FixedDecimalFormatter*>(this);
}

inline const icu4x::FixedDecimalFormatter* icu4x::FixedDecimalFormatter::FromFFI(const icu4x::capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<const icu4x::FixedDecimalFormatter*>(ptr);
}

inline icu4x::FixedDecimalFormatter* icu4x::FixedDecimalFormatter::FromFFI(icu4x::capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<icu4x::FixedDecimalFormatter*>(ptr);
}

inline void icu4x::FixedDecimalFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_FixedDecimalFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::FixedDecimalFormatter*>(ptr));
}


#endif // icu4x_FixedDecimalFormatter_HPP
