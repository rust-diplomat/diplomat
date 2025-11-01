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

} // namespace capi
} // namespace


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
