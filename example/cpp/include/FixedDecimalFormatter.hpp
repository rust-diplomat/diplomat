#ifndef ICU4X_FixedDecimalFormatter_HPP
#define ICU4X_FixedDecimalFormatter_HPP

#include "FixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataProvider.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalFormatterOptions.hpp"
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_FixedDecimalFormatter_try_new_mv1_result {union {icu4x::capi::FixedDecimalFormatter* ok; }; bool is_ok;} icu4x_FixedDecimalFormatter_try_new_mv1_result;
    icu4x_FixedDecimalFormatter_try_new_mv1_result icu4x_FixedDecimalFormatter_try_new_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DataProvider* provider, icu4x::capi::FixedDecimalFormatterOptions options);

    void icu4x_FixedDecimalFormatter_format_write_mv1(const icu4x::capi::FixedDecimalFormatter* self, const icu4x::capi::FixedDecimal* value, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<icu4x::FixedDecimalFormatter, std::monostate> icu4x::FixedDecimalFormatter::try_new(const icu4x::Locale& locale, const icu4x::DataProvider& provider, icu4x::FixedDecimalFormatterOptions options) {
    auto result = icu4x::capi::icu4x_FixedDecimalFormatter_try_new_mv1(locale.AsFFI(),
        provider.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<icu4x::FixedDecimalFormatter, std::monostate>(icu4x::diplomat::Ok<icu4x::FixedDecimalFormatter>(icu4x::FixedDecimalFormatter::FromFFI(result.ok))) : icu4x::diplomat::result<icu4x::FixedDecimalFormatter, std::monostate>(icu4x::diplomat::Err<std::monostate>());
}

inline std::string icu4x::FixedDecimalFormatter::format_write(const icu4x::FixedDecimal& value) const {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    icu4x::capi::icu4x_FixedDecimalFormatter_format_write_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::FixedDecimalFormatter::format_write_write(const icu4x::FixedDecimal& value, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_FixedDecimalFormatter_format_write_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
}


#endif // ICU4X_FixedDecimalFormatter_HPP
