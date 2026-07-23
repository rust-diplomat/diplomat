#ifndef ICU4X_DataProvider_HPP
#define ICU4X_DataProvider_HPP

#include "DataProvider.d.hpp"

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

    icu4x::capi::DataProvider* icu4x_DataProvider_new_static_mv1(void);

    typedef struct icu4x_DataProvider_returns_result_mv1_result { bool is_ok;} icu4x_DataProvider_returns_result_mv1_result;
    icu4x_DataProvider_returns_result_mv1_result icu4x_DataProvider_returns_result_mv1(void);

    void icu4x_DataProvider_destroy_mv1(DataProvider* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::DataProvider icu4x::DataProvider::new_static() {
    auto result = icu4x::capi::icu4x_DataProvider_new_static_mv1();
    return icu4x::DataProvider::FromFFI(result);
}

inline icu4x::diplomat::result<std::monostate, std::monostate> icu4x::DataProvider::returns_result() {
    auto result = icu4x::capi::icu4x_DataProvider_returns_result_mv1();
    return result.is_ok ? icu4x::diplomat::result<std::monostate, std::monostate>(icu4x::diplomat::Ok<std::monostate>()) : icu4x::diplomat::result<std::monostate, std::monostate>(icu4x::diplomat::Err<std::monostate>());
}


#endif // ICU4X_DataProvider_HPP
