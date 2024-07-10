#ifndef ICU4XFixedDecimalFormatter_HPP
#define ICU4XFixedDecimalFormatter_HPP

#include "ICU4XFixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XFixedDecimal.hpp"
#include "ICU4XFixedDecimalFormatterOptions.hpp"
#include "ICU4XLocale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XFixedDecimalFormatter_try_new_result {union {icu4x::capi::ICU4XFixedDecimalFormatter* ok; }; bool is_ok;} ICU4XFixedDecimalFormatter_try_new_result;
    ICU4XFixedDecimalFormatter_try_new_result ICU4XFixedDecimalFormatter_try_new(const icu4x::capi::ICU4XLocale* locale, const icu4x::capi::ICU4XDataProvider* provider, icu4x::capi::ICU4XFixedDecimalFormatterOptions options);
    
    void ICU4XFixedDecimalFormatter_format_write(const icu4x::capi::ICU4XFixedDecimalFormatter* self, const icu4x::capi::ICU4XFixedDecimal* value, ::capi::DiplomatWrite* write);
    
    
    void ICU4XFixedDecimalFormatter_destroy(ICU4XFixedDecimalFormatter* self);
    
    } // extern "C"
}
}
inline diplomat::result<std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>, std::monostate> icu4x::ICU4XFixedDecimalFormatter::try_new(const icu4x::ICU4XLocale& locale, const icu4x::ICU4XDataProvider& provider, icu4x::ICU4XFixedDecimalFormatterOptions options) {
  auto result = capi::ICU4XFixedDecimalFormatter_try_new(locale.AsFFI(),
    provider.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>, std::monostate>(diplomat::Ok<std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>>(std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>(icu4x::ICU4XFixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>, std::monostate>(diplomat::Err<std::monostate>());
}

inline std::string icu4x::ICU4XFixedDecimalFormatter::format_write(const icu4x::ICU4XFixedDecimal& value) const {
  std::string output;
  ::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XFixedDecimalFormatter_format_write(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::ICU4XFixedDecimalFormatter* icu4x::ICU4XFixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ICU4XFixedDecimalFormatter*>(this);
}

inline icu4x::capi::ICU4XFixedDecimalFormatter* icu4x::ICU4XFixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::ICU4XFixedDecimalFormatter*>(this);
}

inline const icu4x::ICU4XFixedDecimalFormatter* icu4x::ICU4XFixedDecimalFormatter::FromFFI(const icu4x::capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<const icu4x::ICU4XFixedDecimalFormatter*>(ptr);
}

inline icu4x::ICU4XFixedDecimalFormatter* icu4x::ICU4XFixedDecimalFormatter::FromFFI(icu4x::capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<icu4x::ICU4XFixedDecimalFormatter*>(ptr);
}

inline void icu4x::ICU4XFixedDecimalFormatter::operator delete(void* ptr) {
  capi::ICU4XFixedDecimalFormatter_destroy(reinterpret_cast<icu4x::capi::ICU4XFixedDecimalFormatter*>(ptr));
}


#endif // ICU4XFixedDecimalFormatter_HPP
