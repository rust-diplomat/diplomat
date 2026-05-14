#ifndef SOMELIB_ErrorStruct_HPP
#define SOMELIB_ErrorStruct_HPP

#include "ErrorStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    typedef struct ErrorStruct_returns_result_option_result {union {somelib::capi::ErrorStruct_option ok; }; bool is_ok;} ErrorStruct_returns_result_option_result;
    ErrorStruct_returns_result_option_result ErrorStruct_returns_result_option(bool is_some);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<std::optional<somelib::ErrorStruct>, std::monostate> somelib::ErrorStruct::returns_result_option(bool is_some) {
    auto result = somelib::capi::ErrorStruct_returns_result_option(is_some);
    return result.is_ok ? somelib::diplomat::result<std::optional<somelib::ErrorStruct>, std::monostate>(somelib::diplomat::Ok<std::optional<somelib::ErrorStruct>>(result.ok.is_ok ? std::optional(somelib::ErrorStruct::FromFFI(result.ok.ok)) : std::nullopt)) : somelib::diplomat::result<std::optional<somelib::ErrorStruct>, std::monostate>(somelib::diplomat::Err<std::monostate>());
}


inline somelib::capi::ErrorStruct somelib::ErrorStruct::AsFFI() const {
    return somelib::capi::ErrorStruct {
        /* .i = */ i,
        /* .j = */ j,
    };
}

inline somelib::ErrorStruct somelib::ErrorStruct::FromFFI(somelib::capi::ErrorStruct c_struct) {
    return somelib::ErrorStruct {
        /* .i = */ c_struct.i,
        /* .j = */ c_struct.j,
    };
}


#endif // SOMELIB_ErrorStruct_HPP
