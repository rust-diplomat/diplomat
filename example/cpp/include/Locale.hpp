#ifndef ICU4X_Locale_HPP
#define ICU4X_Locale_HPP

#include "Locale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::Locale* icu4x_Locale_new_mv1(icu4x::diplomat::capi::DiplomatStringView name);

    void icu4x_Locale_destroy_mv1(Locale* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::Locale icu4x::Locale::new_(std::string_view name) {
    auto result = icu4x::capi::icu4x_Locale_new_mv1({name.data(), name.size()});
    return icu4x::Locale::FromFFI(result);
}


#endif // ICU4X_Locale_HPP
