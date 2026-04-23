#ifndef SOMELIB_FallibleOpaqueConstructor_HPP
#define SOMELIB_FallibleOpaqueConstructor_HPP

#include "FallibleOpaqueConstructor.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ResultOpaque.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    typedef struct FallibleOpaqueConstructor_ctor_result {union {somelib::capi::FallibleOpaqueConstructor ok; somelib::capi::ResultOpaque* err;}; bool is_ok;} FallibleOpaqueConstructor_ctor_result;
    FallibleOpaqueConstructor_ctor_result FallibleOpaqueConstructor_ctor(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::FallibleOpaqueConstructor, std::unique_ptr<somelib::ResultOpaque>> somelib::FallibleOpaqueConstructor::ctor() {
    auto result = somelib::capi::FallibleOpaqueConstructor_ctor();
    return result.is_ok ? somelib::diplomat::result<somelib::FallibleOpaqueConstructor, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Ok<somelib::FallibleOpaqueConstructor>(somelib::FallibleOpaqueConstructor::FromFFI(result.ok))) : somelib::diplomat::result<somelib::FallibleOpaqueConstructor, std::unique_ptr<somelib::ResultOpaque>>(somelib::diplomat::Err<std::unique_ptr<somelib::ResultOpaque>>(std::unique_ptr<somelib::ResultOpaque>(somelib::ResultOpaque::FromFFI(result.err))));
}


inline somelib::capi::FallibleOpaqueConstructor somelib::FallibleOpaqueConstructor::AsFFI() const {
    return somelib::capi::FallibleOpaqueConstructor {
        /* .x = */ x,
    };
}

inline somelib::FallibleOpaqueConstructor somelib::FallibleOpaqueConstructor::FromFFI(somelib::capi::FallibleOpaqueConstructor c_struct) {
    return somelib::FallibleOpaqueConstructor {
        /* .x = */ c_struct.x,
    };
}


#endif // SOMELIB_FallibleOpaqueConstructor_HPP
