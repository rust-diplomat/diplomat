#ifndef SOMELIB_mylib_MethodOverloading_HPP
#define SOMELIB_mylib_MethodOverloading_HPP

#include "MethodOverloading.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::mylib {
namespace capi {
    extern "C" {

    somelib::mylib::capi::MethodOverloading* MethodOverloading_from_int32(int32_t _v);

    somelib::mylib::capi::MethodOverloading* MethodOverloading_from_int64(int64_t _v);

    somelib::mylib::capi::MethodOverloading* MethodOverloading_from_uint32(uint32_t _v);

    void MethodOverloading_destroy(MethodOverloading* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::mylib::MethodOverloading somelib::mylib::MethodOverloading::from(int32_t _v) {
    auto result = somelib::mylib::capi::MethodOverloading_from_int32(_v);
    return somelib::mylib::MethodOverloading::FromFFI(result);
}

inline somelib::mylib::MethodOverloading somelib::mylib::MethodOverloading::from(int64_t _v) {
    auto result = somelib::mylib::capi::MethodOverloading_from_int64(_v);
    return somelib::mylib::MethodOverloading::FromFFI(result);
}

inline somelib::mylib::MethodOverloading somelib::mylib::MethodOverloading::from(uint32_t _v) {
    auto result = somelib::mylib::capi::MethodOverloading_from_uint32(_v);
    return somelib::mylib::MethodOverloading::FromFFI(result);
}


#endif // SOMELIB_mylib_MethodOverloading_HPP
